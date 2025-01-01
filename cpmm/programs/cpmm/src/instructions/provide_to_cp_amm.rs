use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Token;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::state::{AmmsConfig, CpAmm};
use crate::utils::token_instructions::{MintTokensInstructions, TransferTokensInstruction};

#[derive(Accounts)]
pub struct ProvideToCpAmm<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,
    pub base_mint: Box<InterfaceAccount<'info, Mint>>,
    pub quote_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub lp_mint: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    // Token program will check mint and authority via token_instructions instruction
    pub signer_base_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    // Token program will check mint and authority via token_instructions instruction
    pub signer_quote_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = lp_mint,
        associated_token::authority = signer,
    )]
    pub signer_lp_account: Box<Account<'info, token::TokenAccount>>,

    #[account(
        seeds = [AmmsConfig::SEED, amms_config.id.to_le_bytes().as_ref()],
        bump = amms_config.bump
    )]
    pub amms_config: Box<Account<'info, AmmsConfig>>,

    #[account(
        mut,
        constraint = cp_amm.is_launched(),
        constraint = amms_config.key() == cp_amm.amms_config().key(),
        constraint = base_mint.key() == cp_amm.base_mint().key(),
        constraint = quote_mint.key() == cp_amm.quote_mint().key(),
        constraint = cp_amm_base_vault.key() == cp_amm.base_vault().key(),
        constraint = cp_amm_quote_vault.key() == cp_amm.quote_vault().key(),
        seeds = [CpAmm::SEED, lp_mint.key().as_ref()],
        bump = cp_amm.bump()
    )]
    pub cp_amm: Box<Account<'info, CpAmm>>,

    #[account(mut)]
    pub cp_amm_base_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub cp_amm_quote_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub token_2022_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

impl<'info> ProvideToCpAmm<'info>{
    fn get_provide_base_liquidity_transfer_instruction(&self, base_liquidity: u64) -> Result<TransferTokensInstruction<'_, '_, '_, 'info>>{
        TransferTokensInstruction::new(
            base_liquidity,
            &self.base_mint,
            &self.signer_base_account,
            self.signer.to_account_info(),
            &self.cp_amm_base_vault,
            &self.token_program,
            &self.token_2022_program
        )
    }
    fn get_provide_quote_liquidity_transfer_instruction(&self, quote_liquidity: u64) -> Result<TransferTokensInstruction<'_, '_, '_, 'info>>{
        TransferTokensInstruction::new(
            quote_liquidity,
            &self.quote_mint,
            &self.signer_quote_account,
            self.signer.to_account_info(),
            &self.cp_amm_quote_vault,
            &self.token_program,
            &self.token_2022_program
        )
    }
    fn get_liquidity_mint_instruction(&self, liquidity: u64) -> Result<MintTokensInstructions<'_, '_, '_, 'info>>{
        MintTokensInstructions::new(
            liquidity,
            &self.lp_mint,
            self.cp_amm.to_account_info(),
            &self.signer_lp_account,
            &self.token_program
        )
    }
}

pub(crate) fn handler(ctx: Context<ProvideToCpAmm>, base_liquidity: u64, quote_liquidity: u64) -> Result<()> {

    let provide_base_liquidity_instruction = Box::new(ctx.accounts.get_provide_base_liquidity_transfer_instruction(base_liquidity)?);
    let provide_quote_liquidity_instruction = Box::new(ctx.accounts.get_provide_quote_liquidity_transfer_instruction(quote_liquidity)?);

    let base_liquidity_to_provide = provide_base_liquidity_instruction.get_amount_after_fee();
    let quote_liquidity_to_provide = provide_quote_liquidity_instruction.get_amount_after_fee();
    
    let provide_payload = ctx.accounts.cp_amm.get_provide_payload(base_liquidity_to_provide, quote_liquidity_to_provide)?;

    let liquidity_mint_instruction = Box::new(ctx.accounts.get_liquidity_mint_instruction(provide_payload.lp_tokens_to_mint())?);

    provide_base_liquidity_instruction.execute(None)?;
    provide_quote_liquidity_instruction.execute(None)?;

    let cp_amm_seeds = ctx.accounts.cp_amm.seeds();
    let mint_instruction_seeds: &[&[&[u8]]] = &[&cp_amm_seeds];

    liquidity_mint_instruction.execute(Some(mint_instruction_seeds))?;
    
    ctx.accounts.cp_amm.provide(provide_payload);
    
    Ok(())
}