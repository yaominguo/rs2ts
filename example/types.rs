type NumberAlias = i32;

type NumberAlias2 = u32;

#[serde(tag = "t", content = "c")]
enum Color {
    Red(undefined),
    Green(i32),
    Blue(str),
}
