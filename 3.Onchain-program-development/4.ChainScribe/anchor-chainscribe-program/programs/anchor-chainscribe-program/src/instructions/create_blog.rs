use crate::{constants::*, contexts::*, errors::*};
use anchor_lang::prelude::*;

pub fn create_blog(
    ctx: Context<CreateBlog>,
    topic_id: String,
    blog_id: String,
    blog_generator_name: String,
    blog: String,
) -> Result<()> {
    require!(
        topic_id.len() <= MAX_ID_LENGTH,
        BlogAccountError::TopicIdTooLong
    );

    require!(
        blog_id.len() <= MAX_ID_LENGTH,
        BlogAccountError::BlogIdTooLong
    );

    require!(
        blog_generator_name.len() <= MAX_GENERATOR_NAME,
        BlogAccountError::BlogGeneratorNameTooLong
    );

    require!(blog.len() <= MAX_BLOG_SIZE, BlogAccountError::BlogTooLong);

    let blog_account = &mut ctx.accounts.blog_account;
    blog_account.blog_generator = ctx.accounts.blog_generator.key();
    blog_account.blog_generator_name = blog_generator_name;
    blog_account.topic_id = topic_id;
    blog_account.blog_id = blog_id;
    blog_account.comments = 0;
    blog_account.likes = 0;
    blog_account.last_updated_at = Clock::get()?.unix_timestamp;
    blog_account.blog = blog;

    let topic = &mut ctx.accounts.topic;
    topic.no_of_blog = topic.no_of_blog.checked_add(1).unwrap();
    topic.last_updated_at = Clock::get()?.unix_timestamp;

    Ok(())
}
