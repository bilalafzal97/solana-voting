use anchor_lang::prelude::*;

#[event]
pub struct SetVotingEnableEvent {
    pub timestamp: i64,

    pub topic: String,

    pub enable: bool,
}