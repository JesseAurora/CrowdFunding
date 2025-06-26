use anchor_lang::prelude::*;

use crate::errors::ErrorCode::*;
use crate::states::ProgramState;

pub fn update_platform_settings(
    ctx: Context<UpdatePlatformSettings>,
    new_latform_fee: u64,
) -> Result<()> {
    let state = &mut ctx.accounts.program_state;
    let updater = &ctx.accounts.updater;

    if updater.key() != state.platform_address {
        return Err(Unauthorized.into());
    }

    if !(1..=15).contains(&new_latform_fee) {
        return Err(InvalidPlatformFee.into());
    }

    state.platform_fee = new_latform_fee;

    Ok(())
}

#[derive(Accounts)]
pub struct UpdatePlatformSettings<'info> {
    #[account(mut)]
    pub updater: Signer<'info>,

    #[account(
        mut,
        seeds=[b"program_state"],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,
}
