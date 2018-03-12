extern crate ferris_says;

use ferris_says::say;
use std::io::{stdout, BufWriter};

#[macro_export]
macro_rules! ferrisprint {
($fmt:expr) => {
        {
            let stdout = stdout();
            let width = 24;
            let mut writer = BufWriter::new(stdout.lock());
            say(concat!($fmt).as_bytes(), width, &mut writer).unwrap();
        }
    };
($fmt:expr, $($arg:tt)*) => {
        {
            let stdout = stdout();
            let width = 24;
            let mut writer = BufWriter::new(stdout.lock());
            say(format!(concat!($fmt), $($arg)*).as_bytes(), width, &mut writer).unwrap();
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ferrisprint_macro_works() {
        ferrisprint!("hola");
        let str = "ok";
        ferrisprint!("hey {}", &str);
    }

    #[test]
    fn ferrisprint_macro_works_with_multiple_arguments() {
        ferrisprint!("I am {} the {}", "ferris", "crab");
    }
}
