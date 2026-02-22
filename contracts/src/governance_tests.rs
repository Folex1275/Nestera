#[cfg(test)]
mod governance_tests {
    use crate::{NesteraContract, NesteraContractClient, PlanType};
    use crate::rewards::storage_types::RewardsConfig;
    use soroban_sdk::{
        testutils::Address as _,
        Address, BytesN, Env,
    };

    fn setup_contract() -> (Env, NesteraContractClient<'static>, Address) {
        let env = Env::default();
        let contract_id = env.register(NesteraContract, ());
        let client = NesteraContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let admin_pk = BytesN::from_array(&env, &[1u8; 32]);

        env.mock_all_auths();
        client.initialize(&admin, &admin_pk);

        let config = RewardsConfig {
            points_per_token: 10,
            streak_bonus_bps: 0,
            long_lock_bonus_bps: 0,
            goal_completion_bonus: 0,
            enabled: true,
            min_deposit_for_rewards: 0,
            action_cooldown_seconds: 0,
            max_daily_points: 1_000_000,
            max_streak_multiplier: 10_000,
        };
        let _ = client.initialize_rewards_config(&config);

        (env, client, admin)
    }

    #[test]
    fn test_voting_power_zero_for_new_user() {
        let (env, client, _) = setup_contract();
        let user = Address::generate(&env);
        
        let power = client.get_voting_power(&user);
        assert_eq!(power, 0);
    }

    #[test]
    fn test_voting_power_increases_with_deposits() {
        let (env, client, _) = setup_contract();
        let user = Address::generate(&env);
        env.mock_all_auths();

        client.initialize_user(&user);
        let _ = client.create_savings_plan(&user, &PlanType::Flexi, &1000);

        let power = client.get_voting_power(&user);
        assert_eq!(power, 1000);
    }

    #[test]
    fn test_voting_power_accumulates_across_deposits() {
        let (env, client, _) = setup_contract();
        let user = Address::generate(&env);
        env.mock_all_auths();

        client.initialize_user(&user);
        let _ = client.create_savings_plan(&user, &PlanType::Flexi, &1000);
        let _ = client.create_savings_plan(&user, &PlanType::Flexi, &500);

        let power = client.get_voting_power(&user);
        assert_eq!(power, 1500);
    }

    #[test]
    fn test_cast_vote_requires_voting_power() {
        let (env, client, _) = setup_contract();
        let user = Address::generate(&env);
        env.mock_all_auths();

        client.initialize_user(&user);
        
        let result = client.try_cast_vote(&user, &1, &true);
        assert!(result.is_err());
    }

    #[test]
    fn test_cast_vote_succeeds_with_voting_power() {
        let (env, client, _) = setup_contract();
        let user = Address::generate(&env);
        env.mock_all_auths();

        client.initialize_user(&user);
        let _ = client.create_savings_plan(&user, &PlanType::Flexi, &1000);

        let result = client.try_cast_vote(&user, &1, &true);
        assert!(result.is_ok());
    }
}
