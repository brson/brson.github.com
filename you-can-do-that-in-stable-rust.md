---
layout: basic
---

# [Create a hash map](#create-a-hash-map)

[`HashMap`][HashMap] is a workhorse associative container, mapping from keys to
values. Use these a bunch.

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("key1", "value1");
map.insert("key2", "value2");

if map.contains_key("key1") {
    map.remove("key1");
}

for (key, value) in map.into_iter() {
    println!("{}:{}", key, value);
}
```

[HashMap]: http://doc.rust-lang.org/std/collections/struct.HashMap.html

# Put custom keys in a `HashMap`

To put a key in a [`HashMap`][HashMap] a type must implement [`Hash`][hash],
and [`Eq`][eq], which in turn requires `PartialEq`. Fortunately, these
can almost alwise be derived automatically.


```rust
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
struct Key(&'static str);

let mut map = HashMap::new();
map.insert(Key("key1"), "value1");
```

[HashMap]: http://doc.rust-lang.org/std/collections/struct.HashMap.html
[hash]: http://doc.rust-lang.org/std/hash/
[eq]: http://doc.rust-lang.org/std/cmp/

# Create a fast hash map

# Create a thread

# Create a thread and wait for it

<script type="text/javascript" src="you-can-do-that-in-stable-rust.js"></script>
