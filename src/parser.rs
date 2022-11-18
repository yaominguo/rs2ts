pub fn parse_item_type(item_type: &syn::ItemType) -> String {
    let mut output_text = String::new();

    let type_string = parse_type(&item_type.ty);

    let text = format!("export type {} = {};\n", item_type.ident, &type_string);

    output_text.push_str(&text);

    output_text
}

pub fn parse_item_enum(item_enum: &syn::ItemEnum) -> String {
    let mut output_text = String::new();

    output_text.push_str(&format!("export type {} =\n", item_enum.ident));

    item_enum.variants.iter().for_each(|var| {
        output_text.push_str(&format!("| {{ t: \"{}\", c: ", var.ident));

        match &var.fields {
            syn::Fields::Named(named_fields) => {
                output_text.push('{');
                named_fields.named.iter().for_each(|field| {
                    if let Some(ident) = &field.ident {
                        output_text.push_str(&format!(" {}: {};", ident, parse_type(&field.ty)));
                    }
                });
                output_text.push('}')
            }
            syn::Fields::Unnamed(unnamed_fields) => {
                let unnamed_field = unnamed_fields.unnamed.first().unwrap();
                output_text.push_str(&parse_type(&unnamed_field.ty));
            }
            syn::Fields::Unit => {
                output_text.push_str("undefined");
            }
        }
        output_text.push_str(" }\n")
    });
    output_text.push_str(";\n");

    output_text
}

pub fn parse_item_struct(item_struct: &syn::ItemStruct) -> String {
    let mut output_text = String::new();

    output_text.push_str(&format!("export interface {} {{\n", item_struct.ident));

    match &item_struct.fields {
        syn::Fields::Named(named_fields) => {
            named_fields.named.iter().for_each(|named_field| {
                if let Some(ident) = &named_field.ident {
                    output_text.push_str(&format!("  {}: ", ident));
                }
                output_text.push_str(&format!("{};\n", parse_type(&named_field.ty)));
            });
        }
        syn::Fields::Unnamed(unnamed_fields) => {
            unnamed_fields
                .unnamed
                .iter()
                .enumerate()
                .for_each(|(index, unnamed_field)| {
                    output_text.push_str(&format!(
                        "{}: {};\n",
                        index,
                        parse_type(&unnamed_field.ty)
                    ));
                })
        }
        syn::Fields::Unit => (),
    }
    output_text.push_str("}\n");

    output_text
}

fn parse_type(syn_type: &syn::Type) -> String {
    let mut output_text = String::new();

    match syn_type {
        syn::Type::Path(type_path) => {
            let seg = type_path.path.segments.last().unwrap();

            let field_type = seg.ident.to_string();

            let ts_field_type = parse_type_ident(&field_type).to_owned();

            output_text.push_str(&ts_field_type);

            match &seg.arguments {
                syn::PathArguments::AngleBracketed(angle_bracket_args) => {
                    output_text.push('<');
                    angle_bracket_args
                        .args
                        .iter()
                        .enumerate()
                        .for_each(|(index, arg)| {
                            if let syn::GenericArgument::Type(inner_type) = arg {
                                output_text.push_str(&parse_type(inner_type));
                                if index < angle_bracket_args.args.iter().len() - 1 {
                                    output_text.push_str(", ");
                                }
                            } else {
                                dbg!("Unimplemented token");
                            }
                        });

                    output_text.push('>');
                }
                syn::PathArguments::None => {}
                _ => {
                    dbg!("Unimplemented token");
                }
            }
        }
        syn::Type::Tuple(type_tuple) => {
            output_text.push('[');

            type_tuple.elems.iter().enumerate().for_each(|(index, el)| {
                output_text.push_str(&parse_type(el));
                if index < type_tuple.elems.iter().len() - 1 {
                    output_text.push_str(", ");
                }
            });

            output_text.push(']');
        }
        _ => {
            dbg!("Unimplemented token");
        }
    }

    output_text
}

pub fn parse_type_ident(ident: &str) -> &str {
    match ident {
        "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64"
        | "isize" | "usize" => "number",
        "str" | "String" | "char" => "string",
        "bool" => "boolean",
        _ => ident,
    }
}
