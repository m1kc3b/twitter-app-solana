use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ReactionType {
    Like = 0,
    Dislike = 1,
}

#[account]
#[derive(InitSpace)]
pub struct Reaction {
    pub tweet: Pubkey,
    pub author: Pubkey,
}