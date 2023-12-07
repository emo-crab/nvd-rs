use crate::modules::cwe_db::QueryCwe;
use crate::modules::Cwe;
use crate::{ApiResponse, Pool};
use actix_web::{get, web, Error, HttpResponse};
use std::ops::DerefMut;

#[get("/{id}")]
async fn api_cwe_id(id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
  let contact = web::block(move || {
    let mut conn = pool.get()?;
    Cwe::query_by_id(conn.deref_mut(), &id)
  })
  .await??;
  Ok(HttpResponse::Ok().json(contact))
}

#[get("")]
async fn api_cwe_list(args: web::Query<QueryCwe>, pool: web::Data<Pool>) -> ApiResponse {
  let contact = web::block(move || {
    let mut conn = pool.get()?;
    Cwe::query(conn.deref_mut(), &args)
  })
  .await??;
  Ok(HttpResponse::Ok().json(contact))
}