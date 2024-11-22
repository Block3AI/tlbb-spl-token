use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};

declare_id!("YourProgramPublicKeyHere");

#[program]
pub mod tlbb_spl_token {
    use super::*;

    // Initialize the token and distribute the supply
    pub fn initialize(ctx: Context<Initialize>, total_supply: u64) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        let owner = &ctx.accounts.owner;

        // Mint total supply to the owner's account
        token::mint_to(
            ctx.accounts.into_mint_to_context(),
            total_supply,
        )?;

        // Distribute the supply
        let supply_distribution = vec![
            (ctx.accounts.liquidity_pool.to_account_info(), 30),
            (ctx.accounts.presale_wallet.to_account_info(), 25),
            (ctx.accounts.marketing_wallet.to_account_info(), 20),
            (ctx.accounts.team_wallet.to_account_info(), 10),
            (ctx.accounts.community_rewards.to_account_info(), 10),
            (ctx.accounts.charity_wallet.to_account_info(), 5),
        ];

        for (account, percentage) in supply_distribution {
            let allocation = total_supply * percentage / 100;
            token::transfer(
                ctx.accounts
                    .into_transfer_context(owner.to_account_info(), account.clone()),
                allocation,
            )?;
        }

        Ok(())
    }

    // Handle transactions with a fee
    pub fn transfer_with_fee(
        ctx: Context<TransferWithFee>,
        amount: u64,
    ) -> Result<()> {
        let fee = amount * 2 / 100; // 2% transaction fee
        let charity_fee = fee / 2; // 1% to charity
        let development_fee = fee / 2; // 1% to development
        let transfer_amount = amount - fee;

        // Transfer to the charity wallet
        token::transfer(
            ctx.accounts.into_transfer_context(),
            charity_fee,
        )?;

        // Transfer to the development wallet
        token::transfer(
            ctx.accounts.into_transfer_context(),
            development_fee,
        )?;

        // Transfer the remaining amount to the recipient
        token::transfer(
            ctx.accounts.into_transfer_context(),
            transfer_amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub liquidity_pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub presale_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub marketing_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub team_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub community_rewards: Account<'info, TokenAccount>,
    #[account(mut)]
    pub charity_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferWithFee<'info> {
    #[account(mut)]
    pub sender: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    #[account(mut)]
    pub charity_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub development_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

// Context functions for transfer and mint operations
impl<'info> Initialize<'info> {
    fn into_mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.mint.to_account_info(),
                to: self.owner.to_account_info(),
                authority: self.owner.to_account_info(),
            },
        )
    }

    fn into_transfer_context(
        &self,
        from: AccountInfo<'info>,
        to: AccountInfo<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from,
                to,
                authority: self.owner.to_account_info(),
            },
        )
    }
}