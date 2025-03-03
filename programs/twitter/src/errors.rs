
use anchor_lang::error_code;

#[error_code]
pub enum TwitterError {
    #[msg("The topic's lenght is too long!")]
    TopicTooLong,
    #[msg("The content's lenght is too long!")]
    ContentTooLong,
}