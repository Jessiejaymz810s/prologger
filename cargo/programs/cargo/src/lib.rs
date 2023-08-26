use anchor_lang::prelude::*;

declare_id!("6fB9WUia2btTZagzbcrXvoga5VXYJGqa7J8P2rFjEb5b");

#[program]
pub mod cargo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
