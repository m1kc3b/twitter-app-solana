use anchor_lang::prelude::*;

use crate::state::tweet::Tweet;
use crate::errors::TwitterError;

const TOPIC_MAX_LENGHT: usize = 32;
const CONTENT_MAX_LENGHT: usize = 500;

pub fn initialize_tweet(ctx: Context<CreateTweet>, topic: String, content: String) -> Result<()> {
    require!(topic.as_bytes().len() <= TOPIC_MAX_LENGHT, TwitterError::TopicTooLong);
    require!(content.as_bytes().len() <= CONTENT_MAX_LENGHT, TwitterError::ContentTooLong);
    
    let tweet = &mut ctx.accounts.tweet;
    tweet.author = *ctx.accounts.user.key;
    tweet.topic = topic;
    tweet.content = content;
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
    space = 8 + Tweet::INIT_SPACE
  )]
  pub tweet: Account<'info, Tweet>,

  #[account(mut)]
  pub user: Signer<'info>,

  pub system_program: Program<'info, System>
}
