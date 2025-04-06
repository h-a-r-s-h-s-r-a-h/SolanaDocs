use crate::{constants::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(topic_id: String, blog_id: String)]
pub struct CreateBlog<'info> {
    #[account(
        mut,
        seeds=["topic".as_bytes(), topic_id.as_bytes()],
        bump,
    )]
    pub topic: Account<'info, TopicAccountState>,

    #[account(
        init,
        seeds=["blog".as_bytes(), topic_id.as_bytes(), blog_id.as_bytes(),],
        bump,
        payer=blog_generator,
        space=BlogAccountState::INIT_SPACE + MAX_GENERATOR_NAME + MAX_ID_LENGTH + MAX_BLOG_SIZE + MAX_ID_LENGTH
    )]
    pub blog_account: Account<'info, BlogAccountState>,

    #[account(mut)]
    pub blog_generator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl Space for BlogAccountState {
    const INIT_SPACE: usize = ANCHOR_DISCRIMINATOR
        + PUBKEY_SIZE
        + STRING_LENGTH_PREFIX
        + STRING_LENGTH_PREFIX
        + STRING_LENGTH_PREFIX
        + STRING_LENGTH_PREFIX
        + U32_SIZE
        + U32_SIZE
        + I64_SIZE;
}
