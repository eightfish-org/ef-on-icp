use ic_cdk::caller;
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;
const AUTHORIZED_PRINCIPAL: &str = "a33qp-z47xc-7tlpb-zvnd5-ykidh-tf4uk-xcbtp-2jucw-phkiq-hnclv-5ae";
thread_local! {
    static STORE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

#[update]
fn record_heartbeat(subject: String, time: String) {
    let caller = caller();
    assert_eq!(
        caller.to_text(),
        AUTHORIZED_PRINCIPAL,
        "Caller is not authorized to record heartbeats"
    );
    STORE.with(|store| {
        store.borrow_mut().insert(subject, time);
    });
}

#[query]
fn get_last_heartbeat(subject: String) -> Option<String> {
    STORE.with(|store| store.borrow().get(&subject).cloned())
}

#[query]
fn get_all_heartbeats() -> Vec<(String, String)> {
    STORE.with(|store| store.borrow().iter().map(|(k, v)| (k.clone(), v.clone())).collect())
}

// Explicitly generate Candid interface
ic_cdk::export_candid!();