use sqlx::{FromRow};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{MatView, SQL, Sql};

const SUPPORTED_CONDITION: &'static str = "substring(facts->>'os_version', 0, position('.' in facts->>'os_version')) \
                              in (select major from supported_os)";

const UPDATED_CONDITION: &'static str = "facts->>'security_erratas' = '0'";

// const COMPLIANT_CONDITION: &'static str = "cast(facts->>'older_errata' as date) >= ( \
//                                             select v.created_at::date \
//                                             from digital.katello_content_view_versions v \
//                                               left join digital.katello_content_view_histories h on \
//                                                 v.id = h.katello_content_view_version_id and \
//                                                 h.action=1 \
//                                             where h.notes like $1 and content_view_id = $2 \
//                                             order by 1 desc limit 1 offset 1 \
//                                           )";
// fn params() -> SqlParams<'a> { Some(vec![ Param::Str("OS update%".into()), Param::I32(56)]) }

// const COMPLIANT_CONDITION: &'static str = "cast(facts->>'older_errata' as date) >= ( \
//                                             select compliant_update_date \
//                                             from content_views
//                                             where bu = $1 )";
const COMPLIANT_CONDITION: &'static str = "cast(facts->>'older_errata' as date) >= compliant_update_date";

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct Updated {
  pub total: i64,
  pub supported: i64,
  pub supported_p: String,
  pub updated: i64,
  pub updated_p: String,
}
impl<'a> SQL<'a> for Updated {
  fn sql() -> Sql<'a> {
    format!("select 
      total,
      supported,
      round((cast(supported as double precision) * 100 / total)::numeric, 1)::text as supported_p,
      updated,
      round((cast(updated as double precision) * 100 / total)::numeric, 1)::text as updated_p
    from (
      select 
          count(id) as total,
          sum(case when {}
            then 1 else 0 end) as supported,
          sum(case when {} and {}
            then 1 else 0 end) as updated
        from full_report
      ) q
      order by total asc", SUPPORTED_CONDITION, SUPPORTED_CONDITION, UPDATED_CONDITION).into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report"]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}


#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct UpdatedGroupByEnvironment {
  pub environment: String,
  pub total: i64,
  pub supported: i64,
  pub updated: i64
}
impl<'a> SQL<'a> for UpdatedGroupByEnvironment {
  fn sql() -> Sql<'a> {
    format!("select coalesce(environment, '-N/A-') as environment,
        count(id) as total,
        sum(case when {}
          then 1 else 0 end) as supported,
        sum(case when {} and {}
          then 1 else 0 end) as updated
      from full_report
      group by environment
      order by grouping(environment) asc, 1", 
      SUPPORTED_CONDITION, SUPPORTED_CONDITION, UPDATED_CONDITION).into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report"]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct UpdatedGroupByLocation {
  pub location: String,
  pub total: i64,
  pub supported: i64,
  pub updated: i64
}
impl<'a> SQL<'a> for UpdatedGroupByLocation {
  fn sql() -> Sql<'a> {
    format!("select coalesce(location, '-N/A-') as location,
        count(id) as total,
        sum(case when {}
          then 1 else 0 end) as supported,
        sum(case when {} and {}
          then 1 else 0 end) as updated
      from full_report
      group by location
      order by grouping(location) asc, 1", 
      SUPPORTED_CONDITION, SUPPORTED_CONDITION, UPDATED_CONDITION).into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report"]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct NonUpdatedGroupByAdminEnvironment {
  pub admin: String,
  pub environment: String,
  pub total: i64,
  pub non_updated: i64
}
impl<'a> SQL<'a> for NonUpdatedGroupByAdminEnvironment {
  fn sql() -> Sql<'a> {
    format!("select 
      admin,
      -- case when admin='' then '-N/A-'
      -- else admin end,
      environment, total, non_updated from (
        select 
          case 
            when (comment is null or bu = 'sscc') then '-N/A-'
            when substring(comment, 0, position(':' in comment)) = '' then '-N/A-'
            else substring(comment, 0, position(':' in comment))
          end as admin, 
          -- coalesce(substring(comment, 0, position(':' in comment)), '-N/A-') as admin,
          coalesce(environment, '-N/A-') as environment,
          count(id) as total,
          sum(case when {} and {}
            then 0 else 1 end) as non_updated
          from full_report
        group by admin,environment
      ) q
    order by admin asc, environment asc", SUPPORTED_CONDITION, UPDATED_CONDITION).into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report"]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into()]) }
}


#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct UpdatedCompliant {
  pub total: i64,
  pub supported: i64,
  pub supported_p: String,
  pub updated: i64,
  pub updated_p: String,
}
impl<'a> SQL<'a> for UpdatedCompliant {
  fn sql() -> Sql<'a> {
    format!("select 
      total,
      supported,
      round((cast(supported as double precision) * 100 / total)::numeric, 1)::text as supported_p,
      updated,
      round((cast(updated as double precision) * 100 / total)::numeric, 1)::text as updated_p
    from (
      select 
          count(r.id) as total,
          sum(case when {}
            then 1 else 0 end) as supported,
          sum(case when {} and ( {} or {} )
            then 1 else 0 end) as updated
      from full_report r join content_views c on r.bu = c.bu
    ) q
    order by total asc", 
    SUPPORTED_CONDITION, SUPPORTED_CONDITION, UPDATED_CONDITION, COMPLIANT_CONDITION).into()
  }
  // fn params() -> SqlParams<'a> { Some(vec![ Param::Str("digital".into())]) }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report", "content_view_releases"]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into(), "content_view_releases".into()]) }
}
#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct UpdatedGroupByEnvironmentCompliant {
  pub environment: String,
  pub total: i64,
  pub supported: i64,
  pub updated: i64
}
impl<'a> SQL<'a> for UpdatedGroupByEnvironmentCompliant {
  fn sql() -> Sql<'a> {
    format!("select coalesce(environment, '-N/A-') as environment,
        count(r.id) as total,
        sum(case when {}
          then 1 else 0 end) as supported,
        sum(case when {} and ( {} or {} )
          then 1 else 0 end) as updated
      from full_report r join content_views c on r.bu = c.bu
      group by environment
      order by grouping(environment) asc, 1",
      SUPPORTED_CONDITION, SUPPORTED_CONDITION, UPDATED_CONDITION, COMPLIANT_CONDITION).into()
  }
  // fn params() -> SqlParams<'a> { Some(vec![ Param::Str("digital".into())]) }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report", "content_view_releases"]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into(), "content_view_releases".into()]) }
}
#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct UpdatedGroupByLocationCompliant {
  pub location: String,
  pub total: i64,
  pub supported: i64,
  pub updated: i64
}
impl<'a> SQL<'a> for UpdatedGroupByLocationCompliant {
  fn sql() -> Sql<'a> {
    format!("select coalesce(location, '-N/A-') as location,
        count(r.id) as total,
        sum(case when {}
          then 1 else 0 end) as supported,
        sum(case when {} and ( {} or {} )
          then 1 else 0 end) as updated
      from full_report r join content_views c on r.bu = c.bu
      group by location
      order by grouping(location) asc, 1",
      SUPPORTED_CONDITION, SUPPORTED_CONDITION, UPDATED_CONDITION, COMPLIANT_CONDITION).into()
  }
  // fn params() -> SqlParams<'a> { Some(vec![ Param::Str("digital".into())]) }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report", "content_view_releases"]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into(), "content_view_releases".into()]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct NonUpdatedGroupByAdminEnvironmentCompliant {
  pub admin: String,
  pub environment: String,
  pub total: i64,
  pub non_updated: i64
}
impl<'a> SQL<'a> for NonUpdatedGroupByAdminEnvironmentCompliant {
  fn sql() -> Sql<'a> {
    format!("select admin,
      -- case when admin='' then '-N/A-'
      -- else admin end,
      environment, total, non_updated from (
        select 
          case 
            when (comment is null or r.bu = 'sscc') then '-N/A-'
            when substring(comment, 0, position(':' in comment)) = '' then '-N/A-'
            else substring(comment, 0, position(':' in comment))
          end as admin,
          -- coalesce(substring(comment, 0, position(':' in comment)), '-N/A-') as admin,
          coalesce(environment, '-N/A-') as environment,
          count(r.id) as total,
          sum(case when {} and ( {} or {} )
            then 0 else 1 end) as non_updated
          from full_report r join content_views c on r.bu = c.bu
        group by admin,environment
      ) q
    order by admin asc, environment asc",
    SUPPORTED_CONDITION, UPDATED_CONDITION, COMPLIANT_CONDITION).into()
  }
  // fn params() -> SqlParams<'a> { Some(vec![ Param::Str("digital".into())]) }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report", "content_view_releases"]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into(), "content_view_releases".into()]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct ErrataCompliant {
  pub hostname: String,
  pub bu: String,
  pub comment:  Option<String>,
  pub updated_at: Option<chrono::NaiveDateTime>,
  pub location:  Option<String>,
  pub environment:  Option<String>,
  pub os_version:  Option<String>,
  pub kernel_release:  Option<String>,
  pub boot_time:  Option<String>,
  pub compliant: Option<bool>,
  pub supported: Option<bool>,
  pub security_erratas:  Option<i32>,
  pub older_errata:  Option<String>,
  pub errata_list:  Option<String>,
  pub cve_list:  Option<String>,
}

// impl<'a> SQL<'a> for ErrataCompliant {
//   fn sql() -> Sql<'a> {
//     format!("select
//       hostname,
//       'digital' as bu,
//       comment,
//       updated_at,
//       location,
//       environment,
//       facts->>'os_version'       as os_version,
//       facts->>'kernel_release'   as kernel_release,
//       facts->>'boot_time'        as boot_time,
//     case when {} and ({} or {})
//     then true 
//     else false end as compliant,
//       cast(facts->>'security_erratas' as integer) as security_erratas,
//       facts->>'older_errata'     as older_errata,
//       facts->>'errata_list'      as errata_list,
//       facts->>'cve_list'         as cve_list
//     from digital_report
//     union all
//     select
//       hostname,
//       'sscc' as bu,
//       comment,
//       updated_at,
//       location,
//       environment,
//       facts->>'os_version'       as os_version,
//       facts->>'kernel_release'   as kernel_release,
//       facts->>'boot_time'        as boot_time,
//     case when {} and ({} or {})
//     then true 
//     else false end as compliant,
//       cast(facts->>'security_erratas' as integer) as security_erratas,
//       facts->>'older_errata'     as older_errata,
//       facts->>'errata_list'      as errata_list,
//       facts->>'cve_list'         as cve_list
//     from sscc_report
//     order by security_erratas desc, hostname asc", 
//     SUPPORTED_CONDITION, UPDATED_CONDITION, COMPLIANT_CONDITION,
//     SUPPORTED_CONDITION, UPDATED_CONDITION, COMPLIANT_CONDITION).into()
//   }
//   fn params() -> SqlParams<'a> { Some(vec![ Param::Str("digital".into()), Param::Str("sscc".into())]) }
//   fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["digital_report", "digital_content_view_releases", 
//                                                        "sscc_report", "sscc_content_view_releases" ]) }
// }

impl<'a> SQL<'a> for ErrataCompliant {
    fn sql() -> Sql<'a> {
      format!("select
        hostname,
        r.bu,
        comment,
        updated_at,
        location,
        environment,
        facts->>'os_version'       as os_version,
        facts->>'kernel_release'   as kernel_release,
        facts->>'boot_time'        as boot_time,
        case when {} and ({} or {})
        then true 
        else false end as compliant,
        case when {}
        then true
        else false end as supported,
        cast(facts->>'security_erratas' as integer) as security_erratas,
        facts->>'older_errata'     as older_errata,
        facts->>'errata_list'      as errata_list,
        facts->>'cve_list'         as cve_list
      from full_report r join content_views c on r.bu = c.bu
      order by security_erratas desc, hostname asc", 
      SUPPORTED_CONDITION, UPDATED_CONDITION, COMPLIANT_CONDITION,
      SUPPORTED_CONDITION).into()
    }
    fn sql_with_params(p: Vec<&String>) -> Sql<'a> {
        format!("select
            hostname,
            bu,
            comment,
            updated_at,
            location,
            environment,
            os_version,
            kernel_release,
            boot_time,
            compliant,
            supported,
            security_erratas,
            older_errata,
            errata_list,
            cve_list
            from ( {} ) t
            where {} ", 
            ErrataCompliant::sql(),
            p[0]
            ).into() 
    }
    // fn params() -> SqlParams<'a> { Some(vec![ Param::Str("digital".into()) ]) }
    //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report", "content_view_releases" ]) }
    fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into(), "content_view_releases".into()]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct ErrataCompliantGroupbyBu {
  pub bu: String,
  pub non_updated: i64,
  pub non_supported: i64
}

impl<'a> SQL<'a> for  ErrataCompliantGroupbyBu {
    fn sql() -> Sql<'a> {
      format!("select
        bu,
        count(hostname) filter (where supported = true) as non_updated,
        count(hostname) filter (where supported = false) as non_supported
      from ( {} ) ec
      where compliant = false
      group by bu
      order by bu", ErrataCompliant::sql()).into()
    }
    fn sql_with_params(p: Vec<&String>) -> Sql<'a> {
      format!("select
        bu,
        count(hostname) filter (where supported = true) as non_updated,
        count(hostname) filter (where supported = false) as non_supported
      from ( {} ) ec
      where {}
      group by bu
      order by bu", 
      ErrataCompliant::sql(),
      p[0]).into()
    }
    //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report", "content_view_releases" ]) }
    fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into(), "content_view_releases".into()]) }
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct ErrataCompliantGroupbyEnvironmentBu {
  pub environment: String,
  pub bu: String,
  pub count: i64
}

impl<'a> SQL<'a> for  ErrataCompliantGroupbyEnvironmentBu {
    fn sql() -> Sql<'a> {
      format!("select
        bu,
        environment,
        count(hostname) as count
      from ( {} ) ec
      where compliant = false
      group by environment, bu", ErrataCompliant::sql()).into()
    }
    fn sql_with_params(p: Vec<&String>) -> Sql<'a> {
      format!("select
        bu,
        environment,
        count(hostname) as count
      from ( {} ) ec
      where {}
      group by environment, bu", 
      ErrataCompliant::sql(),
      p[0]).into()
    }
    //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["full_report", "content_view_releases" ]) }
    fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["full_report".into(), "content_view_releases".into()]) }
}


#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct ContentViews {
  pub id: i32,
  pub content_view_id: i32,
  pub bu: String,
  pub date: chrono::NaiveDate,
  pub description:  Option<String>,
}

impl<'a> SQL<'a> for ContentViews {
  fn sql() -> Sql<'a> {
    "select 
      id,
      content_view_id,
      bu,
      created_at as date,
      notes as description
    from content_view_releases
    order by content_view_id asc, created_at desc
    ".into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["content_view_releases"]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["content_view_releases".into()]) }
}
