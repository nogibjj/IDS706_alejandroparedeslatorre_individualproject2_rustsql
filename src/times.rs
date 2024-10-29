use std::time::Instant;
mod crud;

fn main() {
    let start = Instant::now();

    // Run the read operation from your existing CRUD module
    let universities = crud::read().unwrap();
    
    let duration = start.elapsed();
    // Print the timing and a sample of the data as YAML-like output
    println!("data_head: {:?}", universities.get(0)); // Sample head of data
    println!("rust_read_time: {:.6}", duration.as_secs_f64());
}
