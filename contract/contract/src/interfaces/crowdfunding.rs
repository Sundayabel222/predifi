use soroban_sdk::{Address, BytesN, Env, String};

use crate::base::{
    errors::CrowdfundingError,
    types::{CampaignDetails, EmergencyWithdrawRequest, PoolConfig, PoolState},
};

pub trait CrowdfundingTrait {
    fn create_campaign(
        env: Env,
        id: BytesN<32>,
        title: String,
        creator: Address,
        goal: i128,
        deadline: u64,
    ) -> Result<(), CrowdfundingError>;

    fn get_campaign(env: Env, id: BytesN<32>) -> Result<CampaignDetails, CrowdfundingError>;

    fn save_pool(
        env: Env,
        name: String,
        description: String,
        creator: Address,
        target_amount: i128,
        deadline: u64,
    ) -> Result<u64, CrowdfundingError>;

    fn get_pool(env: Env, pool_id: u64) -> Option<PoolConfig>;

    fn update_pool_state(
        env: Env,
        pool_id: u64,
        new_state: PoolState,
    ) -> Result<(), CrowdfundingError>;

    fn initialize(env: Env, admin: Address) -> Result<(), CrowdfundingError>;

    fn pause(env: Env) -> Result<(), CrowdfundingError>;

    fn unpause(env: Env) -> Result<(), CrowdfundingError>;

    fn is_paused(env: Env) -> bool;

    fn contribute(
        env: Env,
        pool_id: u64,
        contributor: Address,
        asset: Address,
        amount: i128,
        is_private: bool,
    ) -> Result<(), CrowdfundingError>;

    fn request_emergency_withdraw(
        env: Env,
        pool_id: u64,
        creator: Address,
        asset: Address,
        amount: i128,
    ) -> Result<(), CrowdfundingError>;

    fn execute_emergency_withdraw(env: Env, pool_id: u64) -> Result<(), CrowdfundingError>;

    fn set_grace_period(env: Env, admin: Address, grace_period: u64)
        -> Result<(), CrowdfundingError>;

    fn get_grace_period(env: Env) -> u64;

    fn get_emergency_withdraw_request(
        env: Env,
        pool_id: u64,
    ) -> Option<EmergencyWithdrawRequest>;
}
