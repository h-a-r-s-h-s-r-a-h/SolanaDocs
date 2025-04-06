use crate::{constants::*, contexts::*, errors::*};
use anchor_lang::prelude::*;

pub fn update_topic(
    ctx: Context<UpdateTopic>,
    _topic_id: String,
    topic_generator_name: String,
    topic_title: String,
    topic_description: String,
) -> Result<()> {
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
    require!(
        topic.topic_generator_id == ctx.accounts.topic_generator.key(),
        TopicAccountError::AdminNotFound
    );
    topic.topic_generator_name = topic_generator_name;
    topic.topic_title = topic_title;
    topic.topic_description = topic_description;
    topic.last_updated_at = Clock::get()?.unix_timestamp;

    Ok(())
}
