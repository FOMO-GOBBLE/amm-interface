use std::collections::HashSet;
use anyhow::{anyhow, Result};
use jupiter_amm_interface::{Amm, AmmContext, KeyedAccount};
use solana_sdk::pubkey::Pubkey;

use super::gobbler_swap::GobblerAmm; // Import the GobblerAmm

pub fn amm_factory(
    keyed_account: &KeyedAccount,
    amm_context: &AmmContext,
    _saber_wrapper_mints: &mut HashSet<Pubkey>,
) -> Result<Box<dyn Amm + Send + Sync>> {
    let owner = keyed_account.account.owner;

    // if owner == GobblerAmm::program_id() {
        Ok(Box::new(GobblerAmm::from_keyed_account(keyed_account, amm_context)?))
    // } else {
        // Err(anyhow!("Unsupported AMM program: {}", owner))
    // }
}
