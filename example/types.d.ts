type HashSet<T extends number | string> = Record<T, undefined>;
type HashMap<T extends number | string, U> = Record<T, U>;
type Vec<T> = Array<T>;
type Option<T> = T | undefined;
type Result<T, U> = T | U;
export type NumberAlias = number;
export type TupleAlias = [number, string];
export type Color =
| { name: "Red", type: number }
| { name: "Green", type: number }
| { name: "Blue", type: string }
;
export interface Person {
  name: string;
  age: number;
  enjoy_tea: boolean;
}
export interface ComplexType {
  color_map: HashMap<string, Color>;
  list_of_names: Vec<string>;
  optional_person: Option<Person>;
}
