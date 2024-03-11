use teloxide::prelude::*;
use super::db;
use chrono::{TimeZone, Utc, Duration};

pub async fn send_reply(bot: Bot, msg: Message) {

    let text_message = msg.text().unwrap();
    log::info!("Message received: {:?}", text_message);

    if text_message == "/start" {
        bot.send_message(msg.chat.id, "You are ready for recieving alerts!").await.unwrap();
        // save the chat id in the database
        db::save_chat_db(msg.chat.id).await;
    } else {
        bot.send_message(msg.chat.id, "Just write /start, and wait for updates").await.unwrap();
    }
}

pub async fn check_eq(bot: Bot){
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));
    loop {
        interval.tick().await;
        chech_for_eq(bot.clone()).await;
    }
}

pub async fn chech_for_eq(bot: Bot) {
    // log::info!("I'm alive!");
    let  url = "https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson&eventtype=earthquake&latitude=43.222015&longitude=76.851250&maxradiuskm=1000&orderby=time&minmagnitude=4.0";

    let response = reqwest::get(url).await.unwrap();
    // response json
    let json = response.json::<serde_json::Value>().await.unwrap();
    // log::info!("Response: {:?}", json);
    
    // get first earthquake
    let earthquake = &json["features"][0];
    // log::info!("Earthquake: {:?}", earthquake);
    let properties = &earthquake["properties"];

    let title = properties["title"].as_str().unwrap();
    let magnitude: f64 = properties["mag"].as_f64().unwrap();
    let time: i64 = properties["time"].as_i64().unwrap();
    let time_epoch: i64 = time; // Example UNIX timestamp
    #[allow(deprecated)]
    let date_time = Utc.timestamp(time_epoch, 0) + Duration::hours(5);
    let human_time = date_time.format("%d-%m-%Y %H:%M:%S").to_string();

    let result = db::save_eq_db(title, magnitude, time).await;
    if result == 0 {
        // log::info!("No new earthquakes");
        return;
    }

    db::create_chat_table().await;
    // send message to all chat ids
    let conn = rusqlite::Connection::open("db.sqlite").unwrap();
    let mut stmt = conn.prepare("SELECT chat_id FROM chats").unwrap();
    let chat_ids = stmt.query_map([], |row| {
        let chat_id: String = row.get(0)?;
        Ok(chat_id)
    }).unwrap();

    for chat_id_db in chat_ids {
        log::info!("Sending message to chat id: {:?}", chat_id_db);
        let chat_id = ChatId(chat_id_db.unwrap().parse::<i64>().unwrap()); 
        let text = format!("New earthquake: {}\nMagnitude: {}\nTime: {}", title, magnitude, human_time);
        let _ = bot.send_message(chat_id, text);
    }

}