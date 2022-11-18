type NumberAlias = i32;

#[serde(tag = "t", content = "c")]
enum Color {
    Red(undefined),
    Green(i32),
    Blue(str),
}

struct Person {
    name: String,
    age: u32,
    enjoy_tea: bool,
}
