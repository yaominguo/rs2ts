mod parser;
use clap::{Arg, Command};
use std::{
    fs::{self, metadata, File},
    io::{Read, Write},
    path::Path,
};

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

    if is_dir(input_filename) {
        convert_from_dir(input_filename, output_filename);
    } else {
        convert_from_file(input_filename, output_filename);
    }
}

fn convert_from_dir(input_filename: &str, output_filename: &str) {
    let mut contents: Vec<syn::File> = vec![];
    fs::read_dir(input_filename)
        .unwrap()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_str().unwrap().ends_with(".rs"))
        .for_each(|entry| {
            println!("{}", entry.file_name().to_str().unwrap());
            contents.push(read_file(&entry.path().display().to_string()));
        });
    let mut total_content = String::new();

    contents.into_iter().for_each(|content| {
        total_content.push_str(&convert_data(content));
    });

    total_content.push_str(&initial_types());

    write_file(output_filename, total_content);
}

fn convert_from_file(input_filename: &str, output_filename: &str) {
    let content = read_file(input_filename);

    let mut content = convert_data(content);

    content.push_str(&initial_types());

    write_file(output_filename, content);
}

fn read_file(input_filename: &str) -> syn::File {
    let input_path = Path::new(input_filename);

    let mut input_file = File::open(input_path)
        .unwrap_or_else(|_| panic!("Unable to open file {}", input_path.display()));

    let mut input_file_text = String::new();

    input_file
        .read_to_string(&mut input_file_text)
        .unwrap_or_else(|_| panic!("Unable to read file {}", input_path.display()));

    let input_syntax: syn::File = syn::parse_file(&input_file_text)
        .unwrap_or_else(|_| panic!("Unable to parse file {}", input_path.display()));

    input_syntax
}

fn convert_data(input_syntax: syn::File) -> String {
    let mut output_text = String::new();

    input_syntax.items.iter().for_each(|item| {
        match item {
            syn::Item::Type(item_type) => {
                let type_text = parser::parse_item_type(item_type);
                output_text.push_str(&type_text);
            }
            syn::Item::Enum(item_enum) => {
                let enum_text = parser::parse_item_enum(item_enum);
                output_text.push_str(&enum_text);
            }
            syn::Item::Struct(item_struct) => {
                let struct_text = parser::parse_item_struct(item_struct);
                output_text.push_str(&struct_text);
            }
            _ => {
                dbg!("Unimplemented type!");
            }
        };
    });

    output_text
}

fn write_file(output_filename: &str, content: String) {
    let mut output_file = File::create(output_filename).unwrap();

    write!(output_file, "{}", content)
        .unwrap_or_else(|_| panic!("Failed to write output file {}", output_filename));

    println!("Convert Complete: {}", output_filename);
}

fn initial_types() -> String {
    let mut output_text = String::new();

    output_text.push_str("type HashSet<T extends number | string> = Record<T, undefined>;\n");
    output_text.push_str("type HashMap<T extends number | string, U> = Record<T, U>;\n");
    output_text.push_str("type Vec<T> = Array<T>;\n");
    output_text.push_str("type Option<T> = T | undefined;\n");
    output_text.push_str("type Result<T, U> = T | U;\n");

    output_text
}

fn is_dir(path: &str) -> bool {
    let md = metadata(path).unwrap();
    md.is_dir()
}
