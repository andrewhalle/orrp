extern crate open;

fn main() {
    open::that("https://www.google.com").expect("Couldn't open Google.");
}
