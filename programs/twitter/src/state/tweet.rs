use anchor_lang::prelude::*;

pub const TOPIC_LENGTH: usize = 32;
pub const CONTENT_LENGTH: usize = 500;
pub const TWEET_SEED: &str = "TWEET_SEED";

#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub topic: [u8; TOPIC_LENGTH],
    pub topic_length: u8,
    pub content: [u8; CONTENT_LENGTH],
    pub likes: u64,
    pub dislikes: u64,
    pub bump: u8,
}

impl Tweet {
    pub const LEN: usize = 32 + TOPIC_LENGTH + CONTENT_LENGTH + 8 + 8;
}