use anchor_lang::prelude::*;

pub fn delete_comment(ctx: Context<RemoveComment>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct RemoveComment<'info> {
    
}
