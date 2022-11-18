type NumberAlias = i32;

type TupleAlias = (i32, str);

#[serde(tag = "name", content = "value")]
enum Color {
    Red(u32),
    Green(i32),
    Blue(str),
}

struct Person {
    name: String,
    age: u32,
    enjoy_tea: bool,
}

struct ComplexType {
    color_map: HashMap<String, Color>,
    list_of_names: Vec<String>,
    optional_person: Option<Person>,
}
