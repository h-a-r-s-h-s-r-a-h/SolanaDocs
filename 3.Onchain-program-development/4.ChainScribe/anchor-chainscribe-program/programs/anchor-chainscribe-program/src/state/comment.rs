use anchor_lang::prelude::*;

#[account]
pub struct CommentAccountState {
    pub commenter: Pubkey,
    pub comment_id: String,
    pub topic_id: String,
    pub blog_id: String,
    pub comment_text: String,
    pub last_updated_at: i64,
}
