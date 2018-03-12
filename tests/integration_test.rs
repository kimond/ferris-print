#[macro_use]
extern crate ferris_print;

#[test]
fn ferrisprint_macro_works() {
    ferrisprint!("I'm working well!");
}
#[test]
fn ferrisprint_macro_works_with_parameters() {
    ferrisprint!("I'm working well! {}", "again");
}
