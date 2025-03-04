use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

use crate::state::comment::*;
use crate::state::reaction::TWEET_REACTION_SEED;
use crate::state::tweet::*;
use crate::errors::TwitterError;

pub fn initialize_comment(ctx: Context<AddComment>, content: String) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;
    let tweet_comment = &mut ctx.accounts.tweet_comment;
    let author = ctx.accounts.user.key();

    require!(content.as_bytes().len() <= COMMENT_LENGTH, TwitterError::CommentTooLong);

    tweet_comment.tweet = tweet.key();
    tweet_comment.author = author;

    let mut comment_bytes = [0u8; COMMENT_LENGTH];
    comment_bytes[..content.len()].copy_from_slice(content.as_bytes());
    tweet_comment.content = comment_bytes;
    tweet_comment.content_length = content.as_bytes().len() as u8;
    tweet_comment.bump = ctx.bumps.tweet_comment;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(content: String)]
pub struct AddComment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Comment::LEN,
        seeds = [
            TWEET_REACTION_SEED.as_bytes(), 
            tweet.key().as_ref(), 
            user.key().as_ref(),
            hash(content.as_bytes()).to_bytes().as_ref(),
        ],
        bump,
    )]
    pub tweet_comment: Account<'info, Comment>,

    #[account(
        mut,
        seeds = [TWEET_SEED.as_bytes(), tweet.topic[..tweet.topic_length as usize].as_ref(), tweet.author.key().as_ref()],
        bump = tweet.bump,
    )]
    pub tweet: Account<'info, Tweet>,

    pub system_program: Program<'info, System>,
}
