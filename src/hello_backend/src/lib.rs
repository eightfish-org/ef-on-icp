use ic_cdk::caller;
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;

const AUTHORIZED_PRINCIPAL: &str = "a33qp-z47xc-7tlpb-zvnd5-ykidh-tf4uk-xcbtp-2jucw-phkiq-hnclv-5ae";

thread_local! {
    static PROTOCOL_HEARTBEAT: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    static PROTOCOLS: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[update]
fn record_heartbeat(protocol: String, time: String) {
    let caller = caller();
    assert_eq!(
        caller.to_text(),
        AUTHORIZED_PRINCIPAL,
        "Caller is not authorized to record heartbeats"
    );
    PROTOCOL_HEARTBEAT.with(|store| {
        store.borrow_mut().insert(protocol, time);
    });
}

#[query]
fn get_last_heartbeat(protocol: String) -> Option<String> {
    PROTOCOL_HEARTBEAT.with(|store| store.borrow().get(&protocol).cloned())
}

#[query]
fn get_all_heartbeats() -> Vec<(String, String)> {
    PROTOCOL_HEARTBEAT.with(|store| store.borrow().iter().map(|(k, v)| (k.clone(), v.clone())).collect())
}

#[update]
fn register_protocol(protocol: String) {
    let caller = caller();
    assert_eq!(
        caller.to_text(),
        AUTHORIZED_PRINCIPAL,
        "Caller is not authorized to register protocols"
    );
    PROTOCOLS.with(|protocols| {
        let mut protocols = protocols.borrow_mut();
        if !protocols.contains(&protocol) {
            protocols.push(protocol);
        }
    });
}

#[query]
fn get_registered_protocols() -> Vec<String> {
    PROTOCOLS.with(|protocols| protocols.borrow().clone())
}

// Explicitly generate Candid interface
ic_cdk::export_candid!();