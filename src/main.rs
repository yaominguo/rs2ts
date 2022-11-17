use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use clap::{Arg, Command};

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
                .help("The Rust file need to covert"),
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
        .expect("Please type the rust filename");
    let output_filename = matches
        .get_one::<String>("output")
        .expect("Please type the typescript filename");
    dbg!(input_filename);
    dbg!(output_filename);

    let input_path = Path::new(input_filename);

    let mut input_file = File::open(input_path)
        .unwrap_or_else(|_| panic!("Unable to open file {}", input_path.display()));

    let mut input_file_text = String::new();

    input_file
        .read_to_string(&mut input_file_text)
        .unwrap_or_else(|_| panic!("Unable to read file {}", input_path.display()));

    let input_syntax: syn::File = syn::parse_file(&input_file_text)
        .unwrap_or_else(|_| panic!("Unable to parse file {}", input_path.display()));

    let mut output_text = String::new();

    for item in input_syntax.items.iter() {
        if let syn::Item::Type(item_type) = item {
            let type_text = parse_item_type(item_type);
            output_text.push_str(&type_text);
        } else {
            dbg!("Unimplemented type!");
        }
    }

    let mut output_file = File::create(output_filename).unwrap();
    write!(output_file, "{}", output_text)
        .unwrap_or_else(|_| panic!("Failed to write output file {}", output_filename));
}

fn parse_item_type(item_type: &syn::ItemType) -> String {
    let mut output_text = String::new();

    let type_string = parse_type(&item_type.ty);

    let text = format!("export type {} = {};\n", &item_type.ident, &type_string);

    output_text.push_str(&text);

    output_text
}

fn parse_type(syn_type: &syn::Type) -> String {
    let mut output_text = String::new();
    if let syn::Type::Path(type_path) = syn_type {
        let seg = type_path.path.segments.last().unwrap();

        let field_type = seg.ident.to_string();

        let ts_field_type = parse_type_ident(&field_type).to_owned();

        output_text.push_str(&ts_field_type);

        if let syn::PathArguments::None = &seg.arguments {
        } else {
            dbg!("Unimplemented token");
        }
    } else {
        dbg!("Unimplemented token");
    }
    output_text
}

fn parse_type_ident(ident: &str) -> &str {
    match ident {
        "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64"
        | "isize" | "usize" => "number",
        "str" | "String" | "char" => "string",
        "bool" => "boolean",
        _ => ident,
    }
}