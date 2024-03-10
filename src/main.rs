use teloxide::prelude::*;
use log;
use chrono::{TimeZone, Utc, Duration};

#[tokio::main]
async fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    log::info!("Starting bot...");
    let _ = dotenvy::dotenv();
    let bot: Bot = Bot::from_env();
    check_eq(bot).await;
}

#[allow(dead_code)]
async fn send_reply(bot: Bot, msg: Message) {

    let text_message = msg.text().unwrap();
    log::info!("Message received: {:?}", text_message);

    if text_message == "/start" {
        bot.send_message(msg.chat.id, "You are ready for recieving alerts!").await.unwrap();
        // save the chat id in the database
        save_chat_db(msg.chat.id).await;
    } else {
        bot.send_message(msg.chat.id, "Just write /start, and wait for updates").await.unwrap();
    }
}

async fn create_chat_table() {
    let conn = rusqlite::Connection::open("db.sqlite").unwrap();
    let _result = conn.execute(
        "CREATE TABLE IF NOT EXISTS chats (
            id INTEGER PRIMARY KEY,
            chat_id TEXT NOT NULL
        )",
        [],
    );
    conn.close().unwrap();
}

#[allow(dead_code)]
async fn save_chat_db(chat_id: ChatId) {
    let conn = rusqlite::Connection::open("db.sqlite").unwrap();
    create_chat_table().await;
    // insert the chat id
    conn.execute(
        "INSERT INTO chats (chat_id) VALUES (?1)",
        [chat_id.to_string()],
    ).unwrap();
    conn.close().unwrap();
    log::info!("Chat id saved: {:?}", chat_id);

}

async fn check_eq(bot: Bot){
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));
    loop {
        interval.tick().await;
        chech_for_eq(bot.clone()).await;
    }
}

async fn chech_for_eq(bot: Bot) {
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

    let result = save_eq_db(title, magnitude, time).await;
    if result == 0 {
        log::info!("No new earthquakes");
        return;
    }
    create_chat_table().await;
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
        bot.send_message(chat_id, text).await.unwrap();
    }

}

async fn save_eq_db(title: &str, magnitude: f64, time: i64) -> i8 {
    let conn = rusqlite::Connection::open("db.sqlite").unwrap();
    let _result = conn.execute(
        "CREATE TABLE IF NOT EXISTS earthquakes (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            magnitude REAL NOT NULL,
            time INTEGER NOT NULL,
            time_normalized TEXT NOT NULL
        )",
        [],
    );
    let time_epoch: i64 = time; // Example UNIX timestamp
    #[allow(deprecated)]
    let date_time = Utc.timestamp(time_epoch, 0) + Duration::hours(5);
    let human_time = date_time.format("%d-%m-%Y %H:%M:%S").to_string();
    
    // select last earthquake and compare with the new one by time
    let mut stmt = conn.prepare("SELECT time FROM earthquakes ORDER BY time DESC LIMIT 1").unwrap();
    let last_eq_time = stmt.query_row([], |row| {
        let time: i64 = row.get(0)?;
        Ok(time)
    }).unwrap_or(0);

    if time == last_eq_time && last_eq_time != 0{
        log::info!("No new earthquakes");
        return 0;
    }

    conn.execute(
        "INSERT INTO earthquakes (title, magnitude, time, time_normalized) VALUES (?1, ?2, ?3, ?4)",
        [title, magnitude.to_string().as_str(), time.to_string().as_str(), human_time.as_str()],
    ).unwrap();

    log::info!("Earthquake saved: {:?}", title);
    1
}