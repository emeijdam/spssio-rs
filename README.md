# SPSSIO-RS

Ergonomic Rust bindings for the SPSSIO C Module, generated using the excelent rust [bindgen](https://github.com/rust-lang/rust-bindgen).

Inspired by the duckdb-rs crate.

[IBM SPSS Statistics 29 IO Module](https://community.ibm.com/community/user/ai-datascience/viewdocument/extensions-tools-and-utilities-for?CommunityKey=886b6874-0fb1-402c-8243-c70ef8179a99&tab=librarydocuments)

## Usage

```rust
use std::error::Error;
use spssio::SPSSFile;

fn main()-> Result<(), Box<dyn Error>>{
    let myfile = SPSSFile::open_write("output.sav")?;

    myfile.set_var_name("var1", 0)?;
    myfile.set_var_label("var1", "Variable 1")?;

    myfile.commit_header()?;

    myfile.set_value_numeric("var1", 123.45)?;
    myfile.commit_case_record()?;
    
    myfile.set_value_numeric("var1", 678.90)?;
    myfile.commit_case_record()?;

    myfile.close_write()?;

}
```

More examples can be found in the examples folder.

# Status
This is a work in progress, but the basic functionality for writing and reading SPSS files is working for OSX Rosetta and Windows.

Need something? Open an issue or just contact me somehow on linkedin.

## Why Rust?
GenAi: Modern data science and machine learning applications require a high-performance, safe, and reliable programming language. Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. Rust is a great choice for building high-performance applications that require low-level control over system resources. 

Me the dev: Cause I like Rust programming, especially with the great Cargo Toolchain. It makes programming fun again.


## Why SPSSIO? 

### Use Cases
Embed SPSS data files in your applications, such as data collection, survey platforms.
Read SPSS Metadata and Data from SPSS data files.
Write SPSS Metadata and Data to SPSS data files.
Convert SPSS data files to other formats, such as CSV, JSON, Parquet, etc.
Convert other data formats to SPSS data files.
Analyze SPSS data files in your applications, such as data mining, machine learning, statistical analysis, etc.
Visualize SPSS data files in your applications, such as data visualization, reporting, dashboarding, etc.
Integrate SPSS data files with other data sources, such as databases, data lakes, data warehouses, etc.
Automate SPSS data processing tasks, such as data import, data export, data transformation, data cleaning, data validation, etc.
Build SPSS data processing pipelines in your applications, such as ETL, ELT, data integration, data migration, data synchronization, etc.
Develop SPSS data processing workflows in your applications, such as data preparation, data analysis, data modeling, data visualization, etc.
Deploy SPSS data processing services in your applications, such as data APIs, data services, data pipelines, data workflows, etc.
Scale SPSS data processing applications in your applications, such as data processing, data analytics, data science, etc.§




## Installation    

```toml 

[dependencies]
spssio_rs = "0.1.0"

```                         

## License 



## Building on and for OSX.
As far I know, IBM is working on a IBM SPSS v30 version for Apple Silicon, meanwhile we compile for Rosetta.


cargo run --target x86_64-apple-darwin    
cargo build --target x86_64-apple-darwin   

cargo build -vv --target x86_64-apple-darwin  


## Building for Linux
https://blog.krzyzanowskim.com/2018/12/05/rpath-what/
install_name_tool -add_rpath @executable_path SPSS-RS


https://doc.rust-lang.org/cargo/reference/publishing.html

export RUSTFLAGS="-C link-args=-Wl,-rpath,@executable_path"
cargo build --target x86_64-apple-darwin

cargo run --example create_sav --target x86_64-apple-darwin

// set LD_LIBRARY_PATH
