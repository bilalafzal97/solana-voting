use anchor_lang::prelude::*;
use crate::error::Voting;
use crate::events::SetVotingEnableEvent;
use crate::states::{VOTING_CONFIG_ACCOUNT_PREFIX, VotingConfigAccount};

#[derive(Accounts)]
#[instruction(topic: String, _voting_config_bump: u8)]
pub struct SetVotingEnableInputAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub owner: Signer<'info>,

    #[account(
    mut,
    seeds = [
    VOTING_CONFIG_ACCOUNT_PREFIX.as_ref(),
    topic.as_ref(),
    ],
    bump = _voting_config_bump,
    )]
    pub voting_config: Box<Account<'info, VotingConfigAccount>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_set_voting_enable(ctx: Context<SetVotingEnableInputAccounts>,
                                topic: String, _voting_config_bump: u8, enable: bool) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let voting_config: &mut Box<Account<VotingConfigAccount>> = &mut ctx.accounts.voting_config;

    if voting_config.owner.key() != ctx.accounts.owner.key() {
        return Err(Voting::InvalidOwner.into());
    };

    voting_config.last_block_timestamp = timestamp;
    voting_config.enable = enable;

    let event: SetVotingEnableEvent = SetVotingEnableEvent {
        timestamp,
        topic,
        enable,
    };

    emit!(event);

    Ok(())
}