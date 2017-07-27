
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

@[1](Immutable by default)
@[1-2](error[E0384]: re-assignment of immutable variable 'x')

---?code=src/bin/immutable_by_default.rs&lang=rust

@[2](The **mut** keyword makes a variable mutable)
@[1-6](Ouput <br/> `The value of x is: 5` <br/> `The value of x is: 6`)





