use anchor_lang::prelude::*;

use crate::state::tweet::{Tweet, TOPIC_LENGTH, CONTENT_LENGTH};
use crate::errors::TwitterError;


pub fn initialize_tweet(ctx: Context<CreateTweet>, topic: String, content: String) -> Result<()> {
    require!(topic.as_bytes().len() <= TOPIC_LENGTH, TwitterError::TopicTooLong);
    require!(content.as_bytes().len() <= CONTENT_LENGTH, TwitterError::ContentTooLong);

    let mut topic_bytes = [u8; TOPIC_LENGTH];
    let mut content_bytes = [u8; CONTENT_LENGTH];

    topic_bytes[..topic.len()].copy_from_slice(topic);
    content_bytes[..content.len()].copy_from_slice(content);
    
    let tweet = &mut ctx.accounts.tweet;
    tweet.author = *ctx.accounts.user.key;
    tweet.topic = topic_bytes;
    tweet.content = content_bytes;
    Ok(())
}

#[derive(Accounts)]
#[instruction(topic: String)]
pub struct CreateTweet<'info> {
  #[account(
    init,
    payer = user,
    seeds = [topic.as_bytes(), user.key().as_ref()],
    bump,
    space = 8 + Tweet::LEN,
  )]
  pub tweet: Account<'info, Tweet>,

  #[account(mut)]
  pub user: Signer<'info>,

  pub system_program: Program<'info, System>
}
