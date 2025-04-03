use crate::{contexts::comment_context::DeleteComment, errors::*};
use anchor_lang::prelude::*;

pub fn delete_comment(
    ctx: Context<DeleteComment>,
    _comment_id: String,
    _blog_id: String,
    _topic_id: String,
) -> Result<()> {
    let comment = &mut ctx.accounts.comment;
    require!(
        comment.commenter == ctx.accounts.commenter.key(),
        CommentAccountError::AdminNotFound
    );
    Ok(())
}
