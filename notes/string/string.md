# String
### What is a String?

- `String` → **owned, mutable, growable**, UTF-8 encoded
    
- `&str` → **borrowed string slice**, usually immutable
    
- Internally: `String = Vec<u8> + UTF-8 rules`
    

---

### Creating Strings

```rust
let s = String::new();
let s = "hello".to_string();
let s = String::from("hello");
```

- `to_string()` and `String::from()` → same result, stylistic choice
    

---

### UTF-8 Reality Check

- Length ≠ number of characters
    
- `"Hola"` → 4 bytes
    
- `"Здравствуйте"` → 24 bytes
    
- Rust **refuses indexing** (`s[0]`) to avoid lying to you
    

---

### Updating Strings

```rust
let mut s = String::from("foo");
s.push_str("bar"); // foo + &str
s.push('!');       // single char
```

---

### Concatenation (important differences)

- `+` operator
    

```rust
let s3 = s1 + &s2; // s1 is moved, s2 borrowed
```

- `format!` macro ✅ preferred
    

```rust
let s = format!("{s1}-{s2}-{s3}");
```

- `format!`:
    
    - readable
        
    - no ownership loss
        
    - uses references internally
        

---

### Why No Indexing?

- Strings can be viewed as:
    
    - **bytes** (`.bytes()`)
        
    - **Unicode scalar values** (`.chars()`)
        
    - **grapheme clusters** (not in std)
        
- Indexing is:
    
    - ambiguous
        
    - not O(1)
        
    - unsafe for UTF-8
        

---

### Slicing Strings ⚠️

```rust
let s = &hello[0..4]; // valid byte boundary
```

- Must slice on **char boundaries**
    
- Invalid slice → **runtime panic**
    

---

### Iterating (explicit is holy)

```rust
for c in s.chars() { }
for b in s.bytes() { }
```

---

### Handy String Methods

- **Replace**
    

```rust
let s = "hi world".replace("world", "rust");
```

- **Trim (whitespace)**
    

```rust
let s = "  hello  ".trim();
```

- Others worth remembering:
    
    - `.contains()`
        
    - `.starts_with()`
        
    - `.ends_with()`
        

---

### Mental Model (tattoo this)

> Rust strings are **bytes first**,  
> **text second**,  
> and **safe by default**.
