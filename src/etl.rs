use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct University {
    pub name: String,
    pub country: String,
    pub alpha_two_code: String,
    #[serde(rename = "state-province")]
    pub state_province: Option<String>,
    pub domains: Vec<String>,
    pub web_pages: Vec<String>,
}

pub fn extract(url: &str) -> Result<Vec<University>, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;
    if response.status().is_success() {
        let universities: Vec<University> = response.json()?;
        println!(
            "Data extracted successfully. Sample data: {:?}",
            &universities[..5.min(universities.len())]
        );
        Ok(universities)
    } else {
        Err(format!("Failed to fetch data. Status code: {}", response.status()).into())
    }
}

pub fn transform(data: &mut Vec<University>) {
    println!("Transforming data...");
    // Transformation logic goes here if needed
}

pub fn load(data: &[University]) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("./src/data/UniversitiesDB.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Universities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            country TEXT NOT NULL,
            alpha_two_code TEXT NOT NULL,
            state_province TEXT,
            domains TEXT NOT NULL,
            web_pages TEXT NOT NULL
        )",
        [],
    )?;

    for university in data {
        conn.execute(
            "INSERT INTO Universities (name, country, alpha_two_code, state_province, domains, web_pages) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                university.name,
                university.country,
                university.alpha_two_code,
                university.state_province,
                serde_json::to_string(&university.domains)?,
                serde_json::to_string(&university.web_pages)?
            ],
        )?;
    }

    println!("Data loaded into UniversitiesDB.db");

    // Verifying data
    let mut stmt = conn.prepare("SELECT * FROM Universities")?;
    let universities = stmt.query_map([], |row| {
        Ok(University {
            name: row.get(1)?,
            country: row.get(2)?,
            alpha_two_code: row.get(3)?,
            state_province: row.get(4)?,
            domains: serde_json::from_str(&row.get::<_, String>(5).unwrap()).unwrap(),
            web_pages: serde_json::from_str(&row.get::<_, String>(6).unwrap()).unwrap(),
        })
    })?;

    for university in universities {
        println!("{:?}", university?);
    }
    Ok(())
}
