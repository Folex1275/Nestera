use crate::errors::SavingsError;
use crate::rewards::storage::get_user_rewards;
use soroban_sdk::{Address, Env};

/// Calculates voting power for a user based on their lifetime deposited funds
pub fn get_voting_power(env: &Env, user: &Address) -> u128 {
    let rewards = get_user_rewards(env, user.clone());
    rewards.lifetime_deposited.max(0) as u128
}

/// Casts a weighted vote (placeholder for future implementation)
pub fn cast_vote(
    env: &Env,
    user: Address,
    proposal_id: u64,
    support: bool,
) -> Result<(), SavingsError> {
    user.require_auth();
    let weight = get_voting_power(env, &user);
    
    if weight == 0 {
        return Err(SavingsError::InsufficientBalance);
    }
    
    // TODO: Store vote with weight
    env.events().publish(
        (soroban_sdk::symbol_short!("vote"), user, proposal_id),
        (support, weight),
    );
    
    Ok(())
}
