use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked};


pub fn transfer_tokens<'info> (
    //InterfaceAccount makes our program work on both SPL token programs and token2022 programs
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    let transfer_accounts_options = TransferChecked {
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
        mint: mint.to_account_info(),
    };
    let cpi_context: CpiContext<'_, '_, '_, '_, TransferChecked<'_>> = CpiContext::new
    (token_program.to_account_info(),
    transfer_accounts_options);
    transfer_checked(cpi_context, *amount, mint.decimals)?;

    Ok(())
}
