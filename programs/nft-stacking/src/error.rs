use anchor_lang::prelude::*;

#[error_code]
pub enum StackError {
    #[msg("You have reached your maximum stack reach")]
    MaxStackReach,
}
