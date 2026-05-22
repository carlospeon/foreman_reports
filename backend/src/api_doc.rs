use utoipa::OpenApi;

use crate::reports::hosts::*;
use crate::reports::updated::*;
use crate::reports::report::*;
use crate::reports::erratas::*;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Foreman Reports API",
        version = "0.1.0",
        description = "API de reporting para Foreman. Proporciona información sobre hosts, actualizaciones, erratas, openscap y más."
    ),
    paths(
        get_test,
        get_host_info,
        get_hosts_groupby_osmajor,
        get_hosts_groupby_osmajor_history,
        get_hosts_groupby_environment,
        get_hosts_groupby_location,
        get_hosts_groupby_admingroup,
        get_hosts_groupby_bu,
        get_hosts_groupby_hardware,
        get_hosts_groupby_hardware_history,
        get_hosts_groupby_domain,
        get_hosts_groupby_domain_history,
        get_hosts_groupby_location_environment,
        get_hosts_resources_groupby_location,
        get_hosts_erratas,
        get_hosts_erratas_noncompliant,
        get_hosts_erratas_groupby_bu,
        get_hosts_erratas_groupby_bu_noncompliant,
        get_hosts_erratas_groupby_environment_bu,
        get_hosts_erratas_groupby_environment_bu_noncompliant,
        get_hosts_facts,
        get_hosts_fact,
        get_hosts_groupby_fact,
        get_hosts_groupby_fact_value,
        get_updated,
        get_updated_compliant,
        get_updated_groupby_environment,
        get_updated_groupby_environment_compliant,
        get_updated_groupby_location,
        get_updated_groupby_location_compliant,
        get_nonupdated_adminenvironment,
        get_nonupdated_adminenvironment_compliant,
        get_report_os,
        get_report_facts,
        get_report_facts_key,
        get_report_facts_key_value,
        get_report_facts_key_cpu_groupby_environment,
        get_report_facts_key_cpu_history,
        get_report_facts_key_groupby_environment,
        get_report_facts_key_value_groupby_environment,
        get_erratas,
        get_contentviews,
        get_os_eol,
        get_openscap,
        get_openscap_host,
        get_openscap_rule,
        post_login,
    ),
    components(schemas(
        HostInfo,
        HostsGroupbyOsmajor,
        HostsGroupbyOSmajorHistory,
        HostsGroupbyEnvironment,
        HostsGroupbyLocation,
        HostsGroupbyLocationEnvironment,
        HostsGroupbyAdminGroup,
        HostsGroupbyBU,
        HostsGroupbyHardware,
        HostsGroupbyHardwareHistory,
        HostsGroupbyDomain,
        HostsGroupbyDomainHistory,
        VisibleFacts,
        Fact,
        Updated,
        UpdatedCompliant,
        UpdatedGroupByEnvironment,
        UpdatedGroupByEnvironmentCompliant,
        UpdatedGroupByLocation,
        UpdatedGroupByLocationCompliant,
        NonUpdatedGroupByAdminEnvironment,
        NonUpdatedGroupByAdminEnvironmentCompliant,
        ErrataCompliant,
        ErrataCompliantGroupbyBu,
        ErrataCompliantGroupbyEnvironmentBu,
        ContentViews,
        OSReport,
        OSResourcesByLocation,
        FactsReport,
        FactsReportGroupByEnvironment,
        CPUFactsReportGroupByEnvironment,
        HostsGroupbyFact,
        OSEOL,
        HostFactsReport,
        CPUFactsHistory,
        OpenscapReport,
        OpenscapHostReport,
        OpenscapRuleReport,
        Erratas,
    )),
    tags(
        (name = "test", description = "Test de conectividad"),
        (name = "hosts", description = "Información y agrupaciones de hosts"),
        (name = "updated", description = "Estado de actualización de hosts"),
        (name = "nonupdated", description = "Hosts no actualizados"),
        (name = "report", description = "Reportes detallados"),
        (name = "erratas", description = "Catálogo de erratas"),
        (name = "contentviews", description = "Content Views de Foreman"),
        (name = "os", description = "Información de sistemas operativos"),
        (name = "openscap", description = "Reportes de OpenSCAP"),
        (name = "auth", description = "Autenticación"),
    )
)]
pub struct ApiDoc;

#[utoipa::path(
    get, path = "/api/test",
    tag = "test",
    responses((status = 200, description = "Test de conectividad con la base de datos", body = String))
)]
async fn get_test() {}

#[utoipa::path(
    get, path = "/api/hosts/info/{key}",
    tag = "hosts",
    params(("key" = String, Path, description = "Hostname del servidor")),
    responses((status = 200, description = "Información detallada de un host", body = Vec<HostInfo>))
)]
async fn get_host_info() {}

#[utoipa::path(
    get, path = "/api/hosts/groupby/osmajor",
    tag = "hosts",
    responses((status = 200, description = "Hosts agrupados por versión mayor de SO", body = Vec<HostsGroupbyOsmajor>))
)]
async fn get_hosts_groupby_osmajor() {}

#[utoipa::path(
    get, path = "/api/hosts/groupby/osmajor/history",
    tag = "hosts",
    responses((status = 200, description = "Historial de hosts agrupados por versión mayor de SO", body = Vec<HostsGroupbyOSmajorHistory>))
)]
async fn get_hosts_groupby_osmajor_history() {}

#[utoipa::path(
    get, path = "/api/hosts/groupby/environment",
    tag = "hosts",
    responses((status = 200, description = "Hosts agrupados por entorno", body = Vec<HostsGroupbyEnvironment>))
)]
async fn get_hosts_groupby_environment() {}

#[utoipa::path(
    get, path = "/api/hosts/groupby/location",
    tag = "hosts",
    responses((status = 200, description = "Hosts agrupados por localización", body = Vec<HostsGroupbyLocation>))
)]
async fn get_hosts_groupby_location() {}

#[utoipa::path(
    get, path = "/api/hosts/groupby/admingroup",
    tag = "hosts",
    responses((status = 200, description = "Hosts agrupados por grupo de administración", body = Vec<HostsGroupbyAdminGroup>))
)]
async fn get_hosts_groupby_admingroup() {}

#[utoipa::path(
    get, path = "/api/hosts/groupby/bu",
    tag = "hosts",
    responses((status = 200, description = "Hosts agrupados por Business Unit", body = Vec<HostsGroupbyBU>))
)]
async fn get_hosts_groupby_bu() {}

#[utoipa::path(
    get, path = "/api/hosts/groupby/hardware",
    tag = "hosts",
    responses((status = 200, description = "Hosts agrupados por modelo de hardware", body = Vec<HostsGroupbyHardware>))
)]
async fn get_hosts_groupby_hardware() {}

#[utoipa::path(
    get, path = "/api/hosts/groupby/hardware/history",
    tag = "hosts",
    responses((status = 200, description = "Historial de hosts agrupados por hardware", body = Vec<HostsGroupbyHardwareHistory>))
)]
async fn get_hosts_groupby_hardware_history() {}

#[utoipa::path(
    get, path = "/api/hosts/groupby/domain",
    tag = "hosts",
    responses((status = 200, description = "Hosts agrupados por dominio DNS", body = Vec<HostsGroupbyDomain>))
)]
async fn get_hosts_groupby_domain() {}

#[utoipa::path(
    get, path = "/api/hosts/groupby/domain/history",
    tag = "hosts",
    responses((status = 200, description = "Historial de hosts agrupados por dominio", body = Vec<HostsGroupbyDomainHistory>))
)]
async fn get_hosts_groupby_domain_history() {}

#[utoipa::path(
    get, path = "/api/hosts/groupby/location/environment",
    tag = "hosts",
    responses((status = 200, description = "Hosts agrupados por localización y entorno", body = Vec<HostsGroupbyLocationEnvironment>))
)]
async fn get_hosts_groupby_location_environment() {}

#[utoipa::path(
    get, path = "/api/hosts/resources/groupby/location",
    tag = "hosts",
    responses((status = 200, description = "Recursos (CPU, memoria, disco) agrupados por localización", body = Vec<OSResourcesByLocation>))
)]
async fn get_hosts_resources_groupby_location() {}

#[utoipa::path(
    get, path = "/api/hosts/erratas",
    tag = "hosts",
    responses((status = 200, description = "Estado de compliance de erratas por host", body = Vec<ErrataCompliant>))
)]
async fn get_hosts_erratas() {}

#[utoipa::path(
    get, path = "/api/hosts/erratas/noncompliant/{days}",
    tag = "hosts",
    params(("days" = i32, Path, description = "Número de días para considerar no-compliant")),
    responses((status = 200, description = "Hosts no-compliant según días especificados", body = Vec<ErrataCompliant>))
)]
async fn get_hosts_erratas_noncompliant() {}

#[utoipa::path(
    get, path = "/api/hosts/erratas/groupby/bu",
    tag = "hosts",
    responses((status = 200, description = "Erratas no-compliant agrupadas por BU", body = Vec<ErrataCompliantGroupbyBu>))
)]
async fn get_hosts_erratas_groupby_bu() {}

#[utoipa::path(
    get, path = "/api/hosts/erratas/groupby/bu/noncompliant/{days}",
    tag = "hosts",
    params(("days" = i32, Path, description = "Número de días para considerar no-compliant")),
    responses((status = 200, description = "Erratas no-compliant agrupadas por BU según días", body = Vec<ErrataCompliantGroupbyBu>))
)]
async fn get_hosts_erratas_groupby_bu_noncompliant() {}

#[utoipa::path(
    get, path = "/api/hosts/erratas/groupby/environment/bu",
    tag = "hosts",
    responses((status = 200, description = "Erratas no-compliant agrupadas por entorno y BU", body = Vec<ErrataCompliantGroupbyEnvironmentBu>))
)]
async fn get_hosts_erratas_groupby_environment_bu() {}

#[utoipa::path(
    get, path = "/api/hosts/erratas/groupby/environment/bu/noncompliant/{days}",
    tag = "hosts",
    params(("days" = i32, Path, description = "Número de días para considerar no-compliant")),
    responses((status = 200, description = "Erratas no-compliant agrupadas por entorno y BU según días", body = Vec<ErrataCompliantGroupbyEnvironmentBu>))
)]
async fn get_hosts_erratas_groupby_environment_bu_noncompliant() {}

#[utoipa::path(
    get, path = "/api/hosts/facts",
    tag = "hosts",
    responses((status = 200, description = "Lista de facts visibles disponibles", body = Vec<VisibleFacts>))
)]
async fn get_hosts_facts() {}

#[utoipa::path(
    get, path = "/api/hosts/facts/{key}",
    tag = "hosts",
    params(("key" = String, Path, description = "Nombre del fact")),
    responses((status = 200, description = "Detalle de un fact específico", body = Vec<Fact>))
)]
async fn get_hosts_fact() {}

#[utoipa::path(
    get, path = "/api/hosts/groupby/fact/{key}",
    tag = "hosts",
    params(("key" = String, Path, description = "Nombre del fact para agrupar")),
    responses((status = 200, description = "Hosts agrupados por valor de un fact", body = Vec<HostsGroupbyFact>))
)]
async fn get_hosts_groupby_fact() {}

#[utoipa::path(
    get, path = "/api/hosts/groupby/fact/{key}/{value}",
    tag = "hosts",
    params(
        ("key" = String, Path, description = "Nombre del fact"),
        ("value" = String, Path, description = "Valor del fact para filtrar"),
    ),
    responses((status = 200, description = "Hosts filtrados por fact y valor", body = Vec<HostsGroupbyFact>))
)]
async fn get_hosts_groupby_fact_value() {}

#[utoipa::path(
    get, path = "/api/updated/",
    tag = "updated",
    responses((status = 200, description = "Resumen global de estado de actualización", body = Vec<Updated>))
)]
async fn get_updated() {}

#[utoipa::path(
    get, path = "/api/updated/compliant",
    tag = "updated",
    responses((status = 200, description = "Resumen de compliance de actualización", body = Vec<UpdatedCompliant>))
)]
async fn get_updated_compliant() {}

#[utoipa::path(
    get, path = "/api/updated/groupby/environment",
    tag = "updated",
    responses((status = 200, description = "Estado de actualización agrupado por entorno", body = Vec<UpdatedGroupByEnvironment>))
)]
async fn get_updated_groupby_environment() {}

#[utoipa::path(
    get, path = "/api/updated/groupby/environment/compliant",
    tag = "updated",
    responses((status = 200, description = "Compliance de actualización agrupado por entorno", body = Vec<UpdatedGroupByEnvironmentCompliant>))
)]
async fn get_updated_groupby_environment_compliant() {}

#[utoipa::path(
    get, path = "/api/updated/groupby/location",
    tag = "updated",
    responses((status = 200, description = "Estado de actualización agrupado por localización", body = Vec<UpdatedGroupByLocation>))
)]
async fn get_updated_groupby_location() {}

#[utoipa::path(
    get, path = "/api/updated/groupby/location/compliant",
    tag = "updated",
    responses((status = 200, description = "Compliance de actualización agrupado por localización", body = Vec<UpdatedGroupByLocationCompliant>))
)]
async fn get_updated_groupby_location_compliant() {}

#[utoipa::path(
    get, path = "/api/nonupdated/groupby/adminenvironment",
    tag = "nonupdated",
    responses((status = 200, description = "Hosts no actualizados agrupados por admin y entorno", body = Vec<NonUpdatedGroupByAdminEnvironment>))
)]
async fn get_nonupdated_adminenvironment() {}

#[utoipa::path(
    get, path = "/api/nonupdated/groupby/adminenvironment/compliant",
    tag = "nonupdated",
    responses((status = 200, description = "Hosts no actualizados (compliant) por admin y entorno", body = Vec<NonUpdatedGroupByAdminEnvironmentCompliant>))
)]
async fn get_nonupdated_adminenvironment_compliant() {}

#[utoipa::path(
    get, path = "/api/report/os",
    tag = "report",
    responses((status = 200, description = "Reporte completo de SO de todos los hosts", body = Vec<OSReport>))
)]
async fn get_report_os() {}

#[utoipa::path(
    get, path = "/api/report/facts",
    tag = "report",
    responses((status = 200, description = "Reporte con todos los facts de cada host", body = Vec<HostFactsReport>))
)]
async fn get_report_facts() {}

#[utoipa::path(
    get, path = "/api/report/facts/{key}",
    tag = "report",
    params(("key" = String, Path, description = "Nombre del fact")),
    responses((status = 200, description = "Reporte filtrado por un fact específico", body = Vec<FactsReport>))
)]
async fn get_report_facts_key() {}

#[utoipa::path(
    get, path = "/api/report/facts/{key}/{value}",
    tag = "report",
    params(
        ("key" = String, Path, description = "Nombre del fact"),
        ("value" = String, Path, description = "Valor del fact para filtrar"),
    ),
    responses((status = 200, description = "Reporte filtrado por fact y valor", body = Vec<FactsReport>))
)]
async fn get_report_facts_key_value() {}

#[utoipa::path(
    get, path = "/api/report/facts/{key}/cpu/groupby/environment",
    tag = "report",
    params(("key" = String, Path, description = "Nombre del fact")),
    responses((status = 200, description = "CPUs agrupadas por entorno para un fact", body = Vec<CPUFactsReportGroupByEnvironment>))
)]
async fn get_report_facts_key_cpu_groupby_environment() {}

#[utoipa::path(
    get, path = "/api/report/facts/{key}/cpu/history",
    tag = "report",
    responses((status = 200, description = "Historial de CPUs por fact", body = Vec<CPUFactsHistory>))
)]
async fn get_report_facts_key_cpu_history() {}

#[utoipa::path(
    get, path = "/api/report/facts/{key}/groupby/environment",
    tag = "report",
    params(("key" = String, Path, description = "Nombre del fact")),
    responses((status = 200, description = "Facts agrupados por entorno", body = Vec<FactsReportGroupByEnvironment>))
)]
async fn get_report_facts_key_groupby_environment() {}

#[utoipa::path(
    get, path = "/api/report/facts/{key}/{value}/groupby/environment",
    tag = "report",
    params(
        ("key" = String, Path, description = "Nombre del fact"),
        ("value" = String, Path, description = "Valor del fact"),
    ),
    responses((status = 200, description = "Facts filtrados por valor agrupados por entorno", body = Vec<FactsReportGroupByEnvironment>))
)]
async fn get_report_facts_key_value_groupby_environment() {}

#[utoipa::path(
    get, path = "/api/erratas/",
    tag = "erratas",
    responses((status = 200, description = "Catálogo de erratas disponibles", body = Vec<Erratas>))
)]
async fn get_erratas() {}

#[utoipa::path(
    get, path = "/api/contentviews/",
    tag = "contentviews",
    responses((status = 200, description = "Content Views y sus releases", body = Vec<ContentViews>))
)]
async fn get_contentviews() {}

#[utoipa::path(
    get, path = "/api/os/eol",
    tag = "os",
    responses((status = 200, description = "Fechas de End of Life por versión de SO", body = Vec<OSEOL>))
)]
async fn get_os_eol() {}

#[utoipa::path(
    get, path = "/api/openscap/",
    tag = "openscap",
    responses((status = 200, description = "Reporte de OpenSCAP de todos los hosts", body = Vec<OpenscapReport>))
)]
async fn get_openscap() {}

#[utoipa::path(
    get, path = "/api/openscap/host/{key}",
    tag = "openscap",
    params(("key" = String, Path, description = "Hostname")),
    responses((status = 200, description = "Reporte OpenSCAP detallado de un host", body = Vec<OpenscapHostReport>))
)]
async fn get_openscap_host() {}

#[utoipa::path(
    get, path = "/api/openscap/rule/{key}",
    tag = "openscap",
    params(("key" = String, Path, description = "Nombre de la regla OpenSCAP")),
    responses((status = 200, description = "Resultado de una regla OpenSCAP en todos los hosts", body = Vec<OpenscapRuleReport>))
)]
async fn get_openscap_rule() {}

#[utoipa::path(
    post, path = "/api/login",
    tag = "auth",
    request_body(content = LoginRequest, description = "Credenciales de usuario"),
    responses(
        (status = 200, description = "Login exitoso", body = String),
        (status = 401, description = "Credenciales incorrectas"),
    )
)]
async fn post_login() {}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
