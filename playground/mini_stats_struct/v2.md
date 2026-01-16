# Update: Mini Stats Calculator v2.0 (Structs & Metadata)

This update explores nested structs and the use of the `std::time` module in Rust.

## What's New?

In the second iteration of the struct-based calculator, we've introduced a `MetaData` struct to handle non-statistical information about the dataset.

### 1. Nested Structs
The `Dataset` struct now contains a `MetaData` struct instance. This is a common pattern for organizing related data.

```rust
struct MetaData {
    name: String,
    created_on: time::SystemTime
}

struct Dataset {
    list: Vec<i32>,
    data: MetaData
}
```

### 2. Time Handling
We use `std::time::SystemTime::now()` to capture when the dataset was created. This introduces basic working with system clocks and the `Debug` trait for printing timestamps.

### 3. Method Implementation
The `MetaData` struct has its own `impl` block with a `describe` method:

```rust
impl MetaData {
    fn describe (&self) {
        println!("Name is: {:?}", self.name);
        println!("{:?}", self.created_on);
    }
}
```

## Links
- [Main Repository](https://github.com/lakshayknows/rust-mini-stats_calculator)
- [Source Code (v2.0)](https://github.com/lakshayknows/rust-mini-stats_calculator/blob/main/src/bin/using_struct.rs)
- [Learning Log Home](https://github.com/lakshayknows/rust-learning-log)
