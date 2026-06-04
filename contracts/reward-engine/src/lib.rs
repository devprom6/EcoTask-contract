#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String};

#[contract]
pub struct RewardEngine;

#[contractimpl]
impl RewardEngine {
    pub fn initialize(_e: Env, _admin: Address, _token: Address, _registry: Address, _oracle: Address) {}

    pub fn submit_proof(_e: Env, _oracle: Address, _user: Address, _task_id: u64, _proof_cid: String) {}

    pub fn approve_proof(_e: Env, _oracle: Address, _user: Address, _task_id: u64) {}

    pub fn reject_proof(_e: Env, _oracle: Address, _user: Address, _task_id: u64) {}

    pub fn get_verification(_e: Env, _task_id: u64, _user: Address) {
        panic!("not implemented")
    }
}
