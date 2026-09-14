use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SoroQuestError {
    BountyNotFound = 1,
    BountyNotOpen = 2,
    BountyNotClaimed = 3,
    AlreadyClaimed = 4,
    NotOwner = 5,
    DeadlineNotPassed = 6,
    InvalidAmount = 7,
    InvalidDeadline = 8,
    TransferFailed = 9,
}
