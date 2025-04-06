use crate::{constants::*, state::{CommentAccountState, BlogAccountState}};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(comment_id: String, blog_id: String,topic_id:String, comment_text: String)]
pub struct AddComment<'info> {

    #[account(
        mut,
        seeds=["blog".as_bytes(), topic_id.as_bytes(), blog_id.as_bytes()],
        bump,
    )]
    pub blog_account: Account<'info, BlogAccountState>,
    #[account(
        init,
        seeds = [
            "comment".as_bytes(), 
            comment_id.as_bytes(), 
            blog_id.as_bytes(), 
            commenter.key.as_ref()
        ],
        bump,
        payer = commenter,
        space = CommentAccountState::INIT_SPACE 
            + comment_text.len() 
            + comment_id.len() 
            + blog_id.len() 
            + topic_id.len(),
    )]
    pub comment:Account<'info, CommentAccountState>,

    #[account(mut)]
    pub commenter: Signer<'info>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(comment_id: String, blog_id: String,topic_id:String, comment_text: String)]
pub struct UpdateComment<'info> {
    #[account(
        mut,
        seeds = [
            "comment".as_bytes(), 
            comment_id.as_bytes(), 
            blog_id.as_bytes(), 
            commenter.key.as_ref()
        ],
        bump,
        realloc=CommentAccountState::INIT_SPACE + comment_text.len() 
        + comment_id.len() 
        + blog_id.len() 
        + topic_id.len(),
        realloc::payer = commenter,
        realloc::zero = true
    )]
    pub comment:Account<'info, CommentAccountState>,

    #[account(mut)]
    pub commenter: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(comment_id: String, blog_id: String,topic_id:String)]
pub struct DeleteComment<'info> {
    #[account(
        mut,
        seeds = [
            "comment".as_bytes(), 
            comment_id.as_bytes(), 
            blog_id.as_bytes(), 
            commenter.key.as_ref()
        ],
        bump,
        close = commenter
    )]
    pub comment:Account<'info, CommentAccountState>,

    #[account(mut)]
    pub commenter: Signer<'info>,

    pub system_program: Program<'info, System>,

}


impl Space for CommentAccountState {
    const INIT_SPACE: usize = ANCHOR_DISCRIMINATOR
        + PUBKEY_SIZE
        + STRING_LENGTH_PREFIX
        + STRING_LENGTH_PREFIX
        + STRING_LENGTH_PREFIX
        + STRING_LENGTH_PREFIX
        + I64_SIZE;
}