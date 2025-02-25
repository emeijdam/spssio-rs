# SPSSIO_RS

Ergonomic Rust bindings for the SPSSIO library, kindly privided by IBM.

## Usage

```rust
use spssio_rs::SpssIO;

fn main() {
    let spss_io = SpssIO::new();
    let file = spss_io.open("data.sav");
    let data = file.read();
    println!("{:?}", data);
}
```

## Installation    

```toml 

[dependencies]
spssio_rs = "0.1.0"

```                         

## License 




export CARGO_BUILD_TARGET="x86_64-apple-darwin"


cargo run --target x86_64-apple-darwin    

cargo build --target x86_64-apple-darwin   

cargo build -vv --target x86_64-apple-darwin  


https://blog.krzyzanowskim.com/2018/12/05/rpath-what/
install_name_tool -add_rpath @executable_path SPSS-RS


https://doc.rust-lang.org/cargo/reference/publishing.html

export RUSTFLAGS="-C link-args=-Wl,-rpath,@executable_path"
cargo build --target x86_64-apple-darwin

cargo run --example create_sav --target x86_64-apple-darwin

// set LD_LIBRARY_PATH
