use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RewardsConfig {
    pub points_per_token: u32,      // e.g., 10 points per 1 token
    pub streak_bonus_bps: u32,      // Bonus for consistent saving
    pub long_lock_bonus_bps: u32,   // Bonus for long-term locks
    pub goal_completion_bonus: u32, // Flat points for finishing a goal
    pub enabled: bool,
}

#[contracttype]
pub enum RewardsDataKey {
    Config,
}
