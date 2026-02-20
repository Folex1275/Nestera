pub mod config;
pub mod storage_types;

// Re-export the struct so it's easier to access as rewards::RewardsConfig
pub use storage_types::RewardsConfig;
