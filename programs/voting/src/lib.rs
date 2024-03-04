use anchor_lang::prelude::*;

use instructions::*;

mod instructions;
mod states;
mod events;
mod error;
mod utils;

declare_id!("51kTukB2F2XmA9WaKWWB7bLxdTA5VLF8b7urt5naKCtg");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_voting_topic(ctx: Context<InitializeVotingTopicInputAccounts>,
                                   len: u8, topic: String, voting_options: Vec<String>) -> Result<()> {
        handle_initialize_voting_topic(ctx, len, topic, voting_options)
    }

    pub fn set_voting_enable(ctx: Context<SetVotingEnableInputAccounts>,
                             topic: String, _voting_config_bump: u8, enable: bool) -> Result<()> {
        handle_set_voting_enable(ctx, topic, _voting_config_bump, enable)
    }

    pub fn cast_vote(ctx: Context<CastVoteInputAccounts>,
                            topic: String, _voting_config_bump: u8, option: String) -> Result<()> {
        handle_cast_vote(ctx, topic, _voting_config_bump, option)
    }
}
