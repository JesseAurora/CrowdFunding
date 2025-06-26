
use crate::errors::ErrorCode::*;
use anchor_lang::prelude::*;

use crate::{
    constants::ANCHOR_DISCRIMINATION_SIZE,
    states::{Campaign, ProgramState, Transaction},
};

pub fn withdraw(ctx: Context<WithdrawCtx>, cid: u64, amount: u64) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;

    let transaction = &mut ctx.accounts.transaction;
    let state = &mut ctx.accounts.program_state;
    let creator = &ctx.accounts.creator;
    let platform_account_info = &ctx.accounts.platform_addreess;

    if campaign.cid != cid {
        return Err(CampaignNotFound.into());
    }

    if campaign.creator != creator.key() {
        return Err(Unauthorized.into());
    }

    if amount < 1_000_000_000 {
        return Err(InvalidWithdrawalAmount.into());
    }

    if amount > campaign.balance {
        return Err(CampaignGoalActualized.into());
    }

    if platform_account_info.key() != state.platform_address {
        return Err(InvalidPlatformAddress.into());
    }

    // rent safe money to keep not to be deleted
    let rent_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());

    // withdraw money have to keep
    // ** deference twice
    // all - rent
    if amount > **campaign.to_account_info().lamports.borrow() - rent_balance {
        msg!("withdrawal exceed campagin's usable balance");
        return Err(InsufficientFund.into());
    }

    let platform_fee = amount * state.platform_fee / 100;
    let creator_amount = amount - platform_fee;

    **campaign.to_account_info().try_borrow_mut_lamports()? -= creator_amount;
    **creator.to_account_info().try_borrow_mut_lamports()? += creator_amount;

    **campaign.to_account_info().try_borrow_mut_lamports()? -= platform_fee;
    **platform_account_info.try_borrow_mut_lamports()? += platform_fee;

    campaign.withdrawals += 1;
    campaign.balance -= amount;

    transaction.amount = amount;
    transaction.cid = cid;
    transaction.credited = false;
    transaction.owner = creator.key();
    transaction.timestamp = Clock::get()?.unix_timestamp as u64;

    Ok(())
}

#[derive(Accounts)]
#[instruction(cid:u64)]
pub struct WithdrawCtx<'info> {
    #[account(
        mut,
        seeds=[b"campaign",cid.to_le_bytes().as_ref()],
        bump
    )]
    pub campaign: Account<'info, Campaign>,

    #[account(
        init,
        payer=creator,
        space=ANCHOR_DISCRIMINATION_SIZE+Transaction::INIT_SPACE,
        seeds=[b"withdraw",
        creator.key().as_ref(),
        cid.to_le_bytes().as_ref(),
        (campaign.withdrawals +1).to_le_bytes().as_ref()],
        bump,
    )]
    pub transaction: Account<'info, Transaction>,

    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,

    /// CHECK:
    #[account(mut)]
    pub platform_addreess: AccountInfo<'info>,

    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
