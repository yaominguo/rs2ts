export type NumberAlias = number;
export type Color =
| { t: "Red", c: undefined }
| { t: "Green", c: number }
| { t: "Blue", c: string }
;
export interface Person {
  name: string;
  age: number;
  enjoy_tea: boolean;
}
