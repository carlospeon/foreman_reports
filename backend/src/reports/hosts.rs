use sqlx::{FromRow};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{MatView, Param, SQL, Sql, SqlParams, report::ReportHistory};

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct HostInfo {
    pub hostname: String,
    pub bu: String,
    pub comment:  Option<String>,
    pub location:  Option<String>,
    pub environment:  Option<String>,
    pub facts_datetime: Option<String>,
    pub os_version:  Option<String>,
}

impl<'a> SQL<'a> for HostInfo {
  fn sql() -> Sql<'a> {
    format!("select
      hostname,
      bu,
      comment,
      location,
      environment,
      facts->>'facts_datetime'  as facts_datetime,
      facts->>'os_version'      as os_version
      from full_report
      where hostname = $1").into()
  }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec![MatView::from("full_report")]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct HostsGroupbyOsmajor {
    osmajor: String,
    count: i64,
}

impl<'a> SQL<'a> for HostsGroupbyOsmajor {
  fn sql() -> Sql<'a> {
    "select 
        case when facts->>'os_version' is null then
          '-N/A-'
        else
          'RHEL ' || substring(facts->>'os_version', 0, position('.' in facts->>'os_version'))
        end as osmajor, 
        count(id) as count
      from full_report 
      group by osmajor
      order by count desc".into()
  }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec![MatView::from("full_report")]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct HostsGroupbyOSmajorHistory {
  // pub gathered_at: Option<chrono::DateTime<chrono::Utc>>,
  pub gathered_at_week: String,
  pub key: String,
  pub value: i32
}
impl<'a> SQL<'a> for HostsGroupbyOSmajorHistory {
  fn sql() -> Sql<'a> { ReportHistory::sql() }
  fn params() -> SqlParams<'a> { Some(vec![Param::Str("hosts_groupby_osmajor".into())]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct HostsGroupbyEnvironment {
    environment: String,
    count: i64,
}
impl<'a> SQL<'a> for HostsGroupbyEnvironment {
  fn sql() -> Sql<'a> {
    "select 
      coalesce(environment, '-N/A-') as environment,
      count(id) as count
    from full_report
    group by environment
    order by count desc".into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report".into()]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct HostsGroupbyLocation {
    location: String,
    count: i64,
}
impl<'a> SQL<'a> for HostsGroupbyLocation {
  fn sql() -> Sql<'a> {
    "select 
      coalesce(location, '-N/A-') as location,
      count(id) as count
    from full_report
    group by location
    order by count desc".into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report".into()]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct HostsGroupbyLocationEnvironment {
    location: String,
    environment: String,
    count: i64,
}
impl<'a> SQL<'a> for HostsGroupbyLocationEnvironment {
  fn sql() -> Sql<'a> {
    "select 
      coalesce(location, '-N/A-') as location,
      coalesce(environment, '-N/A-') as environment,
      count(id) as count
    from full_report
    group by location, environment
    order by location asc, environment asc, count desc".into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report".into()]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct HostsGroupbyAdminGroup {
    admingroup: String,
    count: i64,
}
impl<'a> SQL<'a> for HostsGroupbyAdminGroup {
  fn sql() -> Sql<'a> {
    "select 
      case 
      when (comment is null or bu = 'sscc') then
        '-N/A-'
      when substring(comment, 0, position(':' in comment)) = '' then
        '-N/A-'
      else
        substring(comment, 0, position(':' in comment))
      end as admingroup, 
      count(id) as count
    from full_report 
    group by admingroup
    order by count desc".into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report".into()]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct HostsGroupbyBU {
    bu: String,
    count: i64,
}
impl<'a> SQL<'a> for HostsGroupbyBU {
  fn sql() -> Sql<'a> {
    "select 
      bu, 
      count(id) as count
    from full_report 
    group by bu
    order by count desc".into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report".into()]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct HostsGroupbyHardware {
    product_name: String,
    count: i64,
}
impl<'a> SQL<'a> for HostsGroupbyHardware {
  fn sql() -> Sql<'a> {
    "select 
      case when facts->>'product_name' is null then
        '-N/A-'
      else
        facts->>'product_name'
      end as product_name, 
      count(id) as count
    from full_report 
    group by product_name
    order by count desc".into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report".into()]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct HostsGroupbyHardwareHistory {
  // pub gathered_at: Option<chrono::DateTime<chrono::Utc>>,
  pub gathered_at_week: String,
  pub key: String,
  pub value: i32
}
impl<'a> SQL<'a> for HostsGroupbyHardwareHistory {
  fn sql() -> Sql<'a> { ReportHistory::sql() }
  fn params() -> SqlParams<'a> { Some(vec![Param::Str("hosts_groupby_hardware".into())]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct HostsGroupbyDomain {
    domain: String,
    count: i64,
}
impl<'a> SQL<'a> for HostsGroupbyDomain {
  fn sql() -> Sql<'a> {
    "select 
      substring(hostname, position('.' in hostname) + 1) as domain,
      count(id) as count
    from full_report 
    group by domain
    order by count desc".into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report".into()]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}
#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct HostsGroupbyDomainHistory {
  // pub gathered_at: Option<chrono::DateTime<chrono::Utc>>,
  pub gathered_at_week: String,
  pub key: String,
  pub value: i32
}
impl<'a> SQL<'a> for HostsGroupbyDomainHistory {
  fn sql() -> Sql<'a> { ReportHistory::sql() }
  fn params() -> SqlParams<'a> { Some(vec![Param::Str("hosts_groupby_domain".into())]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct VisibleFacts {
  name: String,
  description: String,
}

impl<'a> SQL<'a> for VisibleFacts {
  fn sql() -> Sql<'a> {
    "select short_name as name, description
    from facts
    where visible = true
    order by description asc".into()
  }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct Fact {
  name: String,
  description: String,
  aggregable: bool,
  online_cpu: bool,
}
impl<'a> SQL<'a> for Fact {
  fn sql() -> Sql<'a> {
    "select short_name as name, 
      description, aggregable, online_cpu
    from facts
    where short_name = $1".into()
  }
}
