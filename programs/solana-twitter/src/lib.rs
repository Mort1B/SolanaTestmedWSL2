use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("9pCHeTL6CVzUsWgNbZn52wvvNz46RmhvgLezpjtniGpM");

#[program]
pub mod solana_twitter {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendTweet<'info> {
    #[account(init, payer = author, space = Tweet::LEN)]
    pub tweet: Account<'info, Tweet>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

// Define structure of tweet account
#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}

// Constants for sizing properties
const DISCRIMINTAOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TOPIC_LENGTH: usize = 50 * 4;
const MAX_CONTENT_LENGTH: usize = 280 * 4;

// Add a constant on the tweet account that provides its total size.
impl Tweet {
    const LEN: usize = DISCRIMINTAOR_LENGTH
    + PUBLIC_KEY_LENGTH // author
    + TIMESTAMP_LENGTH // timestamp
    + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // topic
    + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH; // content
}
