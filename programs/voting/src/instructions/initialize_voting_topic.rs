use anchor_lang::prelude::*;
use crate::error::Voting;
use crate::events::InitializeVotingTopicEvent;
use crate::states::{VOTING_CONFIG_ACCOUNT_PREFIX, VotingConfigAccount};

#[derive(Accounts)]
#[instruction(len: u8, topic: String)]
pub struct InitializeVotingTopicInputAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub owner: Signer<'info>,

    #[account(
    init,
    payer = payer,
    space = VotingConfigAccount::space(len),
    seeds = [
    VOTING_CONFIG_ACCOUNT_PREFIX.as_ref(),
    topic.as_ref(),
    ],
    bump,
    )]
    pub voting_config: Box<Account<'info, VotingConfigAccount>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_initialize_voting_topic(ctx: Context<InitializeVotingTopicInputAccounts>,
                                      len: u8, topic: String, voting_options: Vec<String>) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    if len as usize != voting_options.len() {
        return Err(Voting::InvalidOptionLen.into());
    };

    let voting_config: &mut Box<Account<VotingConfigAccount>> = &mut ctx.accounts.voting_config;

    voting_config.last_block_timestamp = timestamp;
    voting_config.owner = ctx.accounts.owner.key();
    voting_config.topic = topic.clone();
    voting_config.enable = true;
    voting_config.voting_options = Vec::new();
    voting_config.voting_options_vote_count = Vec::new();

    for i in 0..voting_options.len() {
        if voting_config.voting_options.contains(&voting_options[i]) {
            return Err(Voting::OptionAlreadyExist.into());
        };

        let op: String = voting_options[i].clone();
        voting_config.voting_options.push(op);
        voting_config.voting_options_vote_count.push(0);
    };

    let event: InitializeVotingTopicEvent = InitializeVotingTopicEvent {
        timestamp,
        topic,
        owner: ctx.accounts.owner.owner.key(),
    };

    emit!(event);

    Ok(())
}