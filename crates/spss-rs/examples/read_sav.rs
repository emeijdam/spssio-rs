use std::error::Error;
use spssio::SPSSFile;

fn main()->  Result<(), Box<dyn Error>>  {
    let result = readfile();
    match result {
        Ok(_) => println!("File created successfully"),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

fn readfile() -> Result<(), Box<dyn Error>> {
    let filehandle = SPSSFile::open_read("car_sales.sav")?;
    SPSSFile::close_read(&filehandle)?;
    Ok(())
}
