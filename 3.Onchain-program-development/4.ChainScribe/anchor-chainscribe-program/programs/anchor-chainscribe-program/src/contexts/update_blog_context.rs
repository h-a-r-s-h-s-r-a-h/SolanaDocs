use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(topic_id: String, blog_id: String)]
pub struct UpdateBlog<'info> {
    #[account(
        mut,
        seeds=["topic".as_bytes(), topic_id.as_bytes()],
        bump,
    )]
    pub topic: Account<'info, TopicAccountState>,

    #[account(
        mut,
        seeds=["blog".as_bytes(), topic_id.as_bytes(), blog_id.as_bytes()],
        bump,
    )]
    pub blog_account: Account<'info, BlogAccountState>,

    #[account(mut)]
    pub blog_generator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
