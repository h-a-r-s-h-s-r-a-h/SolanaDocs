use crate::{constants::*, contexts::*, errors::*};
use anchor_lang::prelude::*;

pub fn create_topic(
    ctx: Context<CreateTopic>,
    topic_id: String,
    topic_generator_name: String,
    topic_title: String,
    topic_description: String,
) -> Result<()> {
    require!(
        topic_id.len() <= MAX_ID_LENGTH,
        TopicAccountError::TopicIdTooLong
    );

    require!(
        topic_generator_name.len() <= MAX_GENERATOR_NAME,
        TopicAccountError::GeneratorNameTooLong
    );

    require!(
        topic_title.len() <= MAX_TOPIC_TITLE,
        TopicAccountError::TopicTitleTooLong
    );

    require!(
        topic_description.len() <= MAX_TOPIC_DESCRIPTION,
        TopicAccountError::TopicDescriptionTooLong
    );

    let topic = &mut ctx.accounts.topic;
    topic.topic_generator_id = ctx.accounts.topic_generator.key();
    topic.topic_generator_name = topic_generator_name;
    topic.topic_id = topic_id;
    topic.topic_title = topic_title;
    topic.topic_description = topic_description;
    topic.no_of_blog = 0;
    topic.likes = 0;
    topic.comments = 0;
    topic.is_active = true;
    topic.is_public = true;
    topic.last_updated_at = Clock::get()?.unix_timestamp;

    Ok(())
}


