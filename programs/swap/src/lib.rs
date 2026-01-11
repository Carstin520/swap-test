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
    use crate::instruction::MakeOffer;

    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>) -> Result<()> {
        instruction::make_offer::send_offer_tokens_to_vault()?;
        instruction::make_offer::save_offer()?;
    }
}
