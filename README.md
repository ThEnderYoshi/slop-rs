# SLOP-RS

This is the official Rust implementation of the SLOP language.

Sans' Lovely Properties (SLOP) is a data storage language designed to be tiny,
(both in complexity and amount of characters) while still being human-readable.

---

**Note:** The crate is still in a pre-release state,
so even minor updates may introduce breaking changes.

If you have a problem/suggestion/general feedback, please create an issue on the
crate's GitHub page.

---

```rust
use std::{error::Error, fs};

use slop_rs::Slop;

fn main() -> Result<(), Box<dyn Error>> {
    /// See also: Slop::new() and Slop::open()
    let mut slop: Slop = "some-key=some value".parse()?;
    println!("{:?}", slop.get("some-key"));

    let mut s = slop.get_string("some-key").unwrap().to_owned();
    s.push_str("!!!");

    slop.insert("some-key".to_string(), s)?;
    println!("{:?}", slop.get("some-key"));

    slop.save("example.slop");
    Ok(())
}
```

See `examples/` for both examples of the API and sample SLOP files.

## The Language

SLOP is so simple it can be entirely explained by the following code block:

```slop
# Lines starting with `#` are comments
# Any leading (but not trailing) whitespace in lines is ignored.
# A singular trailing carriage return (\r) is also igonred, if it is present.

# This is a string key-value. (AKA string KV)
some-key=some value # This is NOT a comment, it is part of the value.

# This is a list KV. Each line is an item.
list-kv{
    item 1
    item 2
    item 3
    # This is NOT a comment, every line (even empty ones) between the brackets
    # are treated as items.
Indentation is optional.
List KVs cannot be nested.
}

# This is how empty list KVs look:
empty-list-kv{
}

# This is invalid:
#empty-list-kv{}

# Keys can contain any character except for `=`, `{` and newlines.
```

And that's it!
