use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Comment {
    pub tweet: Pubkey,
    pub author: Pubkey,
    #[max_len(500)]
    pub content: String,
}