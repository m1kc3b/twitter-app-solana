use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ReactionType {
    Like = 0,
    Dislike = 1,
}

impl anchor_lang::Space for ReactionType {
  const INIT_SPACE: usize = std::mem::size_of::<u8>();
}

#[account]
#[derive(InitSpace)]
pub struct Reaction {
    pub tweet: Pubkey,
    pub author: Pubkey,
    pub reaction: ReactionType,
}