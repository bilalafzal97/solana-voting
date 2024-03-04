use anchor_lang::prelude::*;
use crate::error::Voting;
use crate::events::CastVoteEvent;
use crate::states::{VOTING_CONFIG_ACCOUNT_PREFIX, VotingConfigAccount,
                    USER_VOTED_ACCOUNT_PREFIX, UserVotedConfigAccount,
};

#[derive(Accounts)]
#[instruction(_topic: String, _voting_config_bump: u8)]
pub struct CastVoteInputAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub user: Signer<'info>,

    #[account(
    mut,
    seeds = [
    VOTING_CONFIG_ACCOUNT_PREFIX.as_ref(),
    _topic.as_ref(),
    ],
    bump = _voting_config_bump,
    )]
    pub voting_config: Box<Account<'info, VotingConfigAccount>>,

    #[account(
    init,
    payer = payer,
    space = UserVotedConfigAccount::space(),
    seeds = [
    USER_VOTED_ACCOUNT_PREFIX.as_ref(),
    voting_config.key().as_ref(),
    user.key().as_ref(),
    ],
    bump,
    )]
    pub user_voted: Box<Account<'info, UserVotedConfigAccount>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_cast_vote(ctx: Context<CastVoteInputAccounts>,
                        topic: String, _voting_config_bump: u8, option: String) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let voting_config: &mut Box<Account<VotingConfigAccount>> = &mut ctx.accounts.voting_config;

    if !voting_config.enable {
        return Err(Voting::VotingIsDisable.into());
    }

    if !voting_config.voting_options.contains(&option) {
        return Err(Voting::InvalidOption.into());
    }

    let index = voting_config.voting_options.iter().position(|r| r == &option).unwrap();
    voting_config.voting_options_vote_count[index] += 1;

    let user_voted: &mut Box<Account<UserVotedConfigAccount>> = &mut ctx.accounts.user_voted;

    user_voted.last_block_timestamp = timestamp;
    user_voted.topic = topic.clone();
    user_voted.option = option.clone();

    let event: CastVoteEvent = CastVoteEvent {
        timestamp,
        topic,
        option,
        option_vote_count: voting_config.voting_options_vote_count[index],
        user: ctx.accounts.user.key()
    };

    emit!(event);

    Ok(())
}