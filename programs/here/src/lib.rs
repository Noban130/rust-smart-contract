use anchor_lang::prelude::*;

declare_id!("9vxxPimLzVMLiDf788mTzJyv66EvtyJpZXS9ZYq8JZjo");

#[program]
pub mod mysolanaapp {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, number: u64, message: String) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        data_account.number = number;
        data_account.message = message;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 4 + 50)] // 8 bytes for discriminator, 8 for u64, 4 + 50 for string
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub number: u64,
    pub message: String,
}



