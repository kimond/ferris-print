use std::io::Result as IoResult;

extern crate ferris_says;

pub fn say<W>(input: &[u8], width: usize, writer: &mut W) -> IoResult<()>
    where W: std::io::Write {
    ferris_says::say(input, width, writer)
}

#[macro_export]
macro_rules! ferrisprint {
($fmt:expr) => {
        {
            use std::io::{stdout, BufWriter};
            let stdout = stdout();
            let width = 24;
            let mut writer = BufWriter::new(stdout.lock());
            $crate::say(concat!($fmt).as_bytes(), width, &mut writer).unwrap();
        }
    };
($fmt:expr, $($arg:tt)*) => {
        {
            use std::io::{stdout, BufWriter};
            let stdout = stdout();
            let width = 24;
            let mut writer = BufWriter::new(stdout.lock());
            $crate::say(format!(concat!($fmt), $($arg)*).as_bytes(), width, &mut writer).unwrap();
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
