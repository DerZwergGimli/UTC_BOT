use std::sync::Arc;

use chrono::*;
use serenity::framework::standard::{CommandResult, macros::command};
use serenity::http::Http;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::info;

#[command]
pub async fn time(ctx: &Context, msg: &Message) -> CommandResult {
    let time = chrono::offset::Utc::now();

    let time_string = format!("TIME IS {:?}", time).to_string();

    msg.channel_id.say(&ctx.http, time_string).await?;

    Ok(())
}

pub async fn change_display_name(ctx: &Arc<Http>, guilds: &Vec<GuildId>) -> CommandResult {

    for guild in guilds {
        let datetime: DateTime<Utc> = DateTime::from(chrono::Utc::now());
        let time = datetime.format("%d. %b %H:%M UTC");
        let time_string = format!("{}", time);


        guild.edit_nickname(ctx, Some(&*time_string)).await?;
    }
    Ok(())
}
