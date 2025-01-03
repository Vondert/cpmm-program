use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use anchor_spl::token_2022::{Token2022};
use anchor_spl::token_interface::{get_mint_extension_data, transfer_checked, transfer_checked_with_fee, Mint, TokenAccount};
use anchor_spl::token_interface::spl_token_2022::extension::transfer_fee::TransferFeeConfig;
use crate::utils::token_instructions::{TransferContextRegular, TransferContextWithFee};
use crate::error::ErrorCode;

pub struct TransferTokensInstruction<'at, 'bt, 'ct, 'info> {
    amount: u64,
    decimals: u8,
    context: TransferContextType<'at, 'bt, 'ct, 'info>,
}
impl<'at, 'bt, 'ct, 'info>  TransferTokensInstruction<'at, 'bt, 'ct, 'info>  {

    pub fn new(
        amount: u64, 
        mint: &'_ InterfaceAccount<'info, Mint>, 
        from: &'_ InterfaceAccount<'info, TokenAccount>, 
        from_authority: AccountInfo<'info>, 
        to: &'_ InterfaceAccount<'info, TokenAccount>, 
        token_program: &'_ Program<'info, Token>, 
        token_2022_program: &'_ Program<'info, Token2022>
    ) -> Result<Self> {
        require!(amount >= from.amount, ErrorCode::InsufficientBalanceForTransfer);
        
        let context = if mint.to_account_info().owner.key() == token_program.key(){
            TransferContextType::Regular(
                TransferContextRegular::new_for_spl_token(
                    mint, from, from_authority, to, token_program
                )
            )
        }else if let Ok(transfer_fee_config) = get_mint_extension_data::<TransferFeeConfig>(&mint.to_account_info()){
            let fee = transfer_fee_config.calculate_epoch_fee(Clock::get()?.epoch, amount).ok_or(ErrorCode::MintTransferFeeCalculationFailed)?;
            TransferContextType::WithFee(
                TransferContextWithFee::new_for_token_2022(
                    fee, mint, from, from_authority, to, token_2022_program
                )
            )
        }else{
            TransferContextType::Regular(
                TransferContextRegular::new_for_token_2022(
                    mint, from, from_authority, to, token_2022_program
                )
            )
        };

        Ok(Self {
            amount,
            decimals: mint.decimals,
            context,
        })
    }
    pub fn execute(mut self, optional_signers_seeds: Option<&'at[&'bt[&'ct[u8]]]>) -> Result<()>{
        if let Some(signer_seeds) = optional_signers_seeds {
            self.context = self.context.add_signers_seeds(signer_seeds);
        }
        match self.context {
            TransferContextType::Regular(context) => {
                transfer_checked(context.cpi_context, self.amount, self.decimals)
            },
            TransferContextType::WithFee(context) => {
                transfer_checked_with_fee(context.cpi_context, self.amount, self.decimals, context.fee)
            }
        }
    }
    
    pub fn get_decimals(&self) -> u8{
        self.decimals
    }
    pub fn get_amount_after_fee(&self) -> u64{
        self.get_raw_amount().checked_sub(self.get_fee()).unwrap()
    }
    pub fn get_fee(&self) -> u64{
        match &self.context{
            TransferContextType::Regular(_) => {
                0
            },
            TransferContextType::WithFee(context) => {
                context.fee
            }
        }
    }
    pub fn get_raw_amount(&self) -> u64{
        self.amount
    }
}


pub(crate) enum TransferContextType<'at, 'bt, 'ct, 'info> {
    Regular(TransferContextRegular<'at, 'bt, 'ct, 'info> ),
    WithFee(TransferContextWithFee<'at, 'bt, 'ct, 'info> )
}
impl<'at, 'bt, 'ct, 'info> TransferContextType<'at, 'bt, 'ct, 'info>{
    fn add_signers_seeds(self, signers_seeds: &'at[&'bt[&'ct[u8]]]) -> Self {
        match self {
            TransferContextType::Regular(context) => {
                TransferContextType::Regular(context.with_signers(signers_seeds))
            },
            TransferContextType::WithFee(context) => {
                TransferContextType::WithFee(context.with_signers(signers_seeds))
            }
        }
    }
}