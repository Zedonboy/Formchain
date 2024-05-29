use std::{borrow::Cow, cell::{Ref, RefCell}, collections::HashMap};

use candid::{candid_method, CandidType};
use canister_http_router::{extractors::extract_form_or_json_data, CallType, CanisterRouter, CanisterRouterContext, HttpRequest, HttpResponse};
use ic_cdk::{init, post_upgrade, print, query, update};
use ic_stable_structures::{memory_manager::{MemoryId, MemoryManager, VirtualMemory}, Cell, DefaultMemoryImpl};
use serde::Deserialize;
use serde_bytes::{ByteBuf, Bytes};
use types::{Form, FormChainError, FormEntry, Ledger};

mod types;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    // static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    // RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
   static ROUTER : RefCell<CanisterRouter> = RefCell::new(CanisterRouter::new());
//    static LEDGER : RefCell<Ledger> = RefCell::new(Ledger::new());
   static LEDGER : RefCell<Ledger> = RefCell::new(Ledger::new())
}

fn setup_routes() -> CanisterRouter {
    let mut router = CanisterRouter::new();
    router.post("/form/{form_id}",  |cntx: CanisterRouterContext| async move {
        
        match cntx.call_type {
            CallType::Query => {
                print("Upgrading Query");
                HttpResponse::upgrade()
            },
            CallType::Update => {
                print("Updating Entry");
                if cntx.params.is_none() {
                    return HttpResponse::bad_request(None);
                }

                let param = cntx.params.as_ref().unwrap();
                let form_id_opt = param.get("form_id");
                if form_id_opt.is_none() {
                    return HttpResponse::bad_request(None);
                }

                let form_id = form_id_opt.unwrap();
                // extractor.
                let data_result = extract_form_or_json_data(&cntx);
                if data_result.is_err() {
                    let err_str = data_result.unwrap_err();
                    return HttpResponse::bad_request(Some(format!("Content body seems invalid: {}", err_str).as_str()));
                }

                let rslt = LEDGER.with_borrow_mut(|ledger| {
                    ledger.add_entry(form_id.to_string(), data_result.unwrap())
                });

                if rslt.is_err() {
                    return HttpResponse::bad_request(Some(rslt.unwrap_err().to_string().as_str()))
                }

                HttpResponse::new()
            },
        }

    });

    router.get("/hello", |cntx : CanisterRouterContext| async move {
        let mut resp = HttpResponse::new();
        resp.status(200u16).set_body(ByteBuf::from("Formchain Canister version 1"));
        resp
    });

    router
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[init]
fn app_init() {
    let router = setup_routes();
    ROUTER.set(router);
}

#[post_upgrade]
fn app_postupgrade() {
    let router = setup_routes();
    ROUTER.set(router);
}


#[update]
#[candid_method(update)]
async fn create_form(form : Form) -> Result<String, FormChainError> {
    let (rnd_bytes, ) = ic_cdk::api::management_canister::main::raw_rand().await.unwrap();
    LEDGER.with_borrow_mut(|form_ledger| {
        form_ledger.create_form(form, rnd_bytes)
    })
}

#[update]
async fn http_request_update(req : HttpRequest) -> HttpResponse {
    let router = ROUTER.with_borrow(|r| r.clone());
    router.process(req, CallType::Update).await
}

#[query]
async fn http_request(req : HttpRequest) -> HttpResponse {
    let router = ROUTER.with_borrow(|r| r.clone());
    router.process(req, CallType::Query).await
}

#[query]
#[candid_method(query)]
fn export_candid() -> String {
    ic_cdk::export_candid!();
    __export_service()
}
