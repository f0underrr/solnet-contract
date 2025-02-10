use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("No Active Tree Found")]
    NoActiveTreeFound,

    #[msg("Skipped Tree")]
    SkippedTree,

    #[msg("Previous cell active")]
    PreviousCellActive,

    #[msg("Cell not active")]
    CellNotActive,

    #[msg("ZeroAmount")]
    ZeroAmount,

    #[msg("Not Owner")]
    NotOwner,

    #[msg("InvalidTreasury")]
    InvalidTreasury,

    #[msg("TreeNotActive")]
    TreeNotActive
}
// TODO: add other errors, change naming