# rs2ts

A command line tool to convert Rust types to Typescript types.


### Usage

Git clone project and then
```bash
cargo run -- --input=example/types.rs --output=example/types/d.ts
```
or
```bash
cargo build --release && target/release/rs2ts --input=example --output=example/types.d.ts
```
