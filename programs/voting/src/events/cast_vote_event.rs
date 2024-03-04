use anchor_lang::prelude::*;

#[event]
pub struct CastVoteEvent {
    pub timestamp: i64,

    pub topic: String,
    pub option: String,
    pub option_vote_count: u64,

    pub user: Pubkey,
}