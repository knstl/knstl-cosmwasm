use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Allowance is expired")]
    Expired {},

    #[error("No allowance for this account")]
    NoAllowance {},

    #[error("Minting cannot exceed the cap")]
    CannotExceedCap {},

    #[error("Logo binary data exceeds 5KB limit")]
    LogoTooBig {},

    #[error("Invalid xml preamble for SVG")]
    InvalidXmlPreamble {},

    #[error("Invalid png header")]
    InvalidPngHeader {},

    #[error("Duplicate initial balance addresses")]
    DuplicateInitialBalanceAddresses {},

    #[error("Unstake amount exceeds staked amount")]
    InvalidUnstakeAmount{},
    
    #[error("Cannot Stake to Multiple Validators")]
    InvalidMultipleValidator {},

    #[error("Sent Tokens {} are not Stake-Able", denom)]
    UnstakeableTokenSent {denom: String},

    #[error("Received Multiple Tokens")]
    InvalidMultipleTokens {},

    #[error("Still On Unbonding Period")]
    OnUnbondingPeriod {},

    #[error("Only Admin and Owner of this contract can execute")]
    UnknownUser {},

    #[error("Cannot compound when unbonded tokens exist, unbond first")]
    CompoundWithUnbondeds {},
}