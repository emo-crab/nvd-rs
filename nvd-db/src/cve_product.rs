use crate::error::{NVDDBError, Result};
use crate::schema::cve_product;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};

use crate::models::{CveProduct, Product};
use crate::product::QueryProductByVendorName;
#[derive(Insertable)]
#[diesel(table_name = cve_product)]
pub struct CreateCveProduct {
  pub cve_id: String,
  pub product_id: Vec<u8>,
}
#[derive(Debug)]
pub struct CreateCveProductByName {
  pub cve_id: String,
  pub vendor: String,
  pub product: String,
}
#[derive(Debug)]
pub struct QueryCveProduct {
  pub cve_id: Option<String>,
  pub vendor: Option<String>,
  pub product: Option<String>,
  pub limit: i64,
  pub offset: i64,
}
impl CveProduct {
  // 创建CVE和产品关联
  pub fn create(conn: &mut MysqlConnection, args: &CreateCveProduct) -> Result<Self> {
    if let Err(err) = diesel::insert_into(cve_product::table)
      .values(args)
      .execute(conn)
    {
      // 重复了，说明已经存在弱点
      match err {
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {}
        _ => {
          return Err(NVDDBError::DieselError { source: err });
        }
      }
    }
    Ok(
      // mysql 不支持 get_result，要再查一次得到插入结果
      cve_product::dsl::cve_product
        .filter(cve_product::cve_id.eq(&args.cve_id))
        .filter(cve_product::product_id.eq(&args.product_id))
        .first::<CveProduct>(conn)?,
    )
  }
  // 创建CVE和产品关联从名称
  pub fn create_by_name(conn: &mut MysqlConnection, args: &CreateCveProductByName) -> Result<Self> {
    let vp = QueryProductByVendorName {
      vendor_name: args.vendor.clone(),
      name: args.product.clone(),
    };
    let p = Product::query_by_name(conn, &vp)?;
    Self::create(
      conn,
      &CreateCveProduct {
        cve_id: args.cve_id.clone(),
        product_id: p.id,
      },
    )
  }

  pub fn query(_conn:&mut MysqlConnection,_args:&QueryCveProduct) {

  }
}
