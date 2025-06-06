use candid::Principal;
use ic_cdk::{caller, query, update};

use crate::{INTENTS};

#[update]
fn store_intent(address :String, amount : u64){
    let principal: Principal = caller();
    INTENTS.with(|intents| {
        intents.borrow_mut().insert(principal, (address,amount))
    });
}

#[query]
fn get_user_intent() -> Option<(String, u64)>{
    let principal = caller();
    INTENTS.with(|intents| intents.borrow().get(&principal).cloned())
}