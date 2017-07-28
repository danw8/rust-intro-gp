# Rust
### [rust-lang.org](rust-lang.org)

---

# Features

* zero-cost abstractions
* move semantics
* guaranteed memory safety
* threads without data races
* trait-based generics
* pattern matching
* type inference
* minimal runtime
* efficient C bindings

---

# Why Rust

* Low level
* Memory safe
* Modern syntax
* High performance
* No garbage collection
* Powerful abstractions

---

# What is Rust good at?

* Kernels/operating systems
* Device drivers
* Web applications
* Games
* Etc...

---

# Installation

* Windows
  * [https://rustup.rs](https://rustup.rs)
  * download and run rustup-init.exe

* Linux/Mac
  * `curl https://sh.rustup.rs -sSf | sh`

* For all platforms update Rust
  * `rustup update`


---?code=src/bin/hello_world.rs&lang=rust

@[1-3](Hello World!)
@[1](Function Definitions)
@[2](Macros)

---

```
fn main() {
  let x = 5;
  x = 6;
}
```

@[2](Immutable by default)
@[3](error[E0384]: re-assignment of immutable variable 'x')

---?code=src/bin/immutable_by_default.rs&lang=rust

@[2](The **mut** keyword makes a variable mutable)
@[1-6](Ouput <br/> `The value of x is: 5` <br/> `The value of x is: 6`)

---?code=src/bin/shadowing.rs&lang=rust

@[2-3](using **let** again makes a shadowed variable)
@[3-4](A shadowed variable is a brand new variable with the same name)
@[1-6](Output <br/> `The value of x is: 12`)

---?code=src/bin/shadowingagain.rs&lang=rust

@[2-3](You can shadow a variable to a new type)
@[2-3](Which is useful if you want the same name but a different type.)
@[1-6](Output <br/> `The value of x is: five`)

---

# Rust Data Types

---

# Integers

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| arch   | isize  | usize    |

---?code=src/bin/integers.rs&lang=rust

@[2-3](Default integer is i32)
@[4-5](Type suffix 2**u8** of literal)
@[6-7](Hex **0xf**i16 for a 16 bit integer whose value is 15)
@[8-9](Octal can also be used **0o77**i64)
@[10-11](Or Binary **0b1111_0000**i32)
@[12-13](Or a byte array **b'A'** _u8 only_)

---?code=src/bin/floats.rs&lang=rust

@[1-5](Floating Point Types)
@[2](Default float is 64-bit float or **f64**)
@[2](The type can be infered)
@[3](You can make a float 32-bits or **f32**)
@[3](You can use the suffix to declare the type.)

---?code=src/bin/types.rs&lang=rust

@[1-8](Other Types)
@[2](Booleans)
@[3](Characters)
@[4](String references)
@[5-6](Tuples)
@[7](Output <br/> `The value in the tuple is: A true Hedgehog`)

---?code=src/bin/strings.rs&lang=rust

@[1-6](Rust has two string types &str and String)
@[2-3](This is a &str. It is a pointer to a String)
@[4-5](This is a String, It is not a pointer)
@[1-6](Rust need the two types of Strings for ownership reasons, More on ownership later.)

---?code=src/bin/arrays.rs&lang=rust

@[2-3](Rust has Arrays)
@[4-5](Access an arrays element)
@[6](Output <br/> `The value of the second index of array: 2`)

---?code=src/bin/explicatedeclaration.rs&lang=rust
@[1-9](You can also explicatly declare variables)

---?code=src/bin/functionbasics.rs&lang=rust

@[1-10](Functions)
@[7-9](Declaration)
@[7](return value types **-> i32** this is required you cannot have an implicate return value type)
@[8](The use of the **return** keyword is optional. notice there is no semi-colon)
@[8](You could rewrite this to **return x + 1;** it's legal but considered bad form in Rust)

---?code=src/unittypefunction.rs&lang=rust
@[1-3](This function returns a **`()`** unit type)
@[1-3](the unit type is simaler to void in other languages)
@[1](You do not need to specify the type for a funciton that returns a unit)

--- 

# Comments
#### C style
```
// This is a comment
```

---?code=src/bin/control.rs&lang=rust

@[3-5](if expression)
@[6](Expression can be assigned to variables)
@[7-10](while loop)
@[11-14](for loops use iterators)
@[15-17](generic loop, it never ends unless you **break** it)

---?code=src/bin/namedloops.rs&lang=rust

@[3](This is a named loop)
@[6]( So is this.)
@[9-10](You can break named loops directly)

---

Output 
```
In outer loop, x's value is: 1
In outer loop, x's value is: 2
In outer loop, x's value is: 3
In inner loop, x's value is: 4
In outer loop, x's value is: 5
In inner loop, x's value is: 6
In outer loop, x's value is: 7
In inner loop, x's value is: 8
```

---

# Ownership

Ownership is the most unique feature in rust  
It is what enable memory safety without the need for garbage collection

---

#### Variable scope

```
{
    let s = "hello"
    // do stuff with s here
}
```

@[1](_s_ is not declared yet and is not valid)
@[2](_s_ is declare and is no valid)
@[3]( do stuff with _s_ here)
@[4](_s_ goes out of scope and the **drop** method is called)
@[1-4](This is similar to other languages)

---

#### Copy

```
{
    let s1 = "hi";
    let s2 = s1;
    println!("{}{}", s1, s2);
}
```
@[2](string literals are created on the stack)
@[3](stack variable are a copy type)
@[4](both _s1_ and _s2_ are valid after a copy)

---
#### Move
```
{
    let s1 = String::from("hi");
    let s2 = s1;
    println!("{}{}", s2);
}
```
@[1-5](This looks similar to the previous example)
@[2](The String type is heap allocated)
@[3](Heap allocated value are move by default)
@[3]( _s1_ moved into _s2_ here and is no longer valid)
@[4]( only _s2_ is valid here)

---
#### Clone
```
{
    let s1 = String::from("hi");
    let s2 = s1.clone();
    println!("{}{}", s1, s2);
}
```
@[3](Here we use the clone method to do a deep copy of _s1_ on the heap)
@[4](_s1_ and _s2_ now point to different memory on the heap with the same data)
@[4](we can now use both s1 and s2)

---


















