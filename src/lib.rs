pub extern crate ferris_says;

#[macro_export]
macro_rules! ferrisprint {
($fmt:expr) => {
        {
            use std::io::{stdout, BufWriter};
            let stdout = stdout();
            let width = 24;
            let mut writer = BufWriter::new(stdout.lock());
            $crate::ferris_says::say(concat!($fmt).as_bytes(), width, &mut writer).unwrap();
        }
    };
($fmt:expr, $($arg:tt)*) => {
        {
            use std::io::{stdout, BufWriter};
            let stdout = stdout();
            let width = 24;
            let mut writer = BufWriter::new(stdout.lock());
            $crate::ferris_says::say(format!(concat!($fmt), $($arg)*).as_bytes(), width, &mut writer).unwrap();
        }
    };
}

#[cfg(test)]
mod tests {
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
