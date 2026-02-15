## 1. Crate

A crate is the smallest unit of compilation in Rust.

Two types:

- Binary crate → has `main.rs`
    
- Library crate → has `lib.rs`
    

In one package you can have:

- One library crate
    
- One or more binary crates
    

If your `Cargo.toml` says:

```toml
name = "my-project"
```

Then the crate name in code becomes:

```rust
my_project
```

Hyphens become underscores in Rust code.

---

## 2. Package

A package:

- Contains `Cargo.toml`
    
- Can contain multiple crates
    
- Is managed by Cargo
    

Cargo automatically:

- Compiles `lib.rs` as library crate
    
- Compiles `main.rs` as binary crate
    
- Links binary to library
    

---

## 3. Module

A module is a namespace inside a crate.

Modules organize code and control visibility.

Modules must be explicitly declared.

Files do not automatically become modules.

Example structure:

```
src/
├── lib.rs
└── garden/
    └── vegetable.rs
```

You must declare modules:

```rust
// lib.rs
pub mod garden;

// garden.rs or garden/mod.rs
pub mod vegetable;
```

Rust builds a module tree from the crate root downward.

---

## 4. Module Tree

Given:

```rust
pub mod garden;
```

and

```rust
pub mod vegetable;
```

The tree becomes:

```
crate
 └── garden
       └── vegetable
```

Each level must be declared using `mod`.

Rust never auto-discovers modules from folders.

---

## 5. Visibility Rules

Everything is private by default.

To access something from outside its module:

- The module must be `pub`
    
- The item must be `pub`
    

Visibility is hierarchical.

If a parent module is private, children cannot be accessed even if they are public.

---

## 6. use Keyword

`use` does not import code like Python.

It creates a local alias for a path.

This:

```rust
use std::time::SystemTime;
```

is equivalent to writing:

```rust
std::time::SystemTime
```

everywhere.

`use` is local to the module it is written in.

It does not propagate into child modules.

---

## 7. Common Mistakes I Made

### 1. Thinking use works like Python import

Wrong mental model:

```python
from lib import time1
```

Rust equivalent does not exist.

Correct model:

```rust
use my_project::timings::time1;
```

Importing always starts from the crate name.

---

### 2. Expecting outer use to work inside modules

This does not work:

```rust
use std::time::SystemTime;

pub mod timings {
    pub fn now() -> SystemTime { ... } // error
}
```

Because `use` is scoped to the current module only.

You must re-import inside:

```rust
pub mod timings {
    use std::time::SystemTime;
}
```

---

### 3. Passing wrong types

Function:

```rust
pub fn time2(n: Duration)
```

Calling:

```rust
time2(5); // wrong
```

Rust requires explicit construction:

```rust
time2(Duration::new(5, 0));
```

No implicit conversions.

---

### 4. Forgetting semicolon after use

Wrong:

```rust
use my_project::timings::{time1, time2}
```

Correct:

```rust
use my_project::timings::{time1, time2};
```

---

### 5. Thinking mod.rs is a new crate

`mod.rs` is not a crate.

It is the root file of a module inside the same crate.

Difference:

- `lib.rs` → root of crate
    
- `mod.rs` or `garden.rs` → root of module
    

---

## 8. Modern Module Layout

Avoid `mod.rs` in new projects.

Use:

```
src/
├── lib.rs
├── main.rs
├── garden.rs
└── garden/
    └── vegetable.rs
```

Example:

```rust
// lib.rs
pub mod garden;
pub use garden::vegetable::grow;

// garden.rs
pub mod vegetable;

// vegetable.rs
pub fn grow() {
    println!("Growing vegetable");
}
```

Now external code can use:

```rust
use my_project::grow;
```

Re-exports simplify public API.

---

## 9. Example Full Minimal Project

```
src/
├── lib.rs
├── main.rs
├── farm.rs
└── farm/
    └── vegetable.rs
```

```rust
// lib.rs
pub mod farm;
pub use farm::vegetable::{Vegetable, VegetableType};

// farm.rs
pub mod vegetable;

// farm/vegetable.rs
#[derive(Debug)]
pub enum VegetableType {
    Carrot,
    Potato,
}

#[derive(Debug)]
pub struct Vegetable {
    pub kind: VegetableType,
    water_level: u8,
}

impl Vegetable {
    pub fn new(kind: VegetableType) -> Self {
        Self { kind, water_level: 0 }
    }

    pub fn water(&mut self, amount: u8) {
        self.water_level += amount;
    }

    pub fn status(&self) {
        println!("{:?} | Water: {}", self.kind, self.water_level);
    }
}

// main.rs
use my_project::{Vegetable, VegetableType};

fn main() {
    let mut carrot = Vegetable::new(VegetableType::Carrot);
    carrot.water(5);
    carrot.status();
}
```

---

## 10. Final Mental Model

Crate:

- Compilation unit
    
- Rooted at `lib.rs` or `main.rs`
    

Module:

- Namespace inside crate
    
- Declared explicitly with `mod`
    

Package:

- Cargo project
    
- Contains one or more crates
    

use:

- Path alias
    
- Scoped to current module only
    

Files:

- Implementation detail
    
- Not automatically modules
    

main.rs:

- Entry point
    
- Should stay thin
    
- Calls into library code
    

lib.rs:

- Public API surface
    
- Re-export clean interfaces