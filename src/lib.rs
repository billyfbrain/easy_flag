/*! `easy_flag` is a simple command line flag parser.

This library contains functions for handling command line flags.

# Example

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
*/

mod var;
use var::Var;

use std::any::Any;
use std::fmt::Write;

#[derive(Debug)]
struct Flag<'a> {
    name: &'a str,
    help: &'a str,
    var: Var<'a>,
}

impl<'a> Flag<'a> {
    fn unquote_usage(&self) -> (String, String) {
        let mut help = self.help.to_owned();
        let mut name = String::from(self.var.unquotename());

        let seg: Vec<&str> = help.split_terminator('`').collect();
        if seg.len() >= 3 {
            name = seg[1].to_owned();
            help = help.replacen('`', "", 2);
        }

        (name, help)
    }
}

/// Represents a set of defined flags.
#[derive(Debug)]
pub struct FlagSet<'a> {
    name: String,
    set: Vec<Flag<'a>>,
}

impl<'a> FlagSet<'a> {
    /// Constructs a new `FlagSet` instance. The `name` will be added to the usage information.
    ///
    /// # Examples
    ///
    /// ```no_run
    ///  # use easy_flag::FlagSet;
    ///  let set = FlagSet::new("my set")
    ///  # ;
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            set: Vec::new(),
        }
    }

    /// Adds a new flag to the `FlagSet`
    ///
    /// The function itself determines the type of flag.
    /// The standard types are supported: `bool`, `f32`, `f64`, `i8`, `i16`, `i32`, `i128`,
    /// `isize`, `u8`, `u16`, `u32`, `u128`, `usize`, `String`.
    ///
    /// **WARNING:** Unsupported type will cause panic.
    ///
    /// # Examples
    ///
    /// ```no_run
    ///  # use easy_flag::FlagSet;
    ///  let mut f = String::from("default value");
    ///  FlagSet::new("my set").add("-f, --flag", &mut f, "flag `example`")
    ///  # ;
    /// ```
    pub fn add(mut self, name: &'a str, var: &'a mut dyn Any, help: &'a str) -> Self {
        self.set.push(Flag {
            name,
            help,
            var: Var::new(var),
        });
        self
    }

    /// Parsing args.
    ///
    /// # Examples
    ///
    /// ```no_run
    ///  # use easy_flag::FlagSet;
    ///  let mut f = String::from("default value");
    ///  let args: Vec<String> = std::env::args().collect();
    ///  FlagSet::new(&args[0])
    ///                 .add("-f, --flag", &mut f, "flag `example`")
    ///                 .parse(&args[1..])
    ///                 .unwrap()
    ///  # ;
    /// ```
    pub fn parse(&mut self, args: &[String]) -> Result<(), String> {
        let mut args_iter = args.iter();
        while let Some(item) = args_iter.next() {
            if let Some(f) = self.set.iter_mut().find(|f| {
                f.name
                    .split_terminator(',')
                    .filter(|name| !name.is_empty())
                    .find(|name| name.trim() == item.as_str())
                    .is_some()
            }) {
                if let Var::Bool(var) = &mut f.var {
                    **var = true;
                } else {
                    if let Some(value) = args_iter.next() {
                        if let Err(err) = f.var.parse(value) {
                            return Err(format!("parse flag {} failed: {}", f.name, err));
                        }
                    } else {
                        return Err(format!("flag needs an argument: {}", f.name));
                    }
                }
            } else {
                return Err(format!("flag {} provided but not defined", item));
            }
        }
        Ok(())
    }

    /// Gets flag set name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Formats a usage message with set name.
    pub fn usage(&self) -> String {
        let mut usage: String = format!("Usage of {}:\n", self.name);
        usage.push_str(self.defaults().as_str());
        usage
    }

    /// Formats a flag list message.
    pub fn defaults(&self) -> String {
        let mut formated_help: String = String::new();
        for f in self.set.iter() {
            formated_help
                .write_fmt(format_args!("  {}", f.name))
                .unwrap();
            let (name, help) = f.unquote_usage();
            if !name.is_empty() {
                formated_help.push_str(" ");
                formated_help.push_str(name.as_str());
            }
            if formated_help.len() <= 4 {
                formated_help.push_str("\t");
            } else {
                formated_help.push_str("\n    \t");
            }
            formated_help.push_str(help.replace("\n", "\n    \t").as_str());
            formated_help.push_str("\n");
        }
        formated_help
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flagset_test() {
        let args = vec![
            "app_name".to_string(),
            "-b".to_string(),
            "--isize".to_string(),
            "-10".to_string(),
            "--f32".to_string(),
            "1.888".to_string(),
            "-u".to_string(),
            "40".to_string(),
            "-s".to_string(),
            "test_value".to_string(),
        ];

        let mut bool_val = false;
        let mut isize_val = 20;
        let mut usize_val = 15;
        let mut f32_val = 1.4;
        let mut string_val = String::from("default");
        let mut string_val2 = String::from("default");

        let mut flag_set = FlagSet::new(&args[0])
            .add("-b,--bool", &mut bool_val, "value bool")
            .add("-i, --isize", &mut isize_val, "value isize")
            .add("-u, --usize", &mut usize_val, "`value` usize")
            .add("-f, --f32", &mut f32_val, "`value` f32")
            .add("-s, --string", &mut string_val, "test string")
            .add("--s2", &mut string_val2, "test string default");
        flag_set.parse(&args[1..]).unwrap();
        // println!("{}", flag_set.usage());

        assert_eq!(bool_val, true);
        assert_eq!(isize_val, -10);
        assert_eq!(usize_val, 40);
        assert_eq!(f32_val, 1.888);
        assert_eq!(string_val, "test_value");
        assert_eq!(string_val2, "default");
    }
}
