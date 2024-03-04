use anchor_lang::prelude::*;

#[error_code]
pub enum Voting {
    #[msg("Invalid Option Len")]
    InvalidOptionLen,

    #[msg("Invalid Option")]
    InvalidOption,

    #[msg("Option Already Exist")]
    OptionAlreadyExist,

    #[msg("Invalid Owner")]
    InvalidOwner,

    #[msg("Voting Is Disable")]
    VotingIsDisable,
}