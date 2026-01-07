
---

## Mutable References to Arrays

### Example

```rust
let x = &mut [1, 2, 4];
```

### Explanation

- `[1, 2, 4]` creates a **temporary array** on the stack
    
- `&mut` creates a **mutable reference** to that array
    
- `x` does **not** own the array
    
- `x` is a **reference (pointer)** to the array
    

Type:

```rust
x: &mut [i32; 3]
```

`&mut` allows both reading and writing to the referenced data.

---

## Defining a Reference Without a Named Array

You **can directly create a reference to an array without first defining the array in a separate variable**, as long as the reference stays within a limited scope.

```rust
let x = &mut [1, 2, 4];
```

Rust performs **temporary lifetime extension**, meaning the array lives as long as the reference `x` is valid.

Conceptually equivalent to:

```rust
let mut temp = [1, 2, 4];
let x = &mut temp;
```

### Limitation

This is only allowed for short, local usage.

You cannot return such a reference:

```rust
fn bad() -> &mut [i32] {
    &mut [1, 2, 4] // ❌ invalid
}
```

The temporary array would be dropped at the end of the function.

### Practical Rule

- Local, short-lived logic → direct reference is fine
    
- Reuse, sharing, or returning → define the array explicitly
    

```rust
let mut arr = [1, 2, 4];
let x = &mut arr;
```

---

## What `iter_mut()` Does

### Example

```rust
for elem in x.iter_mut() {
    *elem += 2;
}
```

### Explanation

- `iter_mut()` returns an **iterator**
    
- The iterator yields `&mut i32`
    
- Each `elem` is a **mutable reference to an array element**
    

No values are copied.  
All changes affect the original array.

---

## Dereferencing with `*`

Given:

```rust
elem: &mut i32
```

You cannot modify a reference directly.

`*elem` dereferences the pointer and accesses the **actual value**.

```rust
*elem += 2;
```

Equivalent to:

```rust
*elem = *elem + 2;
```

This modifies the element stored inside the array.

---

## What Is Actually Being Mutated

- The reference is **not** modified
    
- No copies are created
    
- The **original array element** is mutated in place
    

Memory view:

```text
x ──▶ [1, 2, 4]
        ↑
      elem
```

---

## Loop vs Iterator

### Loop

A loop controls **repetition**.

Examples:

```rust
loop { }
while condition { }
for x in something { }
```

A loop answers:

> How long should this code run?

---

### Iterator

An iterator controls **data flow**.

- Produces values one at a time
    
- Core method:
    

```rust
next() -> Option<Item>
```

An iterator answers:

> What is the next value?

---

## How `for` Works with Iterators

```rust
for elem in x.iter_mut() {
    *elem += 2;
}
```

Is equivalent to:

```rust
let mut iter = x.iter_mut();

loop {
    match iter.next() {
        Some(elem) => {
            *elem += 2;
        }
        None => break,
    }
}
```

- `iter_mut()` supplies values
    
- `for` handles repetition
    

---

## Why Iterators Are Preferred

Index-based loop:

```rust
for i in 0..x.len() {
    x[i] += 2;
}
```

Problems:

- Manual indexing
    
- Easier to make logic errors
    
- Harder for the borrow checker
    

Iterator-based loop:

```rust
for elem in x.iter_mut() {
    *elem += 2;
}
```

Benefits:

- No indices
    
- No bounds errors
    
- Mutability and ownership enforced
    

---

## Core Mental Models

- A reference is an **address**, not a value
    
- `iter_mut()` yields **mutable references**
    
- `*` accesses the **actual data**
    
- A loop repeats code
    
- An iterator supplies values
    

---

## One-Line Summary

`iter_mut()` provides mutable access to each element,  
`*elem` modifies the actual value,  
and the `for` loop continues until the iterator is exhausted.