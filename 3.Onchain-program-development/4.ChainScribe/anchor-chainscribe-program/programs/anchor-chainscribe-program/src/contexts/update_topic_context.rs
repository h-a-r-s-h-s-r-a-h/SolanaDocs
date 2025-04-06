use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(topic_id: String)]
pub struct UpdateTopic<'info> {
    #[account(
        mut,
        seeds=["topic".as_bytes(), topic_id.as_bytes()],
        bump,
    )]
    pub topic: Account<'info, TopicAccountState>,

    #[account(mut)]
    pub topic_generator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
