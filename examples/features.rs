//! This example shows some of the API's features.
//! 
//! Each function (other than [main]) is a different example.

use std::error::Error;

use slop_rs::Slop;

fn empty_slop() -> Result<(), Box<dyn Error>> {
    // Let's create a new Slop...
    let mut slop = Slop::new();

    // ...insert something into it...
    slop.insert("some-key".to_string(), "some value")?;

    // ...then print its KV.
    println!("`some-key` = `{:?}`", slop.get("some-key"));

    Ok(())
}

fn parsed_slop() -> Result<(), Box<dyn Error>> {
    // Let's create a Slop by parsing a SLOP string...
    let mut slop: Slop = "some-key=some value".parse()?;

    // ...print its KV...
    println!("`some-key` = `{:?}`", slop.get("some-key"));

    // ...get the KV's value...
    let mut s = slop
        .get_string("some-key")
        .expect("expected string kv `some-key`")
        .to_owned();

    // ...modify it...
    s.push_str("!!!");

    // ...put it back into the Slop...
    slop.insert("some-key".to_string(), s)?;

    // ...then print it again.
    println!("`some-key` = `{:?}`", slop.get("some-key"));

    // Let's also add a list KV.
    // Also, since we know that the key is valid, we can use insert_unchecked()
    // instead of insert().
    slop.insert_unchecked("other-key".to_string(), vec!["value 1", "value 2"]);
    println!("`other-key` = `{:?}`", slop.get("other-key"));

    // Looks good! Let's save this Slop.
    slop.save("examples/example.slop")?;

    // Actually, let's pretty-save it so it's more readable.
    slop.save_pretty("examples/example_pretty.slop")?;

    Ok(())
}

fn file_slop() -> Result<(), Box<dyn Error>> {
    // Let's create a Slop by reading a SLOP file...
    let slop = Slop::open("examples/test.slop")?;

    // ...and print its contents.
    for (key, value) in slop {
        println!("Key: {key}\tValue: {value:?}");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Running empty_slop()...");
    empty_slop()?;

    println!("\nRunning parsed_slop()...");
    parsed_slop()?;

    println!("\nRunning file_slop()...");
    file_slop()?;
    Ok(())
}
