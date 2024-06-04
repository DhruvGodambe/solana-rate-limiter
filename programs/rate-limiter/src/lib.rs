use anchor_lang::prelude::*;

declare_id!("BY1CwE5H6c2WwG2VXQJatWVecNCBBcFQgNZtZaYx1cXZ");

#[program]
pub mod rate_limiter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, init_price: u64, rate_limit: i64) -> Result<()> {
        let mut price_oracle = &ctx.accounts.price_oracle.to_account_info();
        let now_ts = Clock::get().unwrap().unix_timestamp;

        price_oracle.price = init_price;
        price_oracle.last_fetch_timestamp = now_ts;
        price_oracle.rate_limit = rate_limit;

        Ok(())
    }

    pub fn set_price(ctx: Context<OracleOperation>, price: u64) -> Result<()> {
        let mut price_oracle = &ctx.accounts.price_oracle.to_account_info();

        price_oracle.price = price;

        Ok(())
    }

    pub fn update_rate_limit(ctx: Context<OracleOperation>, rate_limit: i64) -> Result<()> {
        let mut price_oracle = &ctx.accounts.price_oracle.to_account_info();

        price_oracle.rate_limit = rate_limit;

        Ok(())
    }


    pub fn fetch_price(ctx: Context<OracleOperation>) -> Result<()> {

        let mut price_oracle = &ctx.accounts.price_oracle.to_account_info();
        
        let price = price_oracle.get_price();
        let rate_limit = price_oracle.rate_limit;
        let last_fetch_timestamp = price_oracle.last_fetch_timestamp;

        let now_ts = Clock::get().unwrap().unix_timestamp;

        // verify rate_limit is satisfied
        let mut fetchingAvailable = false;
        if now_ts - last_fetch_timestamp > rate_limit {
            fetchingAvailable = true;
        }

        // do something with this price
        msg!("Price fetched: {}", price);

        // update last fetched time
        price_oracle.last_fetch_timestamp = now_ts;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // store last time of price fetching
    #[account(init, payer = authority, space = 8 + size_of::<PriceOracle>())]
    pub price_oracle: Account<'info, PriceOracle>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct OracleOperation<'info> {
    #[account(mut)]
    pub price_oracle: Account<'info, PriceOracle>,

    pub authority: Signer<'info>
}

#[account]
#[derive(Default)]
pub struct PriceOracle {
    // store price
    price: u64,
    
    // store last time of price
    last_fetch_timestamp: i64,

    // store rate limit
    rate_limit: i64
}

impl PriceOracle {
    pub fn get_price(&self) -> u64 {
        self.price
    }

    pub fn update_fetch_time(&self, timestamp: i64) {
        self.last_fetch_timestamp = timestamp;
    }
}
