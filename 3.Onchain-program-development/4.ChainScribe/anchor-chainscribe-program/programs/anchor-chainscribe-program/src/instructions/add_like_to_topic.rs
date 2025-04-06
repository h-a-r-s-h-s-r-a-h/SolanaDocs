use crate::contexts::*;
use anchor_lang::prelude::*;

pub fn add_like_to_topic(ctx: Context<UpdateTopic>, _topic_id: String) -> Result<()> {
    let topic = &mut ctx.accounts.topic;

    topic.likes = topic.likes.checked_add(1).unwrap();

    Ok(())
}
