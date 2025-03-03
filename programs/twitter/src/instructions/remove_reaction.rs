use anchor_lang::prelude::*;

pub fn delete_reaction(ctx: Context<RemoveReaction>) -> Result<()> {

  Ok(())
}

#[derive(Accounts)]
pub struct RemoveReaction<'info> {
    
}