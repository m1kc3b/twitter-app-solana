pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

declare_id!("7e957SngurXVSomdZ7Qhw3QeBB3FtZu9GEdpcMcBS5Mu");

#[program]
pub mod twitter {
    use super::*;

    pub fn create_tweet(ctx: Context<CreateTweet>) -> Result<()> {
        todo!();
        Ok(())
    }

    pub fn add_comment(ctx: Context<AddComment>) -> Result<()> {
        todo!();
        Ok(())
    }

    pub fn remove_comment(ctx: Context<RemoveComment>) -> Result<()> {
        todo!();
        Ok(())
    }

    pub fn add_reaction(ctx: Context<AddReaction>) -> Result<()> {
        todo!();
        Ok(())
    }

    pub fn remove_reaction(ctx: Context<RemoveReaction>) -> Result<()> {
        todo!();
        Ok(())
    }
}
