use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static STORE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

#[update]
fn record_heartbeat(subject: String, time: String) {
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