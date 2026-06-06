use soroban_sdk::{contract, contractimpl, Address, Env, String, Symbol};
use crate::storage;

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn initialize(e: Env, admin: Address, name: String, symbol: String, decimal: u32) {
        if storage::has_admin(&e) {
            panic!("already initialized");
        }
        storage::write_admin(&e, &admin);
        storage::write_metadata(&e, &name, &symbol, &decimal);
        storage::write_supply(&e, 0);
    }

    pub fn mint(e: Env, to: Address, amount: i128) {
        let admin = storage::read_admin(&e);
        admin.require_auth();

        let balance = storage::read_balance(&e, &to);
        storage::write_balance(&e, &to, balance + amount);

        let supply = storage::read_supply(&e);
        storage::write_supply(&e, supply + amount);

        e.events().publish(
            (Symbol::new(&e, "mint"), admin, to.clone()),
            amount,
        );
    }

    pub fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let from_balance = storage::read_balance(&e, &from);
        if from_balance < amount {
            panic!("insufficient balance");
        }

        storage::write_balance(&e, &from, from_balance - amount);

        let to_balance = storage::read_balance(&e, &to);
        storage::write_balance(&e, &to, to_balance + amount);

        e.events().publish(
            (Symbol::new(&e, "transfer"), from.clone(), to.clone()),
            amount,
        );
    }

    pub fn balance(e: Env, id: Address) -> i128 {
        storage::read_balance(&e, &id)
    }

    pub fn total_supply(e: Env) -> i128 {
        storage::read_supply(&e)
    }

    pub fn name(e: Env) -> String {
        storage::read_name(&e)
    }

    pub fn symbol(e: Env) -> String {
        storage::read_symbol(&e)
    }

    pub fn decimal(e: Env) -> u32 {
        storage::read_decimal(&e)
    }

    pub fn admin(e: Env) -> Address {
        storage::read_admin(&e)
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{Env, Address, String};
    use soroban_sdk::testutils::Address as _;
    use crate::{TokenContract, TokenContractClient};

    #[test]
    fn test_initialize_and_metadata() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let contract_id = e.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&e, &contract_id);

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );

        assert_eq!(client.name(), String::from_str(&e, "ECO"));
        assert_eq!(client.symbol(), String::from_str(&e, "ECO"));
        assert_eq!(client.decimal(), 7);
        assert_eq!(client.total_supply(), 0);
    }

    #[test]
    fn test_mint_and_balance() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let user = Address::generate(&e);
        let contract_id = e.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&e, &contract_id);

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );

        e.mock_all_auths();
        client.mint(&user, &1000);

        assert_eq!(client.balance(&user), 1000);
        assert_eq!(client.total_supply(), 1000);
    }

    #[test]
    fn test_transfer() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let from = Address::generate(&e);
        let to = Address::generate(&e);
        let contract_id = e.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&e, &contract_id);

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );

        e.mock_all_auths();
        client.mint(&from, &500);
        client.transfer(&from, &to, &300);

        assert_eq!(client.balance(&from), 200);
        assert_eq!(client.balance(&to), 300);
    }

    #[test]
    #[should_panic(expected = "insufficient balance")]
    fn test_transfer_insufficient_balance() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let from = Address::generate(&e);
        let to = Address::generate(&e);
        let contract_id = e.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&e, &contract_id);

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );

        e.mock_all_auths();
        client.transfer(&from, &to, &100);
    }

    #[test]
    #[should_panic]
    fn test_mint_only_admin() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let user = Address::generate(&e);
        let contract_id = e.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&e, &contract_id);

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );

        client.mint(&user, &1000);
    }

    #[test]
    #[should_panic(expected = "already initialized")]
    fn test_double_initialize_fails() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let contract_id = e.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&e, &contract_id);

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );
    }
}


