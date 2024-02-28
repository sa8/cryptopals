//extern crate base64;
extern crate hex;
extern crate rustc_serialize as serialize;

use serialize::base64::{self, ToBase64};
use serialize::hex::FromHex;

pub fn hex_to_base64(hex_str:  &str) -> Result<String, hex::FromHexError > {
    // decode hex to byte
    // let bytes = hex::decode(hex_str)?;

    // //Encode the bytes to a base64 string
    // let base64_str = base64::encode(bytes);

    let base64_str = hex_str.from_hex().unwrap().to_base64(base64::STANDARD);

    Ok(base64_str)
    
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64(){
        let hex_input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
        ";
        let expected_base64_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(hex_to_base64(hex_input).unwrap(), expected_base64_output);
    }
}