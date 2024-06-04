use anchor_lang::prelude::*;

declare_id!("BY1CwE5H6c2WwG2VXQJatWVecNCBBcFQgNZtZaYx1cXZ");

#[program]
pub mod rate_limiter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, init_price: u64, rate_limit: i64) -> Result<()> {
        let mut price_oracle = &mut ctx.accounts.price_oracle;
        let now_ts = Clock::get().unwrap().unix_timestamp;

        price_oracle.price = init_price;
        price_oracle.last_fetch_timestamp = now_ts;
        price_oracle.rate_limit = rate_limit;

        Ok(())
    }

    pub fn set_price(ctx: Context<OracleOperation>, price: u64) -> Result<()> {
        let mut price_oracle = &mut ctx.accounts.price_oracle;

        price_oracle.price = price;

        Ok(())
    }

    pub fn update_rate_limit(ctx: Context<OracleOperation>, rate_limit: i64) -> Result<()> {
        let mut price_oracle = &mut ctx.accounts.price_oracle;

        price_oracle.rate_limit = rate_limit;

        Ok(())
    }


    pub fn fetch_price(ctx: Context<OracleOperation>) -> Result<()> {

        let mut price_oracle = &mut ctx.accounts.price_oracle;
        
        let price = price_oracle.price;
        let rate_limit = price_oracle.rate_limit;
        let last_fetch_timestamp = price_oracle.last_fetch_timestamp;

        let now_ts = Clock::get().unwrap().unix_timestamp;

        // verify rate_limit is satisfied
        let mut fetchingAvailable = false;
        if now_ts - last_fetch_timestamp > rate_limit {
            fetchingAvailable = true;
        } else {
            return err!(OracleError::RateLimit);
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
    #[account(init, payer = authority, space = 8 + std::mem::size_of::<PriceOracle>())]
    pub price_oracle: Box<Account<'info, PriceOracle>>,

    #[account(mut)]
    authority: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct OracleOperation<'info> {
    #[account(mut)]
    pub price_oracle: Box<Account<'info, PriceOracle>>,

    pub authority: Signer<'info>
}

#[account]
#[derive(Default)]
pub struct PriceOracle {
    // store price
    pub price: u64,
    
    // store last time of price
    pub last_fetch_timestamp: i64,

    // store rate limit
    pub rate_limit: i64
}

#[error_code]
pub enum OracleError {
    #[msg("rate limit exceeded")]
    RateLimit,
}
