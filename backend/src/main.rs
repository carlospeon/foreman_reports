use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use axum_login::AuthManagerLayerBuilder;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use clap::Parser;
use http::Method;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{
    collections::HashMap,
    net::SocketAddr,
    str::FromStr,
    sync::Arc,
    time::Duration
};
use tokio::sync::RwLock;
use tower::limit::concurrency::ConcurrencyLimitLayer;
use tower_http::{
    cors::{CorsLayer, Any},
    services::{ServeDir, ServeFile},
    trace::{self, TraceLayer},
};
use tracing::Level;
use url::Url;

pub mod response;
pub mod authentication;
pub mod configuration;
pub mod reports;
#[allow(dead_code)]
pub mod api_doc;

use crate::configuration::Configuration;
use crate::authentication::User;
use crate::reports::*;
use crate::reports::hosts::*;
use crate::reports::updated::*;
use crate::reports::report::*;
use crate::reports::erratas::*;


/// Foreman Reports
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Configuration file. Default: 'configuration.toml'
    #[arg(short, long)]
    config: Option<String>,
    /// Debug
    #[arg(short, long, action)]
    debug: bool,
}

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
    configuration: Configuration,
    user_store: Arc<RwLock<HashMap<String, User>>>
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config_file = match args.config {
        Some(c) => c,
        None => "configuration.toml".to_string()
    };

    let configuration = match Configuration::new(&config_file) {
        Err(error) => panic!("error loading configuration: {:?}", error),
        Ok(cfg) => cfg
    };

    let loglevel = match args.debug {
        false => match Level::from_str(&configuration.server.loglevel) {
            Err(error) => panic!("invalid loglevel value: {:?}", error),
            Ok(level) => level
        }
        true => Level::DEBUG
    };

    tracing_subscriber::fmt()
        .with_max_level(loglevel)
        .with_target(false)
        .compact()
        .init();
    tracing::info!("tracing log level: {:?}", loglevel.to_string());


    if loglevel == Level::DEBUG {
        let mut masked_configuration = configuration.clone();
        let mut url = match Url::parse(&masked_configuration.database.url) {
            Err(error) => panic!("unable to parse database url: {:?}", error),
            Ok(u) => u
        };
        url.set_password(Some("******")).unwrap();
        masked_configuration.database.url = url.into();
        tracing::debug!("configuration: {:?}", masked_configuration);
    }

    let server: SocketAddr = match configuration.server.listen.parse() {
        Err(e) => panic!("unable to parse server listen: {} {:?}", configuration.server.listen, e),
        Ok(s) => s
    };
    let concurency_limit = configuration.server.concurency_limit;

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    let user_store: Arc<RwLock<HashMap<String, User>>> = Arc::new(RwLock::new(HashMap::default()));

    let backend = authentication::Backend::new(
        configuration.ldap.clone(),
        user_store.clone(),
    );
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(configuration.database.max_connections)
        .acquire_timeout(Duration::from_secs(configuration.database.acquire_timeout))
        .connect_lazy(&configuration.database.url)
        .expect("can't connect to database");

    if loglevel == Level::DEBUG {
        let mut connect_options = (*pool.connect_options()).clone();
        connect_options = connect_options.password("********");
        tracing::debug!("PgPool connect_options: {:?}", connect_options);
    }
    tracing::debug!("PgPool options: {:?}", pool.options());

    let state = AppState{ configuration: configuration,
                          pool: pool,
                          user_store: user_store };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let static_service = ServeDir::new("static").not_found_service(ServeFile::new("static/index.html"));

    let test_routes = Router::new()
        .route("/test", get(reports::get_test::<Test>));

    let host_routes = Router::new()
        .route("/info/{key}", get(reports::get_with_path_param::<HostInfo>))
        .route("/groupby/osmajor", get(reports::get::<HostsGroupbyOsmajor>))
        .route("/groupby/osmajor/history", get(reports::get::<HostsGroupbyOSmajorHistory>))
        .route("/groupby/environment", get(reports::get::<HostsGroupbyEnvironment>))
        .route("/groupby/location", get(reports::get::<HostsGroupbyLocation>))
        .route("/groupby/admingroup", get(reports::get::<HostsGroupbyAdminGroup>))
        .route("/groupby/bu", get(reports::get::<HostsGroupbyBU>))
        .route("/groupby/hardware", get(reports::get::<HostsGroupbyHardware>))
        .route("/groupby/hardware/history", get(reports::get::<HostsGroupbyHardwareHistory>))
        .route("/groupby/domain", get(reports::get::<HostsGroupbyDomain>))
        .route("/groupby/domain/history", get(reports::get::<HostsGroupbyDomainHistory>))
        .route("/groupby/location/environment", get(reports::get::<HostsGroupbyLocationEnvironment>))
        .route("/resources/groupby/location", get(reports::get::<OSResourcesByLocation>))
        .route("/erratas", get(reports::get::<ErrataCompliant>))
        .route("/erratas/noncompliant/{days}", get(reports::get_noncompliant_erratas::<ErrataCompliant>))
        .route("/erratas/groupby/bu", get(reports::get::<ErrataCompliantGroupbyBu>))
        .route("/erratas/groupby/bu/noncompliant/{days}",
            get(reports::get_noncompliant_erratas::<ErrataCompliantGroupbyBu>))
        .route("/erratas/groupby/environment/bu", get(reports::get::<ErrataCompliantGroupbyEnvironmentBu>))
        .route("/erratas/groupby/environment/bu/noncompliant/{days}",
            get(reports::get_noncompliant_erratas::<ErrataCompliantGroupbyEnvironmentBu>))
        .route("/facts", get(reports::get::<VisibleFacts>))
        .route("/facts/{key}", get(reports::get_with_path_param::<Fact>))
        .route("/groupby/fact/{key}", get(reports::get_with_path_filter::<HostsGroupbyFact>))
        .route("/groupby/fact/{key}/{value}", get(reports::get_with_path_filter::<HostsGroupbyFact>));

    let updated_routes = Router::new()
        .route("/", get(reports::get::<Updated>))
        .route("/compliant", get(reports::get::<UpdatedCompliant>))
        .route("/groupby/environment", get(reports::get::<UpdatedGroupByEnvironment>))
        .route("/groupby/environment/compliant", get(reports::get::<UpdatedGroupByEnvironmentCompliant>))
        .route("/groupby/location", get(reports::get::<UpdatedGroupByLocation>))
        .route("/groupby/location/compliant", get(reports::get::<UpdatedGroupByLocationCompliant>));

    let nonupdated_routes = Router::new()
        .route("/adminenvironment", get(reports::get::<NonUpdatedGroupByAdminEnvironment>))
        .route("/adminenvironment/compliant", get(reports::get::<NonUpdatedGroupByAdminEnvironmentCompliant>));

    let report_routes = Router::new()
        .route("/os", get(reports::get::<OSReport>))
        .route("/facts", get(reports::get::<HostFactsReport>))
        .route("/facts/{key}", get(reports::get_with_path_filter::<FactsReport>))
        .route("/facts/{key}/{value}", get(reports::get_with_path_filter::<FactsReport>))
        .route("/facts/{key}/cpu/groupby/environment", get(reports::get_with_path_filter::<CPUFactsReportGroupByEnvironment>))
        .route("/facts/{key}/cpu/history", get(reports::get::<CPUFactsHistory>))
        .route("/facts/{key}/groupby/environment", get(reports::get_with_path_filter::<FactsReportGroupByEnvironment>))
        .route("/facts/{key}/{value}/groupby/environment", get(reports::get_with_path_filter::<FactsReportGroupByEnvironment>));

    let errata_routes = Router::new()
        .route("/", get(reports::get::<Erratas>));

    let content_views_routes = Router::new()
        .route("/", get(reports::get::<ContentViews>));

    let os_routes = Router::new()
        .route("/eol", get(reports::get::<OSEOL>));

    let openscap_routes = Router::new()
        .route("/", get(reports::get::<OpenscapReport>))
        .route("/host/{key}", get(reports::get_with_path_param::<OpenscapHostReport>))
        .route("/rule/{key}", get(reports::get_with_path_filter::<OpenscapRuleReport>));

    let app = Router::new()
        .nest("/api", test_routes)
        .nest("/api/hosts", host_routes)
        .nest("/api/updated", updated_routes)
        .nest("/api/nonupdated/groupby/", nonupdated_routes)
        .nest("/api/os", os_routes)
        .nest("/api/report", report_routes)
        .nest("/api/erratas", errata_routes)
        .nest("/api/contentviews", content_views_routes)
        .nest("/api/openscap", openscap_routes)
        .route_layer(middleware::from_fn_with_state(state.clone(), authentication::basic_auth))
        .route("/api/login", post(authentication::post_login))
        .merge(SwaggerUi::new("/api/doc")
            .url("/api/doc/openapi.json", api_doc::ApiDoc::openapi()))
        .with_state(state)
        .layer(TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new()
                .level(loglevel))
            .on_response(trace::DefaultOnResponse::new()
                .level(loglevel)),)
        .layer(ConcurrencyLimitLayer::new(concurency_limit))
        .layer(auth_layer)
        .layer(cors)
        .fallback_service(static_service);

    // run it
    let listener = tokio::net::TcpListener::bind(server).await.unwrap();
    tracing::info!("starting server on {}", server);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
