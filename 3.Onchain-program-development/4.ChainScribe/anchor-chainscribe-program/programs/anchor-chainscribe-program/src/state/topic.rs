use anchor_lang::prelude::*;

#[account]
pub struct TopicAccountState {
    pub topic_generator_id: Pubkey,
    pub topic_generator_name: String,
    pub topic_id: String,
    pub topic_title: String,
    pub topic_description: String,
    pub no_of_blog: u32,
    pub likes: u32,
    pub comments: u32,
    pub is_active: bool,
    pub is_public: bool,
    pub last_updated_at: i64,
}
