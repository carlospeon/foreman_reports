use sqlx::{FromRow};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use bigdecimal::BigDecimal;

use crate::{MatView, Param, SQL, Sql, SqlParams};

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ReportHistory();
impl<'a> SQL<'a> for ReportHistory {
  fn sql() -> Sql<'a> {
//    "select date_part('week', gathered_at_week)::smallint as gathered_at_week, key, value from (
//      select date_trunc('week',gathered_at) as gathered_at_week,
//        r.key, floor(avg(r.value))::int as value
//      from history_reports hr
//        cross join lateral json_populate_recordset(NULL::history_report_t, report) as r
//      where hr.name = $1 and
//        gathered_at >= now() - '1 year'::interval + '8 days'::interval
//      group by gathered_at_week, r.key
//      order by gathered_at_week asc, value desc
    "select 
      date_part('year', gathered_at_week)::smallint || '-' ||
      TO_CHAR(date_part('week', gathered_at_week)::smallint, 'FM00') as gathered_at_week,
      key, value
     from (
      select date_trunc('week',gathered_at) as gathered_at_week,
        r.key, floor(avg(r.value))::int as value
      from history_reports hr
        cross join lateral json_populate_recordset(NULL::history_report_t, report) as r
      where hr.name = $1 and
        gathered_at >= now() - '2 year'::interval
      group by gathered_at_week, r.key
      order by gathered_at_week asc, value desc
    ) as r".into()
  }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct OSReport {
    pub hostname: String,
    pub bu: String,
    pub comment:  Option<String>,
    //pub reported_at: chrono::DateTime<chrono::Utc>,
    pub last_report: Option<chrono::NaiveDateTime>,
    pub location:  Option<String>,
    pub environment:  Option<String>,
    pub subnet:  Option<String>,
    pub product_name:  Option<String>,
    pub os_version:  Option<String>,
    pub kernel_release:  Option<String>,
    pub facts_datetime: Option<String>,
    pub sockets: Option<String>,
    pub cpu: Option<String>,
    pub memorysize: Option<String>,
    pub dfsize: Option<String>,
}

impl<'a> SQL<'a> for OSReport {
  fn sql() -> Sql<'a> {
    "select
      hostname,
      bu,
      comment,
      last_report,
      location,
      environment,
      subnet,
      facts->>'facts_datetime'  as facts_datetime,
      facts->>'product_name'    as product_name,
      facts->>'os_version'      as os_version,
      facts->>'kernel_release'  as kernel_release,
      facts->>'sockets'         as sockets,
      facts->>'online_cpu'      as cpu,
      facts->>'memorysize'      as memorysize,
      facts->>'dfsize'          as dfsize
    from full_report
    order by hostname asc ".into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report"]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct OSResourcesByLocation {
    pub location:  Option<String>,
    pub cpu: Option<i64>,
    pub memorysize: Option<BigDecimal>,
    pub dfsize: Option<BigDecimal>,
}

impl<'a> SQL<'a> for OSResourcesByLocation {
  fn sql() -> Sql<'a> {
    "select location, 
      sum(cast(facts->>'online_cpu' as bigint))::bigint as cpu, 
      sum(cast(facts->>'memorysize' as decimal(10,2)))::decimal(10,2) as memorysize, 
      sum(cast(facts->>'dfsize' as decimal(12,0)))::decimal(12,0) as dfsize 
    from full_report 
    group by location 
    order by location asc".into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report"]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct FactsReport {
    pub hostname: String,
    pub bu: String,
    pub comment:  Option<String>,
    //pub reported_at: chrono::DateTime<chrono::Utc>,
    // pub last_report: Option<chrono::NaiveDateTime>,
    pub location:  Option<String>,
    pub environment:  Option<String>,
    pub os_version:  Option<String>,
    pub facts_datetime: Option<String>,
    pub facts_key:  Option<String>,
    pub sockets: Option<String>,
    pub cpu: Option<String>,
    pub memorysize: Option<String>,
    pub dfsize: Option<String>,
}

impl<'a> SQL<'a> for FactsReport {
  fn sql_with_params(p: Vec<&String>) -> Sql<'a> {
    format!("select
      hostname,
      bu,
      comment,
      location,
      environment,
      facts->>'facts_datetime' as facts_datetime,
      facts->>'os_version'     as os_version,
      facts->>'sockets'         as sockets,
      facts->>'online_cpu'      as cpu,
      facts->>'memorysize'      as memorysize,
      facts->>'dfsize'          as dfsize,
      facts->>'{}'             as facts_key
    from full_report
    where {}
    order by hostname asc ", 
      p[0],
      p[1]).into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report"]) }
  fn default_filter(p: Vec<&String>) -> Sql<'a> { format!("facts->>'{}' is not null and
      facts->>'{}' != '' and
      facts->>'{}' != 'None' and
      facts->>'{}' not like '% not installed'", p[0], p[0], p[0], p[0]).into() }
  fn filter(p: Vec<&String>) -> Sql<'a> { format!("facts->>'{}' = $1", p[0]).into() }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct FactsReportGroupByEnvironment {
    pub environment: Option<String>,
    pub facts_key: Option<String>,
    pub count: i64,
}

impl<'a> SQL<'a> for FactsReportGroupByEnvironment {
  fn sql_with_params(p: Vec<&String>) -> Sql<'a> {
    format!("select
      environment,
      facts->>'{}'             as facts_key,
      count(id)                as count
    from full_report
    where
      facts->>'{}' is not null and
      facts->>'{}' != '' and
      facts->>'{}' != 'None' and
      facts->>'{}' not like '% not installed'
      {}
    group by environment, facts_key
    order by environment asc, facts_key desc", 
      p[0],
      p[0],
      p[0],
      p[0],
      p[0],
      p[1]).into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report"]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CPUFactsReportGroupByEnvironment {
    pub environment: Option<String>,
    // pub facts_key: Option<String>,
    pub cpu: i64,
}

impl<'a> SQL<'a> for CPUFactsReportGroupByEnvironment {
  fn sql_with_params(p: Vec<&String>) -> Sql<'a> {
    format!("select
      environment,
      -- facts->>'{}'              as facts_key,
      sum(cast(facts->>'online_cpu' as bigint))::bigint as cpu
    from full_report
    where
      facts->>'{}' is not null and
      facts->>'{}' != '' and
      facts->>'{}' != 'None' and
      facts->>'{}' not like '% not installed'
      {}
    group by environment
    order by environment asc", 
      p[0],
      p[0],
      p[0],
      p[0],
      p[0],
      p[1]).into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report"]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct HostsGroupbyFact {
    pub facts_key: String,
    pub count: i64,
}

impl<'a> SQL<'a> for HostsGroupbyFact {
  fn sql_with_params(p: Vec<&String>) -> Sql<'a> {
    format!("select
      facts->>'{}'             as facts_key,
      count(id)                as count
    from full_report
    where
      facts->>'{}' is not null and
      facts->>'{}' != '' and
      facts->>'{}' != 'None' and
      facts->>'{}' not like '% not installed'
      {}
    group by facts_key
    order by count desc", 
      p[0],
      p[0],
      p[0],
      p[0],
      p[0],
      p[1]).into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report"]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}

// #[derive(Debug, FromRow, Deserialize, Serialize)]
// #[allow(non_snake_case)]
// pub struct ErrataReport {
//   pub hostname: String,
//   pub comment:  Option<String>,
//   pub updated_at: Option<chrono::NaiveDateTime>,
//   pub location:  Option<String>,
//   pub environment:  Option<String>,
//   pub os_version:  Option<String>,
//   pub kernel_release:  Option<String>,
//   pub boot_time:  Option<String>,
//   pub security_erratas:  Option<i32>,
//   pub older_errata:  Option<String>,
//   pub errata_list:  Option<String>,
//   pub cve_list:  Option<String>,
// }

// impl<'a> SQL<'a> for ErrataReport {
//   fn sql() -> Sql<'a> {
//     "select
//       hostname,
//       comment,
//       updated_at,
//       location,
//       environment,
//       facts->>'os_version'       as os_version,
//       facts->>'kernel_release'   as kernel_release,
//       facts->>'boot_time'        as boot_time,
//       cast(facts->>'security_erratas' as integer) as security_erratas,
//       facts->>'older_errata'     as older_errata,
//       facts->>'errata_list'      as errata_list,
//       facts->>'cve_list'         as cve_list
//     from full_report
//     where 
//       facts->'security_erratas' is null or
//       cast(facts->>'security_erratas' as integer) > 0
//     order by security_erratas desc, hostname asc".into()
//   }
//   fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report"]) }
// }

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct OSEOL {
  pub os: String,
  pub eol: chrono::NaiveDate,
}
impl<'a> SQL<'a> for OSEOL {
  fn sql() -> Sql<'a> {
    "select
      'RHEL ' || major as os,
      eol::date as eol
    from os
    order by eol desc ".into()
  }
}


#[derive(Debug, FromRow, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct HostFactsReport {
    pub hostname: String,
    pub bu: String,
    pub comment:  Option<String>,
    //pub reported_at: chrono::DateTime<chrono::Utc>,
    pub last_report: Option<chrono::NaiveDateTime>,
    pub location:  Option<String>,
    pub environment:  Option<String>,
    pub subnet:  Option<String>,
    pub facts: Option<JsonValue>,
}

impl<'a> SQL<'a> for HostFactsReport {
  fn sql() -> Sql<'a> {
    "select
      hostname,
      bu,
      comment,
      last_report,
      location,
      environment,
      subnet,
      facts
    from full_report
    order by hostname asc ".into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report"]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CPUFactsHistory {
  // pub gathered_at: Option<chrono::DateTime<chrono,b::Utc>>,
  pub gathered_at_week: String,
  pub key: String,
  pub value: i32
}
impl<'a> SQL<'a> for CPUFactsHistory {
  fn sql() -> Sql<'a> { ReportHistory::sql() }
  fn params() -> SqlParams<'a> { Some(vec![Param::Str("cpus_groupby_fact".into())]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct OpenscapReport {
    pub hostname: String,
    pub bu: String,
    pub comment:  Option<String>,
    pub reported_at: Option<chrono::NaiveDateTime>,
    pub location:  Option<String>,
    pub environment:  Option<String>,
    pub os:  Option<String>,
    pub metrics:  Option<JsonValue>,
}

impl<'a> SQL<'a> for OpenscapReport {
  fn sql() -> Sql<'a> {
    format!("select
      hostname,
      bu,
      comment,
      reported_at,
      location,
      environment,
      os,
      metrics
      from openscap_report
      order by hostname asc").into()
  }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec![MatView { name: "openscap_report", ttl: Some(8)}]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct OpenscapHostReport {
    pub hostname: String,
    pub bu: String,
    pub comment:  Option<String>,
    pub reported_at: Option<chrono::NaiveDateTime>,
    pub location:  Option<String>,
    pub environment:  Option<String>,
    pub os:  Option<String>,
    pub metrics:  Option<JsonValue>,
    pub result:  Option<JsonValue>,
}

impl<'a> SQL<'a> for OpenscapHostReport {
  fn sql() -> Sql<'a> {
    format!("select
      hostname,
      bu,
      comment,
      reported_at,
      location,
      environment,
      metrics,
      os,
      result
      from openscap_report
      where hostname = $1").into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["openscap_report"]) }
  // fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec![("openscap_report", 8).into()]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec![MatView { name: "openscap_report", ttl: Some(8)}]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct OpenscapRuleReport {
    pub hostname: String,
    pub bu: String,
    pub comment:  Option<String>,
    //pub reported_at: chrono::DateTime<chrono::Utc>,
    pub reported_at: Option<chrono::NaiveDateTime>,
    // pub last_report: Option<chrono::NaiveDateTime>,
    pub location:  Option<String>,
    pub environment:  Option<String>,
    pub os:  Option<String>,
    pub result:  Option<String>,
}

impl<'a> SQL<'a> for OpenscapRuleReport {
  fn sql_with_params(p: Vec<&String>) -> Sql<'a> {
    format!("select
      hostname,
      bu,
      comment,
      reported_at,
      location,
      environment,
      os,
      result->>'{}' as result
      from openscap_report
      where result->>'{}' is not null",
      p[0],
      p[0]
      ).into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["openscap_report"]) }
  //fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec![("openscap_report", 8).into()]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec![MatView { name: "openscap_report", ttl: Some(8)}]) }
}
