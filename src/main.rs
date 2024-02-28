mod set1;
fn main() {
    println!("Hello, world!");
    println!("{}",set1::problem1::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap());
    println!("{}", set1::problem3::bruteforce("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"));
    set1::problem4::get_file("https://cryptopals.com/static/challenge-data/4.txt")
   // println!("{}", set1::problem4::get_file("https://cryptopals.com/static/challenge-data/4.txt"));
}
