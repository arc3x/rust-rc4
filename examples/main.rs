extern crate rc4;

fn main() {
    let keystream:Vec<u8> =  rc4::keystream(100, 10000, "fish");
    println!("{:?}\n", keystream);
}
