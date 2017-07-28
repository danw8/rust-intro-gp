## Rust
[rust-lang.org]

---

#### What is Rust?

Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.

---

#### Who uses Rust?
* Mozilla (Servo/Firefox) they are also the creator of Rust.
* Microsoft (vscode)
* Dropbox
* Coursera
* npm

---

#### Who cares?
Let's look at some code

---

#### Traditional Hello World

```
fn main() {
	println!("Hello World!");
}
```

---
#### Variable binding

```
let i: i32 = 4;
let x = 5;
```

@[1](Use the **let** keyword.)
@[1](The **: i32** is the explicate type binding)
@[2](Uses implicate bindings also)

---

### Types

---

#### Integers

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| arch   | isize  | usize    |

```
let x: i32 = 5;
```

---

#### Floats

* f64 a 64 bit decimal (default)
* f32 a 32 bit decimal

```
let x: f64 = 5.0;
```

---

#### Strings

* **&str** a pointer to a borrowed string. 
* **String** a struct that contains and helps with string manipulation.

```
let name: &str = "Hedgehogs";
let our_name: String = String::from("Hedgehogs");
```

---

#### Boolean

```
let no: bool = false;
```

---

#### Character

```
let a: char = 'a';
```

---

#### Tuples

```
let my_tuple: (char, bool, &str) = ('A', true, "Hedgehog");
let (a, b, c) = my_tuple;
```

---

#### Arrays

```
let a: [i32; 4] = [1, 2, 3, 4];
```

---

#### Structs

```
struct User{
	pub name: String,
	pub email: String,
	pub enabled: bool,
}

impl User {
	fn new(name: String, email: String) -> User {
		User {
			name: name,
			email: email,
			enable: true,
		}
	}
}
```

---

#### Enums

```
enum Bar {
	X(u32),
	Y,
}

let bar = Bar::X(10);
```
@[2](Enum variants can hold data)

---

#### Vectors

```
let numbers: Vec<i32> = Vec::new();
let names: Vec<&str> = vec!["Bob","Stan","Mike"];
```

---






