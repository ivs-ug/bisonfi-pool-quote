#[derive(Debug, Clone, Copy)]
pub struct Tick {
    pub weight_a: u32,
    pub weight_b: u32,
    pub tier_id: i32,
    pub spread: i32,
}

impl Tick {
    pub const ZERO: Tick = Tick {
        weight_a: 0,
        weight_b: 0,
        tier_id: 0,
        spread: 0,
    };

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.weight_a == 0 && self.weight_b == 0
    }
}

#[derive(Debug, Clone)]
pub struct Pool {
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub total_liquidity: u64,
    pub oracle_ref_slot: u64,
    pub price_a_lo: u64,
    pub price_a_hi: u64,
    pub price_c_lo: u64,
    pub price_c_hi: u64,
    pub ticks1: [Tick; TICK_COUNT],
    pub ticks2: [Tick; TICK_COUNT],
    pub decimals_a: u8,
    pub decimals_b: u8,
    pub version: u8,
    pub paused: bool,
    pub v3_fee_rate_0: u16,
    pub v3_fee_rate_1: u16,
    pub v3_field_350: u16,
    pub v3_field_352: u16,
    pub v3_field_358: u16,
    pub v3_field_35a: u16,
    pub v3_field_35c: u16,
    pub v3_field_35e: u16,
    pub v3_field_370: u16,
    pub v3_field_37e: u16,
    pub v3_field_380: u16,
    pub v3_max_fee_0: u16, // diagnostic only
    pub v3_max_fee_1: u16, // diagnostic only
    pub v3_base_amount: u64,
    pub v3_divisor: u64,
    pub v3_pricing_flag: bool,
    pub v3_fee_alt_0: u16,
    pub v3_fee_alt_1: u16,
    pub v3_field_372: u16,
    pub v3_slot_tolerance: u8, // @0x37d
    pub v3_tick_mode: u64,     // @0x408
    pub fee_rate_a: u16,
    pub fee_rate_b: u16,
    pub fee_exp_a: u8,
    pub fee_exp_b: u8,
    pub reserve_cap: u64,
    pub fee_fallback_a: u16,
    pub fee_fallback_b: u16,
    pub last_tick_slot: u64,
    pub v2_pricing_flag: bool,
    pub v2_fee_rate_0: u16,
    pub v2_fee_exp_0: u8,
    pub v2_fee_rate_1: u16,
    pub v2_fee_exp_1: u8,
}

impl Pool {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < ACCOUNT_MIN_LEN {
            return Err(eyre!("too short"));
        }
        if &data[0..8] != DISCRIMINATOR {
            return Err(eyre!("bad discriminator"));
        }
        let version = rd_u8(data, OFF_VERSION);
        if version < 2 || version > 3 {
            return Err(eyre!("unsupported version"));
        }
        let price_a_lo = rd_u64(data, OFF_PRICE_A_LO);
        let price_a_hi = rd_u64(data, OFF_PRICE_A_HI);
        if price_a_lo == 0 && price_a_hi == 0 {
            return Err(eyre!("decimal adjustment zero"));
        }

        let mut ticks1 = [Tick::ZERO; TICK_COUNT];
        let mut ticks2 = [Tick::ZERO; TICK_COUNT];
        for i in 0..TICK_COUNT {
            ticks1[i] = rd_tick(data, OFF_TICKS1 + i * TICK_STRIDE);
            ticks2[i] = rd_tick(data, OFF_TICKS2 + i * TICK_STRIDE);
        }

        let paused = rd_u8(
            data,
            if version == 3 {
                OFF_PAUSED_V3
            } else {
                OFF_PAUSED_V2
            },
        ) != 0;

        Ok(Pool {
            vault_a: rd_pubkey(data, OFF_VAULT_A),
            vault_b: rd_pubkey(data, OFF_VAULT_B),
            mint_a: rd_pubkey(data, OFF_MINT_A),
            mint_b: rd_pubkey(data, OFF_MINT_B),
            reserve_a: rd_u64(data, OFF_RESERVE_A),
            reserve_b: rd_u64(data, OFF_RESERVE_B),
            total_liquidity: rd_u64(data, OFF_TOTAL_LIQ),
            oracle_ref_slot: rd_u64(data, OFF_ORACLE_SLOT),
            price_a_lo,
            price_a_hi,
            price_c_lo: rd_u64(data, OFF_PRICE_C_LO),
            price_c_hi: rd_u64(data, OFF_PRICE_C_HI),
            ticks1,
            ticks2,
            decimals_a: rd_u8(data, OFF_DECIMALS_A),
            decimals_b: rd_u8(data, OFF_DECIMALS_B),
            version,
            paused,
            v3_fee_rate_0: rd_u16(data, OFF_V3_FEE_RATE_0),
            v3_fee_rate_1: rd_u16(data, OFF_V3_FEE_RATE_1),
            v3_field_350: rd_u16(data, OFF_V3_FIELD_350),
            v3_field_352: rd_u16(data, OFF_V3_FIELD_352),
            v3_field_358: rd_u16(data, OFF_V3_FIELD_358),
            v3_field_35a: rd_u16(data, OFF_V3_FIELD_35A),
            v3_field_35c: rd_u16(data, OFF_V3_FIELD_35C),
            v3_field_35e: rd_u16(data, OFF_V3_FIELD_35E),
            v3_field_370: rd_u16(data, OFF_V3_FIELD_370),
            v3_field_37e: rd_u16(data, OFF_V3_FIELD_37E),
            v3_field_380: rd_u16(data, OFF_V3_FIELD_380),
            v3_max_fee_0: rd_u16(data, OFF_V3_MAX_FEE_0),
            v3_max_fee_1: rd_u16(data, OFF_V3_MAX_FEE_1),
            v3_base_amount: rd_u64(data, OFF_V3_BASE_AMOUNT),
            v3_divisor: rd_u64(data, OFF_V3_DIVISOR),
            v3_pricing_flag: rd_u8(data, OFF_V3_PRICING_FLAG) != 0,
            v3_fee_alt_0: rd_u16(data, OFF_V3_FEE_ALT_0),
            v3_fee_alt_1: rd_u16(data, OFF_V3_FEE_ALT_1),
            v3_field_372: rd_u16(data, OFF_V3_FIELD_372),
            v3_slot_tolerance: rd_u8(data, OFF_V3_SLOT_TOLERANCE),
            v3_tick_mode: rd_u64(data, OFF_V3_TICK_MODE),
            fee_rate_a: rd_u16(data, OFF_FEE_RATE_A),
            fee_rate_b: rd_u16(data, OFF_FEE_RATE_B),
            fee_exp_a: rd_u8(data, OFF_FEE_EXP_A),
            fee_exp_b: rd_u8(data, OFF_FEE_EXP_B),
            reserve_cap: rd_u64(data, OFF_RESERVE_CAP),
            fee_fallback_a: rd_u16(data, OFF_FEE_FB_A),
            fee_fallback_b: rd_u16(data, OFF_FEE_FB_B),
            last_tick_slot: rd_u64(data, OFF_LAST_TICK_SLOT),
            v2_pricing_flag: rd_u8(data, 0x33E) != 0,
            v2_fee_rate_0: rd_u16(data, 0x340),
            v2_fee_exp_0: rd_u8(data, 0x342),
            v2_fee_rate_1: rd_u16(data, 0x344),
            v2_fee_exp_1: rd_u8(data, 0x346),
        })
    }
}

pub fn quote(
    amount_in: u64,
    _ra: u64,
    _rb: u64,
    a_to_b: bool,
    pool: &Pool,
    current_slot: u64,
) -> Option<u64> {
  // ...
}
