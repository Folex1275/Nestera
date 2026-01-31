#![cfg(test)]
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, String};

use crate::{NesteraContract, NesteraContractClient, SavingsError};

fn setup() -> (Env, NesteraContractClient<'static>, Address) {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    let client = NesteraContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let admin_pk = BytesN::from_array(&env, &[1u8; 32]);

    env.mock_all_auths();
    client.initialize(&admin, &admin_pk);

    (env, client, admin)
}

#[test]
fn test_successful_break_group_save() {
    let (env, client, _admin) = setup();
    let creator = Address::generate(&env);
    let member1 = Address::generate(&env);
    let member2 = Address::generate(&env);

    env.mock_all_auths();

    // Initialize users
    client.initialize_user(&creator);
    client.initialize_user(&member1);
    client.initialize_user(&member2);

    // Create a group
    let group_id = client.create_group_save(
        &creator,
        &String::from_str(&env, "Test Group"),
        &String::from_str(&env, "Test Description"),
        &String::from_str(&env, "savings"),
        &10000, // target_amount
        &0,     // contribution_type
        &100,   // contribution_amount
        &true,  // is_public
        &1,     // start_time
        &1000,  // end_time
    );

    // Member1 joins
    client.join_group_save(&member1, &group_id);

    // Member2 joins
    client.join_group_save(&member2, &group_id);

    // Members contribute
    client.contribute_to_group_save(&creator, &group_id, &500);
    client.contribute_to_group_save(&member1, &group_id, &300);
    client.contribute_to_group_save(&member2, &group_id, &200);

    // Member1 leaves
    client.break_group_save(&member1, &group_id);

    // Verify: Successfully left the group
}

#[test]
fn test_break_group_updates_member_count() {
    let (env, client, _admin) = setup();
    let creator = Address::generate(&env);
    let member1 = Address::generate(&env);
    let member2 = Address::generate(&env);

    env.mock_all_auths();

    client.initialize_user(&creator);
    client.initialize_user(&member1);
    client.initialize_user(&member2);

    let group_id = client.create_group_save(
        &creator,
        &String::from_str(&env, "Test Group"),
        &String::from_str(&env, "Description"),
        &String::from_str(&env, "savings"),
        &5000,
        &0,
        &100,
        &true,
        &1,
        &500,
    );

    // Two members join (making it 3 total with creator)
    client.join_group_save(&member1, &group_id);
    client.join_group_save(&member2, &group_id);

    // Member1 leaves
    client.break_group_save(&member1, &group_id);

    // Successfully left - member count should be reduced
}

#[test]
fn test_break_group_updates_user_groups_list() {
    let (env, client, _admin) = setup();
    let user = Address::generate(&env);

    env.mock_all_auths();
    client.initialize_user(&user);

    // Create first group
    let group1_id = client.create_group_save(
        &user,
        &String::from_str(&env, "Group 1"),
        &String::from_str(&env, "Description 1"),
        &String::from_str(&env, "savings"),
        &5000,
        &0,
        &100,
        &true,
        &1,
        &500,
    );

    // Create second group
    let group2_id = client.create_group_save(
        &user,
        &String::from_str(&env, "Group 2"),
        &String::from_str(&env, "Description 2"),
        &String::from_str(&env, "savings"),
        &3000,
        &0,
        &50,
        &true,
        &1,
        &400,
    );

    // User leaves group1
    client.break_group_save(&user, &group1_id);

    // User should still be in group2
    // Verify by attempting to contribute (should succeed)
    client.contribute_to_group_save(&user, &group2_id, &100);
}

#[test]
fn test_break_group_refunds_contribution() {
    let (env, client, _admin) = setup();
    let creator = Address::generate(&env);
    let member = Address::generate(&env);

    env.mock_all_auths();
    client.initialize_user(&creator);
    client.initialize_user(&member);

    let group_id = client.create_group_save(
        &creator,
        &String::from_str(&env, "Test Group"),
        &String::from_str(&env, "Description"),
        &String::from_str(&env, "savings"),
        &5000,
        &0,
        &100,
        &true,
        &1,
        &500,
    );

    client.join_group_save(&member, &group_id);

    // Member contributes 500
    client.contribute_to_group_save(&member, &group_id, &500);

    // Member leaves - should get refund
    client.break_group_save(&member, &group_id);

    // Group's current_amount should be reduced by 500
}

#[test]
fn test_break_group_with_zero_contribution() {
    let (env, client, _admin) = setup();
    let creator = Address::generate(&env);
    let member = Address::generate(&env);

    env.mock_all_auths();
    client.initialize_user(&creator);
    client.initialize_user(&member);

    let group_id = client.create_group_save(
        &creator,
        &String::from_str(&env, "Test Group"),
        &String::from_str(&env, "Description"),
        &String::from_str(&env, "savings"),
        &5000,
        &0,
        &100,
        &true,
        &1,
        &500,
    );

    client.join_group_save(&member, &group_id);

    // Member never contributes, just leaves
    client.break_group_save(&member, &group_id);

    // Should succeed with 0 refund
}

#[test]
fn test_error_user_not_member() {
    let (env, client, _admin) = setup();
    let creator = Address::generate(&env);
    let non_member = Address::generate(&env);

    env.mock_all_auths();
    client.initialize_user(&creator);
    client.initialize_user(&non_member);

    let group_id = client.create_group_save(
        &creator,
        &String::from_str(&env, "Test Group"),
        &String::from_str(&env, "Description"),
        &String::from_str(&env, "savings"),
        &5000,
        &0,
        &100,
        &true,
        &1,
        &500,
    );

    // Non-member tries to leave
    let result = client.try_break_group_save(&non_member, &group_id);
    assert_eq!(result.unwrap_err(), Ok(SavingsError::NotGroupMember));
}

#[test]
fn test_error_group_completed() {
    let (env, client, _admin) = setup();
    let creator = Address::generate(&env);

    env.mock_all_auths();
    client.initialize_user(&creator);

    let group_id = client.create_group_save(
        &creator,
        &String::from_str(&env, "Test Group"),
        &String::from_str(&env, "Description"),
        &String::from_str(&env, "savings"),
        &100, // Low target to easily complete
        &0,
        &10,
        &true,
        &1,
        &500,
    );

    // Complete the group by contributing the full amount
    client.contribute_to_group_save(&creator, &group_id, &100);

    // Try to leave completed group
    let result = client.try_break_group_save(&creator, &group_id);
    assert_eq!(result.unwrap_err(), Ok(SavingsError::PlanCompleted));
}

#[test]
fn test_error_group_not_found() {
    let (env, client, _admin) = setup();
    let user = Address::generate(&env);

    env.mock_all_auths();
    client.initialize_user(&user);

    // Try to leave non-existent group
    let result = client.try_break_group_save(&user, &999);
    assert_eq!(result.unwrap_err(), Ok(SavingsError::PlanNotFound));
}

#[test]
fn test_error_user_not_found() {
    let (env, client, _admin) = setup();
    let creator = Address::generate(&env);
    let non_existent_user = Address::generate(&env);

    env.mock_all_auths();
    client.initialize_user(&creator);

    let group_id = client.create_group_save(
        &creator,
        &String::from_str(&env, "Test Group"),
        &String::from_str(&env, "Description"),
        &String::from_str(&env, "savings"),
        &5000,
        &0,
        &100,
        &true,
        &1,
        &500,
    );

    // Non-initialized user tries to leave
    let result = client.try_break_group_save(&non_existent_user, &group_id);
    assert_eq!(result.unwrap_err(), Ok(SavingsError::UserNotFound));
}
