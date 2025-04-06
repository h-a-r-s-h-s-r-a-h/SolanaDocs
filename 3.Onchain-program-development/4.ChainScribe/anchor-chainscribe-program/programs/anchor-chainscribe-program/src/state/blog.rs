use anchor_lang::prelude::*;

#[account]
pub struct BlogAccountState {
    pub blog_generator: Pubkey,
    pub blog_generator_name: String,
    pub topic_id: String,
    pub blog_id: String,
    pub blog: String,
    pub comments: u32,
    pub likes: u32,
    pub last_updated_at: i64,
}
