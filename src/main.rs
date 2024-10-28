/*
mod etl;
mod crud;
use crud::University;

fn main() {
    // Test ETL
    let url = "http://universities.hipolabs.com/search?country=United+States";

    let mut data = etl::extract(url).unwrap();
    etl::transform(&mut data);
    etl::load(&data).unwrap();

    // Initialize the logger
    crud::init_logger();

    // Test Create
    let university = University {
        name: String::from("Example University"),
        country: String::from("Country"),
        alpha_two_code: String::from("EX"),
        state_province: Some(String::from("State")),
        domains: vec![String::from("example.edu")],
        web_pages: vec![String::from("http://www.example.edu")],
    };

    crud::create(&university).unwrap(); // Directly unwrap to print results

    // Test Read
    //let universities = crud::read().unwrap(); // Directly unwrap to print results
    //println!("Universities: {:?}", universities);

    // Test Update
    let updated_university = University {
        name: String::from("Updated University"),
        country: String::from("New Country"),
        alpha_two_code: String::from("UP"),
        state_province: None,
        domains: vec![String::from("newexample.edu")],
        web_pages: vec![String::from("http://www.newexample.edu")],
    };

    crud::update(1, &updated_university).unwrap(); // Directly unwrap to print results

    // Test Delete
    crud::delete(1).unwrap(); // Directly unwrap to print results

    // Test General Query
    let query = "SELECT * FROM Universities WHERE country='Country'";
    let result = crud::general_query(query).unwrap(); // Directly unwrap to print results
    println!("General Query Result: {:?}", result);
}
*/

//pub mod crud;
//pub mod etl;

mod crud;
mod etl;
use crud::University;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [action]", args[0]);
        return;
    }

    let action = &args[1];
    match action.as_str() {
        "etl" => {
            // ETL Process
            let url = "http://universities.hipolabs.com/search?country=United+States";

            let mut data = etl::extract(url).unwrap();
            etl::transform(&mut data);
            etl::load(&data).unwrap();
            println!("ETL process completed successfully.");
        }
        "create" => {
            // Create University
            let university = University {
                name: String::from("Example University"),
                country: String::from("Country"),
                alpha_two_code: String::from("EX"),
                state_province: Some(String::from("State")),
                domains: vec![String::from("example.edu")],
                web_pages: vec![String::from("http://www.example.edu")],
            };

            crud::create(&university).unwrap();
            println!("University created successfully.");
        }
        "read" => {
            // Read Universities
            let universities = crud::read().unwrap();
            println!("Universities: {:?}", universities);
        }
        "update" => {
            // Update University
            let updated_university = University {
                name: String::from("Updated University"),
                country: String::from("New Country"),
                alpha_two_code: String::from("UP"),
                state_province: None,
                domains: vec![String::from("newexample.edu")],
                web_pages: vec![String::from("http://www.newexample.edu")],
            };

            crud::update(1, &updated_university).unwrap();
            println!("University updated successfully.");
        }
        "delete" => {
            // Delete University
            crud::delete(1).unwrap();
            println!("University deleted successfully.");
        }
        "query" => {
            // General Query
            if let Some(q) = args.get(2) {
                let result = crud::general_query(q).unwrap();
                println!("General Query Result: {:?}", result);
            } else {
                println!("Usage: {} query [SQL query]", args[0]);
            }
        }
        _ => {
            println!(
                "Invalid action. Use 'etl', 'create', 'read', 'update', 'delete', or 'query'."
            );
        }
    }
}

// Tests!
#[cfg(test)]
mod tests {
    use crate::crud::{create, delete, general_query, read, update, University};
    use crate::etl::{extract, load, transform};

    #[test]
    fn test_etl_process() {
        let url = "http://universities.hipolabs.com/search?country=United+States";

        // Extract
        let data = extract(url).unwrap();
        assert!(
            !data.is_empty(),
            "Data should not be empty after extraction."
        );

        // Transform
        let mut transformed_data = data.clone(); // Assuming you want to keep original data
        transform(&mut transformed_data);
        assert!(
            !transformed_data.is_empty(),
            "Transformed data should not be empty."
        );

        // Load
        load(&transformed_data).unwrap();
        println!("ETL process completed successfully.");
    }

    #[test]
    fn test_create_university() {
        let university = University {
            name: String::from("Test University"),
            country: String::from("Test Country"),
            alpha_two_code: String::from("TU"),
            state_province: Some(String::from("Test State")),
            domains: vec![String::from("test.edu")],
            web_pages: vec![String::from("http://www.test.edu")],
        };

        create(&university).unwrap();
        println!("University created successfully.");
    }

    #[test]
    fn test_read_universities() {
        let universities = read().unwrap();
        assert!(
            !universities.is_empty(),
            "There should be at least one university in the database."
        );
        println!("Universities: {:?}", universities);
    }

    #[test]
    fn test_update_university() {
        let updated_university = University {
            name: String::from("Updated Test University"),
            country: String::from("Updated Country"),
            alpha_two_code: String::from("UT"),
            state_province: None,
            domains: vec![String::from("updatedtest.edu")],
            web_pages: vec![String::from("http://www.updatedtest.edu")],
        };

        update(1, &updated_university).unwrap(); // Assuming 1 is a valid ID
        println!("University updated successfully.");
    }

    #[test]
    fn test_delete_university() {
        delete(1).unwrap(); // Assuming 1 is a valid ID for deletion
        println!("University deleted successfully.");
    }

    #[test]
    fn test_query() {
        // General Query
        let select_query = "SELECT * FROM Universities WHERE country='United States'";
        let result = general_query(select_query).unwrap();

        assert!(!result.is_empty(), "Query result should not be empty.");
        println!("General Query Result: {:?}", result);
    }
}
