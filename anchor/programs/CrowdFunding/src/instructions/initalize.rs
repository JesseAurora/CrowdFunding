use anchor_lang::prelude::*;

use crate::constants::ANCHOR_DISCRIMINATION_SIZE;
use crate::errors::ErrorCode::*;
use crate::states::ProgramState;

pub fn initalize(ctx: Context<InitalizeCtx>) -> Result<()> {
    let state = &mut ctx.accounts.program_state;
    let deployer = &mut ctx.accounts.deployer;

    if state.initalized {
        return Err(AlreadyInitialized.into());
    }

    state.campaign_count = 0;
    state.platform_fee = 5;
    state.platform_address = deployer.key();
    state.initalized = true;

    Ok(())
}

#[derive(Accounts)]
pub struct InitalizeCtx<'info> {
    #[account(
        init,
        payer=deployer,
        space = ANCHOR_DISCRIMINATION_SIZE + ProgramState::INIT_SPACE,
        seeds=[b"program_state"],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,

    #[account(mut)]
    pub deployer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
