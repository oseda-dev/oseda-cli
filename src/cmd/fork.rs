use std::error::Error;


pub fn fork() -> Result<(), Box<dyn Error>> {
    println!("Fork url:");
    Ok(())
}
