use anchor_lang::prelude::*;

#[event]
pub struct InitializeVotingTopicEvent {
    pub timestamp: i64,

    pub topic: String,

    pub owner: Pubkey,
}