use crate::parser;
use std::{
    fs::{self, metadata, File},
    io::{Read, Write},
    path::Path,
};

pub struct Processor {
    input: String,
    output: String,
}

impl Processor {
    pub fn new(input_filename: &str, output_filename: &str) -> Self {
        Self {
            input: input_filename.to_owned(),
            output: output_filename.to_owned(),
        }
    }

    pub fn convert(&self) {
        if self.is_dir() {
            self.convert_from_dir()
        } else {
            self.convert_from_file()
        }
    }

    fn convert_from_dir(&self) {
        let mut contents: Vec<syn::File> = vec![];
        fs::read_dir(&self.input)
            .unwrap()
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_str().unwrap().ends_with(".rs"))
            .for_each(|entry| {
                println!("{}", entry.file_name().to_str().unwrap());
                contents.push(Self::read_file(&entry.path().display().to_string()));
            });
        let mut total_content = String::new();

        contents.into_iter().for_each(|content| {
            total_content.push_str(&Self::convert_data(content));
        });

        total_content.push_str(&self.initial_types());

        self.write_file(total_content);
    }

    fn convert_from_file(&self) {
        let content = Self::read_file(&self.input);

        let mut content = Self::convert_data(content);

        content.push_str(&self.initial_types());

        self.write_file(content);
    }

    fn is_dir(&self) -> bool {
        let md = metadata(&self.input).unwrap();
        md.is_dir()
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

    fn initial_types(&self) -> String {
        let mut output_text = String::new();

        output_text.push_str("type HashSet<T extends number | string> = Record<T, undefined>;\n");
        output_text.push_str("type HashMap<T extends number | string, U> = Record<T, U>;\n");
        output_text.push_str("type Vec<T> = Array<T>;\n");
        output_text.push_str("type Option<T> = T | undefined;\n");
        output_text.push_str("type Result<T, U> = T | U;\n");

        output_text
    }

    fn write_file(&self, content: String) {
        let mut output_file = File::create(&self.output).unwrap();

        write!(output_file, "{}", content)
            .unwrap_or_else(|_| panic!("Failed to write output file {}", &self.output));

        println!("Convert Complete: {}", &self.output);
    }
}
