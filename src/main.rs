//use rust_crate::PrimaryColor;
//use rust_crate::mix;
use hocuspocus_rust_crate::art::PrimaryColor;
use hocuspocus_rust_crate::art::mix;
use hocuspocus_rust_crate::add_one::add_one;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
