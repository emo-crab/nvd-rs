use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::{IntoParams, ToSchema};
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Paging {
  // 分页每页
  pub size: i64,
  // 分页偏移
  pub page: i64,
  // 结果总数
  pub total: i64,
}
#[cfg_attr(feature = "openapi", derive(IntoParams, ToSchema))]
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct QueryPaging {
  // 分页每页
  pub size: Option<i64>,
  // 分页偏移
  pub page: Option<i64>,
}
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct ListResponse<T> {
  pub result: Vec<T>,
  #[serde(flatten)]
  pub paging: Paging,
}

impl<T> ListResponse<T> {
  pub fn new(result: Vec<T>, total: i64, page: i64, size: i64) -> Self {
    Self {
      result,
      paging: Paging { size, page, total },
    }
  }
  pub fn results(&self) -> &[T] {
    &self.result
  }
}
