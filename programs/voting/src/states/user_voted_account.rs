use anchor_lang::prelude::*;

pub const USER_VOTED_ACCOUNT_PREFIX: &str = "USER_VOTED";

#[account]
pub struct UserVotedConfigAccount {
    /// timestamp when account updated
    pub last_block_timestamp: i64,
    pub topic: String,
    pub option: String,
}

impl UserVotedConfigAccount {
    pub fn space() -> usize {
        8 // default
            + 8 // last_block_timestamp
            + 100 // topic
            + 100 // option
    }
}