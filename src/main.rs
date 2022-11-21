mod parser;
mod processor;
use clap::{Arg, Command};
use processor::Processor;

fn main() {
    let matches = Command::new("rs2ts")
        .version("0.1.0")
        .author("Max Kwok")
        .about("Convert Rust types to Typescript types~")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .help("The Rust file or directory need to covert"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .required(true)
                .help("The filename of Typescript need to output"),
        )
        .get_matches();

    let input_filename = matches
        .get_one::<String>("input")
        .expect("Please type the rust filename or directory path");

    let output_filename = matches
        .get_one::<String>("output")
        .expect("Please type the typescript filename");

    let processor = Processor::new(input_filename, output_filename);
    processor.convert();
}
