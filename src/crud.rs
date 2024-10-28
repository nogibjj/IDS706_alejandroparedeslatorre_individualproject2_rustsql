use log::info; // Removed unused `error`
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct University {
    pub name: String,
    pub country: String,
    pub alpha_two_code: String,
    #[serde(rename = "state-province")]
    pub state_province: Option<String>,
    pub domains: Vec<String>,
    pub web_pages: Vec<String>,
}

const DB_PATH: &str = "./src/data/UniversitiesDB.db";

pub fn init_logger() {
    env_logger::builder().format_timestamp(None).init();
}

pub fn create(university: &University) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(DB_PATH)?;
    conn.execute(
        "INSERT INTO Universities (name, country, alpha_two_code, state_province, domains, web_pages) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            university.name,
            university.country,
            university.alpha_two_code,
            university.state_province,
            serde_json::to_string(&university.domains)?,
            serde_json::to_string(&university.web_pages)?,
        ],
    )?;
    info!("Inserted a new university: {:?}", university);
    println!("Inserted a new university: {:?}", university);
    Ok(())
}

pub fn read() -> Result<Vec<University>, Box<dyn Error>> {
    let conn = Connection::open(DB_PATH)?;
    let mut stmt = conn.prepare("SELECT * FROM Universities")?;
    let universities = stmt
        .query_map([], |row| {
            Ok(University {
                name: row.get(1)?,
                country: row.get(2)?,
                alpha_two_code: row.get(3)?,
                state_province: row.get(4)?,
                domains: serde_json::from_str(&row.get::<_, String>(5).unwrap()).unwrap(),
                web_pages: serde_json::from_str(&row.get::<_, String>(6).unwrap()).unwrap(),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    info!("Successfully read table Universities");
    Ok(universities)
}

pub fn update(record_id: i32, university: &University) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(DB_PATH)?;
    conn.execute(
        "UPDATE Universities SET name=?1, country=?2, alpha_two_code=?3, state_province=?4, domains=?5, web_pages=?6 WHERE id=?7",
        params![
            university.name,
            university.country,
            university.alpha_two_code,
            university.state_province,
            serde_json::to_string(&university.domains)?,
            serde_json::to_string(&university.web_pages)?,
            record_id,
        ],
    )?;
    info!("Updated university with id: {}", record_id);
    println!("Updated university with id: {}", record_id);
    Ok(())
}

pub fn delete(record_id: i32) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(DB_PATH)?;
    conn.execute("DELETE FROM Universities WHERE id=?", params![record_id])?;
    info!("Deleted university with id: {}", record_id);
    println!("Deleted university with id: {}", record_id);
    Ok(())
}

pub fn general_query(query: &str) -> Result<Vec<University>, Box<dyn Error>> {
    let conn = Connection::open(DB_PATH)?;
    let mut stmt = conn.prepare(query)?;
    let result = stmt
        .query_map([], |row| {
            Ok(University {
                name: row.get(1)?,
                country: row.get(2)?,
                alpha_two_code: row.get(3)?,
                state_province: row.get(4)?,
                domains: serde_json::from_str(&row.get::<_, String>(5).unwrap()).unwrap(),
                web_pages: serde_json::from_str(&row.get::<_, String>(6).unwrap()).unwrap(),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    info!("Successfully executed query: {}", query);
    Ok(result)
}
