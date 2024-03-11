use teloxide::prelude::*;
use log;

mod service;
use service::telegram;

#[tokio::main]
async fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    log::info!("Starting bot...");
    let _ = dotenvy::dotenv();
    let bot: Bot = Bot::from_env();
    tokio::spawn({
        telegram::check_eq(bot.clone())
    });
    // check_eq(bot).await;
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        telegram::send_reply(bot, msg).await;
        Ok(())
    }).await
}