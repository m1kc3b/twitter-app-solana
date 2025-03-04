use anchor_lang::prelude::*;

use crate::state::tweet::{Tweet, TOPIC_LENGTH, CONTENT_LENGTH, TWEET_SEED};
use crate::errors::TwitterError;


pub fn initialize_tweet(ctx: Context<CreateTweet>, topic: String, content: String) -> Result<()> {
    require!(topic.as_bytes().len() <= TOPIC_LENGTH, TwitterError::TopicTooLong);
    require!(content.as_bytes().len() <= CONTENT_LENGTH, TwitterError::ContentTooLong);

    let tweet = &mut ctx.accounts.tweet;

    let mut topic_bytes = [0u8; TOPIC_LENGTH];
    topic_bytes[..topic.len()].copy_from_slice(topic.as_bytes());
    tweet.topic = topic_bytes;

    let mut content_bytes = [0u8; CONTENT_LENGTH];
    content_bytes[..content.len()].copy_from_slice(content.as_bytes());
    tweet.content = content_bytes;
    
    tweet.author = *ctx.accounts.user.key;
    tweet.topic_length = topic.as_bytes().len() as u8;
    tweet.dislikes = 0;
    tweet.likes = 0;
    tweet.bump = ctx.bumps.tweet;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(topic: String)]
pub struct CreateTweet<'info> {
  #[account(
    init,
    payer = user,
    seeds = [TWEET_SEED.as_bytes(), topic.as_bytes(), user.key().as_ref()],
    bump,
    space = 8 + Tweet::LEN,
  )]
  pub tweet: Account<'info, Tweet>,

  #[account(mut)]
  pub user: Signer<'info>,

  pub system_program: Program<'info, System>
}
