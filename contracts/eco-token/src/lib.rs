#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String};

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn initialize(_e: Env, _admin: Address, _name: String, _symbol: String, _decimal: u32) {}

    pub fn mint(_e: Env, _to: Address, _amount: i128) {}

    pub fn transfer(_e: Env, _from: Address, _to: Address, _amount: i128) {}

    pub fn balance(_e: Env, _id: Address) -> i128 {
        0
    }

    pub fn total_supply(_e: Env) -> i128 {
        0
    }

    pub fn name(_e: Env) -> String {
        String::from_str(&_e, "ECO")
    }

    pub fn symbol(_e: Env) -> String {
        String::from_str(&_e, "ECO")
    }

    pub fn decimal(_e: Env) -> u32 {
        7
    }

    pub fn admin(_e: Env) -> Address {
        panic!("not implemented")
    }
}
