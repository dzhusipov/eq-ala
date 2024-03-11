use chrono::{TimeZone, Utc, Duration};
use teloxide::types::ChatId;

pub async fn create_chat_table() {
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

pub async fn save_chat_db(chat_id: ChatId) {
    let conn = rusqlite::Connection::open("db.sqlite").unwrap();
    create_chat_table().await;

    // Check if the chat_id already exists
    let mut check_stmt = conn.prepare("SELECT COUNT(*) FROM chats WHERE chat_id = ?1").unwrap();
    let chat_id_count: i64 = check_stmt.query_row([chat_id.to_string()], |row| row.get(0)).unwrap_or(0);

    if chat_id_count == 0 {
        // If the chat_id does not exist, insert it
        conn.execute(
            "INSERT INTO chats (chat_id) VALUES (?1)",
            [chat_id.to_string()],
        ).unwrap();
        log::info!("Chat id saved: {:?}", chat_id);
    } else {
        log::info!("Chat id already exists: {:?}", chat_id);
    }

    // It's a good practice to explicitly close the connection, but it's not strictly necessary here
    // as the connection will be closed when it goes out of scope
    // conn.close().unwrap();
}

pub async fn save_eq_db(title: &str, magnitude: f64, time: i64) -> i8 {
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