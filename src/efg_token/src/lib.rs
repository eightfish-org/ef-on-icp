use candid::{CandidType, Deserialize, Principal};
use ic_cdk::export::{candid};
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize)]
struct Token {
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: u128,
    balances: HashMap<Principal, u128>,
    owner: Principal,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            name: String::new(),
            symbol: String::new(),
            decimals: 0,
            total_supply: 0,
            balances: HashMap::new(),
            owner: Principal::anonymous(),
        }
    }
}

thread_local! {
    static TOKEN: RefCell<Token> = RefCell::new(Token::default());
}

#[init]
fn init() {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        let total_supply = 1_000_000_000 * 10u128.pow(18);
        token.name = "EFG Token".to_string();
        token.symbol = "EFG".to_string();
        token.decimals = 18;
        token.total_supply = total_supply; // 1 billion tokens with 18 decimals
        token.owner = ic_cdk::caller();
        token.balances.insert(ic_cdk::caller(), total_supply);
    });
}

#[query]
fn name() -> String {
    TOKEN.with(|token| token.borrow().name.clone())
}

#[query]
fn symbol() -> String {
    TOKEN.with(|token| token.borrow().symbol.clone())
}

#[query]
fn decimals() -> u8 {
    TOKEN.with(|token| token.borrow().decimals)
}

#[query]
fn total_supply() -> u128 {
    TOKEN.with(|token| token.borrow().total_supply)
}

#[query]
fn balance_of(account: Principal) -> u128 {
    TOKEN.with(|token| {
        token.borrow().balances.get(&account).cloned().unwrap_or(0)
    })
}

#[update]
fn transfer(to: Principal, amount: u128) -> bool {
    let from = ic_cdk::caller();
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        let from_balance = token.balances.get(&from).cloned().unwrap_or(0);
        if from_balance < amount {
            return false;
        }
        let to_balance = token.balances.get(&to).cloned().unwrap_or(0);
        token.balances.insert(from, from_balance - amount);
        token.balances.insert(to, to_balance + amount);
        true
    })
}

#[update]
fn mint(to: Principal, amount: u128) -> bool {
    let caller = ic_cdk::caller();
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        if caller != token.owner {
            return false; // Only the owner can mint
        }
        token.total_supply += amount;
        let to_balance = token.balances.get(&to).cloned().unwrap_or(0);
        token.balances.insert(to, to_balance + amount);
        true
    })
}

#[query]
fn get_owner() -> Principal {
    TOKEN.with(|token| token.borrow().owner)
}

// Candid export
//ic_cdk::export_candid!();