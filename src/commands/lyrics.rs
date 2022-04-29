use genius_rs::Genius;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn lyrics(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    dbg!(&args);
    let music = args.single::<String>()?;
    let lyrics = search(music).await;
    msg.channel_id.say(&ctx.http, lyrics).await?;
    Ok(())
}

async fn search(musica: String) -> String {
    let geniusapitoken =
        "Gta3zcHaVeUlMPxgB-52gCEIDSMTmQNPEKmfEdFgI4uJpnVzclyn3YuTi7d0Fjdl".to_string();
    let genius = Genius::new(geniusapitoken);
    let response = genius.search(&musica).await.unwrap();
    let lyrics = genius.get_lyrics(&response[0].result.url).await.unwrap();
    let mut letras = String::new();
    for verse in lyrics {
        letras.push_str(&verse);
    }
    dbg!(&letras);
    return letras;
}
