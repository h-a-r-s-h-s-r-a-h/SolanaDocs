use crate::{constants::*, contexts::comment_context::AddComment, errors::*};
use anchor_lang::prelude::*;

pub fn add_comment(
    ctx: Context<AddComment>,
    comment_id: String,
    blog_id: String,
    topic_id: String,
    comment_text: String,
) -> Result<()> {
    require!(
        comment_text.len() <= MAX_COMMENT_LENGTH,
        CommentAccountError::CommentTextTooLong
    );

    require!(
        comment_id.len() <= MAX_ID_LENGTH,
        CommentAccountError::CommentIdTooLong
    );

    let comment = &mut ctx.accounts.comment;
    comment.commenter = ctx.accounts.commenter.key();
    comment.comment_id = comment_id;
    comment.topic_id = topic_id;
    comment.blog_id = blog_id;
    comment.comment_text = comment_text;
    comment.last_updated_at = Clock::get()?.unix_timestamp;

    let blog = &mut ctx.accounts.blog_account;
    blog.comments = blog.comments.checked_add(1).unwrap();
    Ok(())
}
