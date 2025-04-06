use anchor_lang::prelude::*;

mod constants;
mod contexts;
mod errors;
mod instructions;
mod state;

use contexts::*;

declare_id!("FnAbLTmMMUA6XvadsrFm8pHtAYYNHEnKo4Q7tcG5vhiL");

#[program]
pub mod anchor_chainscribe_program {
    use super::*;

    pub fn create_topic(
        ctx: Context<CreateTopic>,
        topic_id: String,
        topic_generator_name: String,
        topic_title: String,
        topic_description: String,
    ) -> Result<()> {
        instructions::create_topic(
            ctx,
            topic_id,
            topic_generator_name,
            topic_title,
            topic_description,
        )
    }

    pub fn create_blog(
        ctx: Context<CreateBlog>,
        topic_id: String,
        blog_id: String,
        blog_generator_name: String,
        blog: String,
    ) -> Result<()> {
        instructions::create_blog(ctx, topic_id, blog_id, blog_generator_name, blog)
    }

    pub fn update_topic(
        ctx: Context<UpdateTopic>,
        topic_id: String,
        topic_generator_name: String,
        topic_title: String,
        topic_description: String,
    ) -> Result<()> {
        instructions::update_topic(
            ctx,
            topic_id,
            topic_generator_name,
            topic_title,
            topic_description,
        )
    }

    pub fn update_blog(
        ctx: Context<UpdateBlog>,
        topic_id: String,
        blog_id: String,
        blog: String,
    ) -> Result<()> {
        instructions::update_blog(ctx, topic_id, blog_id, blog)
    }

    pub fn add_like_to_blog(
        ctx: Context<UpdateBlog>,
        topic_id: String,
        blog_id: String,
    ) -> Result<()> {
        instructions::add_likes_to_blog(ctx, topic_id, blog_id)
    }

    pub fn add_like_to_topic(ctx: Context<UpdateTopic>, topic_id: String) -> Result<()> {
        instructions::add_like_to_topic(ctx, topic_id)
    }

    pub fn add_comment(
        ctx: Context<AddComment>,
        comment_id: String,
        blog_id: String,
        topic_id: String,
        comment_text: String,
    ) -> Result<()> {
        instructions::add_comment(ctx, comment_id, blog_id, topic_id, comment_text)
    }

    pub fn update_comment(
        ctx: Context<UpdateComment>,
        comment_id: String,
        blog_id: String,
        topic_id: String,
        comment_text: String,
    ) -> Result<()> {
        instructions::update_comment(ctx, comment_id, blog_id, topic_id, comment_text)
    }

    pub fn delete_comment(
        ctx: Context<DeleteComment>,
        comment_id: String,
        blog_id: String,
        topic_id: String,
    ) -> Result<()> {
        instructions::delete_comment(ctx, comment_id, blog_id, topic_id)
    }
}
