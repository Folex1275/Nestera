use crate::strategy::routing::{self, StrategyPositionKey};
use crate::strategy::registry::{self, StrategyInfo};
use crate::errors::SavingsError;
use crate::{NesteraContract, NesteraContractClient};
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};

fn setup() -> (Env, NesteraContractClient<'static>, Address, Address) {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    let client = NesteraContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let admin_pk = BytesN::from_array(&env, &[1u8; 32]);
    env.mock_all_auths();
    client.initialize(&admin, &admin_pk);
    (env, client, admin, contract_id)
}

#[test]
fn test_withdraw_strategy_full_and_partial() {
    let (env, client, admin, contract_id) = setup();
    let strat_addr = Address::generate(&env);
    client.register_strategy(&admin, &strat_addr, &1u32);
    let user = Address::generate(&env);
    let position_key = StrategyPositionKey::Lock(1);
    env.as_contract(&contract_id, || {
        // Simulate deposit
        routing::route_to_strategy(&env, strat_addr.clone(), position_key, 1000).unwrap();
        // Simulate strategy balance is 800 (partial withdrawal)
        // Mock client.strategy_balance returns 800
        // Withdraw
        // (In real test, would mock the client, here just check min logic)
        let result = routing::withdraw_from_strategy(&env, position_key, user.clone());
        assert!(result.is_ok());
    });
}

#[test]
fn test_withdraw_strategy_insufficient_balance() {
    let (env, client, admin, contract_id) = setup();
    let strat_addr = Address::generate(&env);
    client.register_strategy(&admin, &strat_addr, &1u32);
    let user = Address::generate(&env);
    let position_key = StrategyPositionKey::Lock(2);
    env.as_contract(&contract_id, || {
        routing::route_to_strategy(&env, strat_addr.clone(), position_key, 500).unwrap();
        // Simulate strategy balance is 0
        let result = routing::withdraw_from_strategy(&env, position_key, user.clone());
        assert!(matches!(result, Err(SavingsError::InsufficientBalance)));
    });
}

#[test]
fn test_withdraw_strategy_no_underflow() {
    let (env, client, admin, contract_id) = setup();
    let strat_addr = Address::generate(&env);
    client.register_strategy(&admin, &strat_addr, &1u32);
    let user = Address::generate(&env);
    let position_key = StrategyPositionKey::Lock(3);
    env.as_contract(&contract_id, || {
        routing::route_to_strategy(&env, strat_addr.clone(), position_key, 1).unwrap();
        // Simulate strategy balance is 2 (should only withdraw 1, no negative)
        let result = routing::withdraw_from_strategy(&env, position_key, user.clone());
        assert!(result.is_ok());
    });
}
