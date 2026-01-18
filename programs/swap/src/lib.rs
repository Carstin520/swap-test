pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("4F6SYakdp8jYbvSTMaWsFgBp5Vknp6hAjCgp4GsQzhPx");

#[program]
pub mod swap {
    use super::*;
    pub fn make_offer(
        ctx: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64
    ) -> Result<()> {
        instructions::make_offer::send_offer_tokens_to_vault(&ctx, token_a_offered_amount)?;
        instructions::make_offer::save_offer(ctx, id, token_b_wanted_amount)
    }
}

pub fn accept_offer(ctx: Context<AcceptOffer>) -> Result<()> {
    instructions::accept_offer::accept_offer(ctx)?;
    instructions::accept_offer::withdraw_and_close_vault(ctx)
}
