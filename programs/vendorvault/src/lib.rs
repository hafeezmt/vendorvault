use anchor_lang::prelude::*;

declare_id!("PLACEHOLDER_PROGRAM_ID");

#[program]
pub mod vendorvault {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
