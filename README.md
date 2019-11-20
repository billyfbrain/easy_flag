easy_flag
=========

Simple command line flag parser for rust.

```rust
use easy_flag::FlagSet;

fn main() -> Result<(), String>{
    let mut help = false;
    let mut my_flag = String::from("default value");
    let args: Vec<String> = std::env::args().collect();

    let mut my_set = FlagSet::new(&args[0])
        .add("-h, --help", &mut help, "Prints help message.")
        .add("-m, --my-flag", &mut my_flag, "Help message for my_flag with string `value`");

    if let Err(err) = my_set.parse(&args[1..]) {
        println!("{}", my_set.defaults());
        return Err(err);
    }

    let usage = my_set.usage();
    if help {
        println!("{}", usage);
        return Ok(());
    }

    println!("my_flag flag value: {}", my_flag);

    Ok(())
}
```
#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version2.0</a> 
or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
