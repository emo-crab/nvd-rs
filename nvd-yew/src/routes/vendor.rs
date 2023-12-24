use crate::component::{CPEQuery, CPEQueryProps, Pagination, PaginationProps};
use crate::console_log;
use crate::modules::cpe::{QueryCpe, Vendor};
use crate::modules::ListResponse;
use crate::routes::Route;
use crate::services::cpe::vendor_list;
use crate::services::FetchState;
use std::str::FromStr;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlButtonElement};
use yew::prelude::*;
use yew_router::prelude::*;

pub type VendorInfoList = ListResponse<Vendor, QueryCpe>;
pub enum Msg {
  SetFetchState(FetchState<VendorInfoList>),
  Send,
  Page(PageMsg),
  Query(QueryMsg),
}
pub enum PageMsg {
  Next,
  Prev,
  To(i64),
}
pub enum QueryMsg {
  Query(QueryCpe),
  Part(String),
  Name(String),
  Official(bool),
}
impl Component for VendorInfoList {
  type Message = Msg;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    let query = ctx.link().location().unwrap().query::<QueryCpe>().unwrap();
    VendorInfoList {
      query,
      ..VendorInfoList::default()
    }
  }
  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::SetFetchState(FetchState::Success(cil)) => {
        self.query.page = Some(cil.paging.page);
        self.query.size = Some(cil.paging.size);
        self.paging = cil.paging;
        self.result = cil.result;
        return true;
      }
      Msg::SetFetchState(FetchState::Failed(err)) => {
        console_log!("{}", err);
      }
      Msg::Send => {
        let q = self.query.clone();
        ctx.link().send_future(async {
          match vendor_list(q).await {
            Ok(data) => Msg::SetFetchState(FetchState::Success(data)),
            Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
          }
        });
      }
      Msg::Page(page) => {
        match page {
          PageMsg::Next => {
            self.query.page = Some(self.query.page.unwrap_or(0) + 1);
          }
          PageMsg::Prev => {
            self.query.page = Some(self.query.page.unwrap_or(0) - 1);
          }
          PageMsg::To(page) => {
            self.query.page = Some(page - 1);
          }
        }
        ctx
          .link()
          .navigator()
          .unwrap()
          .push_with_query(&Route::Cpe, &self.query)
          .unwrap();
        ctx.link().send_message(Msg::Send);
      }
      Msg::Query(query) => {
        match query {
          QueryMsg::Name(name) => {
            self.query.name = Some(name);
          }
          QueryMsg::Official(official) => {
            self.query.official = Some(official);
          }
          QueryMsg::Part(part) => {
            self.query.part = Some(part);
          }
          QueryMsg::Query(query) => {
            self.query = query;
          }
        }
        ctx
          .link()
          .navigator()
          .unwrap()
          .push_with_query(&Route::CveList, &self.query)
          .unwrap();
        ctx.link().send_message(Msg::Send);
      }
    }
    false
  }
  fn view(&self, ctx: &Context<Self>) -> Html {
    // let set_vendor = ctx.link().callback(|event: MouseEvent| {
    //   let target = event.target_unchecked_into::<HtmlButtonElement>();
    //   Msg::Query(QueryMsg::Official(target.get_attribute("value").unwrap()))
    // });
    // let set_product = ctx.link().callback(|event: MouseEvent| {
    //   let target = event.target_unchecked_into::<HtmlButtonElement>();
    //   let vendor = target.get_attribute("vendor").unwrap();
    //   let product = target.get_attribute("product").unwrap();
    //   Msg::Query(QueryMsg::Product(vendor, product))
    // });
    html! {
      <div class="card">
        {self.query(ctx)}
        <div class="table-responsive">
          <table class="table card-table table-vcenter table-striped table-sm table-hover">
            <thead class="sticky-top">
              <tr>
                <th scope="col">{"Name"}</th>
                <th scope="col">{"Description"}</th>
                <th scope="col">{"Products"}</th>
                <th scope="col">{"Updated"}</th>
              </tr>
            </thead>
            <tbody>
            {
              self.result.iter().map(|item| {
              html!{
                <tr class="table-group-divider">
                <th scope="row">{item.name.clone()}</th>
                <th scope="row">{item.description.clone()}</th>
                <th scope="row">{item.description.clone()}</th>
                <th scope="row">{item.updated_at.to_string()}</th>
                </tr>
                }
                }).collect::<Html>()
              }
            </tbody>
          </table>
        </div>
      {self.pagination(ctx)}
      </div>
    }
  }
  fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
    if first_render {
      ctx.link().send_message(Msg::Send);
    }
  }
}

impl VendorInfoList {
  fn pagination(&self, ctx: &Context<Self>) -> Html {
    let paging = self.paging.clone();
    let next_page = ctx.link().callback(|_| Msg::Page(PageMsg::Next));
    let prev_page = ctx.link().callback(|_| Msg::Page(PageMsg::Prev));
    let to_page = ctx.link().callback(|event: MouseEvent| {
      let target: EventTarget = event.target().unwrap();
      let page: i64 = i64::from_str(&target.clone().unchecked_into::<HtmlButtonElement>().value())
        .unwrap_or_default();
      Msg::Page(PageMsg::To(page))
    });
    let p = PaginationProps {
      paging,
      next_page,
      prev_page,
      to_page,
    };
    html! {<Pagination ..p.clone()/>}
  }
  fn query(&self, ctx: &Context<Self>) -> Html {
    let query_part = ctx.link().callback(|e: MouseEvent| {
      let target: EventTarget = e.target().unwrap();
      let part: String = target.clone().unchecked_into::<HtmlButtonElement>().value();
      Msg::Query(QueryMsg::Part(part))
    });
    let query = ctx
      .link()
      .callback(|args: QueryCpe| Msg::Query(QueryMsg::Query(args)));
    let p = CPEQueryProps {
      props: self.query.clone(),
      is_product: None,
      query_part,
      query,
    };
    html! {
      <CPEQuery ..p.clone()/>
    }
  }
}
