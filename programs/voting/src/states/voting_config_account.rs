use anchor_lang::prelude::*;

pub const VOTING_CONFIG_ACCOUNT_PREFIX: &str = "VOTING_CONFIG";

#[account]
pub struct VotingConfigAccount {
    /// timestamp when account updated
    pub last_block_timestamp: i64,

    pub owner: Pubkey,

    pub topic: String,

    pub enable: bool,

    pub voting_options: Vec<String>,

    pub voting_options_vote_count: Vec<u64>,
}

impl VotingConfigAccount {
    pub fn space(len: u8) -> usize {
        8 // default
            + 8 // last_block_timestamp
            + 32 // owner
            + 100 // topic
            + 1 // enable
            + 4 + (len as usize * 50) // voting_options
            + 4 + (len as usize * 8) // voting_options_vote_count
    }
}