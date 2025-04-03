use crate::{constants::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(topic_id: String)]
pub struct CreateTopic<'info> {
    #[account(
        init,
        seeds=["topic".as_bytes(), topic_id.as_bytes()],
        bump,
        payer=topic_generator,
        space=TopicAccountState::INIT_SPACE + MAX_GENERATOR_NAME + MAX_ID_LENGTH + MAX_TOPIC_TITLE + MAX_TOPIC_DESCRIPTION
    )]
    pub topic: Account<'info, TopicAccountState>,

    #[account(mut)]
    pub topic_generator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl Space for TopicAccountState {
    const INIT_SPACE: usize = ANCHOR_DISCRIMINATOR
        + PUBKEY_SIZE
        + STRING_LENGTH_PREFIX
        + STRING_LENGTH_PREFIX
        + STRING_LENGTH_PREFIX
        + STRING_LENGTH_PREFIX
        + U32_SIZE
        + U32_SIZE
        + U32_SIZE
        + BOOL_SIZE
        + BOOL_SIZE
        + I64_SIZE;
}
