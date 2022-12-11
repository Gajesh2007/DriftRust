use solana_sdk::pubkey::Pubkey;

pub fn get_pda_address(program_id: &Pubkey, seeds: &[&[u8]]) -> (Pubkey, u8) {
    Pubkey::find_program_address(seeds, program_id)
}

pub fn get_drift_state_account_public_key_and_nonce(program_id: &Pubkey) -> (Pubkey, u8) {
    get_pda_address(program_id, &[b"drift_state"])
}

pub fn get_drift_state_account_public_key(program_id: &Pubkey) -> Pubkey {
    get_drift_state_account_public_key_and_nonce(program_id).0
}

pub fn get_user_account_public_and_nonce(program_id: &Pubkey, user: &Pubkey, sub_account_id: u128) -> (Pubkey, u8) {
    get_pda_address(
        program_id, 
        &[
            b"user", 
            user.as_ref(),
            sub_account_id.to_le_bytes().as_ref()
            ]
        )
}

pub fn get_user_account_public_key(program_id: &Pubkey, user: &Pubkey, sub_account_id: u128) -> Pubkey {
    get_user_account_public_and_nonce(program_id, user, sub_account_id).0
}

pub fn get_user_stats_account_public_key(program_id: &Pubkey, user: &Pubkey) -> Pubkey {
    get_pda_address(
        program_id, 
        &[
            b"user_stats", 
            user.as_ref()
            ]
        ).0
}

pub fn get_perp_market_public_key(program_id: &Pubkey, market_index: u128) -> Pubkey {
    get_pda_address(
        program_id, 
        &[
            b"perp_market", 
            market_index.to_le_bytes().as_ref()
            ]
        ).0
}

pub fn get_spot_market_public_key(program_id: &Pubkey, market_index: u128) -> Pubkey {
    get_pda_address(
        program_id, 
        &[
            b"spot_market", 
            market_index.to_le_bytes().as_ref()
            ]
        ).0
}

pub fn get_spot_market_vault_public_key(program_id: &Pubkey, market_index: u128) -> Pubkey {
    get_pda_address(
        program_id, 
        &[
            b"spot_market_vault", 
            market_index.to_le_bytes().as_ref()
            ]
        ).0
}

pub fn get_insurance_fund_vault_public_key(program_id: &Pubkey, market_index: u128) -> Pubkey {
    get_pda_address(
        program_id, 
        &[
            b"insurance_fund_vault", 
            market_index.to_le_bytes().as_ref()
            ]
        ).0
}

pub fn get_insurance_fund_stake_public_key(program_id: &Pubkey, market_index: u128) -> Pubkey {
    get_pda_address(
        program_id, 
        &[
            b"insurance_fund_stake", 
            market_index.to_le_bytes().as_ref()
            ]
        ).0
}

pub fn get_drift_signer_public_key(program_id: &Pubkey) -> Pubkey {
    get_pda_address(
        program_id, 
        &[
            b"drift_signer"
            ]
        ).0
}

pub fn get_serum_open_orders_public_key(program_id: &Pubkey, market: &Pubkey) -> Pubkey {
    get_pda_address(
        program_id, 
        &[
            b"serum_open_orders", 
            market.as_ref(),
            ]
        ).0
}

pub fn get_serum_signer_public_key(program_id: &Pubkey, market: &Pubkey, nonce: u8) -> Pubkey {
    Pubkey::create_program_address(&[market.as_ref(), &nonce.to_le_bytes()], program_id).unwrap()
}

pub fn get_serum_fulfillment_config_public_key(program_id: &Pubkey, market: &Pubkey) -> Pubkey {
    get_pda_address(
        program_id, 
        &[
            b"serum_fulfillment_config", 
            market.as_ref(),
            ]
        ).0
}