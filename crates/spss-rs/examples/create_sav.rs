use std::error::Error;
use spssio::SPSSFile;


fn main() {
    let _ = mynewfile();
}

fn mynewfile() -> Result<(), Box<dyn Error>> {
    let myfile = SPSSFile::open_write("output.sav")?;

    println!("handle");

    myfile.set_var_name("var1", 0)?;
    myfile.set_var_label("var1", "Variable 1")?;
    myfile.set_var_name("var2", 30)?;
    myfile.set_var_label("var2", "Variable 2")?;

    myfile.commit_header()?;

    println!("header");
    myfile.set_value_numeric("var1", 123.45)?;


    myfile.set_value_char("var2", "Hello")?;
   
    println!("before commit");
    myfile.commit_case_record()?;

    println!("case");
    myfile.set_value_numeric("var1", 678.90)?;
    myfile.set_value_char("var2", "World")?;
    myfile.commit_case_record()?;

    myfile.close_write()?;

    Ok(())
}
