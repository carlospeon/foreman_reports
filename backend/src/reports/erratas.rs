use sqlx::{FromRow};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{MatView, SQL, Sql};


#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct Erratas {
  errata_id: String,
  title: String,
  errata_type: String,
  severity: String,
  issued: chrono::naive::NaiveDate,
  reboot_suggested: bool,
  cve_list: Option<String>
}

impl<'a> SQL<'a> for Erratas {
  fn sql() -> Sql<'a> {
    "select 
      errata_id,
      title,
      errata_type,
      severity,
      issued,
      reboot_suggested,
      cve_list
    from digital_erratas 
    order by issued desc".into()
  }
  //fn refresh_mvs() -> Option<Vec<&'a str>> { Some(vec!["digital_erratas"]) }
  fn refresh_mvs() -> Option<Vec<MatView<'a>>> { Some(vec!["digital_erratas".into()]) }
}

