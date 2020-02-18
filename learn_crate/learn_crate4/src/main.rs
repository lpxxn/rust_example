extern crate crypto;

use crypto::digest::Digest;
use crypto::sha3::Sha3;
fn main() {
    let mut haster = Sha3::sha3_256();
    haster.input_str("hello world");
    let result = haster.result_str();
    println!("resutl {}", result);
}
