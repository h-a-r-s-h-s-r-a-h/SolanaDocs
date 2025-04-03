use crate::{constants::*, contexts::comment_context::UpdateComment, errors::*};
use anchor_lang::prelude::*;

pub fn update_comment(
    ctx: Context<UpdateComment>,
    _comment_id: String,
    _blog_id: String,
    _topic_id: String,
    comment_text: String,
) -> Result<()> {
    require!(
        comment_text.len() <= MAX_COMMENT_LENGTH,
        CommentAccountError::CommentTextTooLong
    );

    let comment = &mut ctx.accounts.comment;
    require!(
        comment.commenter == ctx.accounts.commenter.key(),
        CommentAccountError::AdminNotFound
    );
    comment.comment_text = comment_text;
    comment.last_updated_at = Clock::get()?.unix_timestamp;

    Ok(())
}
