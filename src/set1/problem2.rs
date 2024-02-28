extern crate hex;

pub fn xor(str1: &str, str2: &str) -> Result<String, hex::FromHexError> {
    // check length
    // if str1.len() != str2.len() {
    //     Err();
    // }

    let bytes1 = hex::decode(str1).unwrap();
    let bytes2 = hex::decode(str2).unwrap();


    let xor_bytes: Vec<u8> = bytes1
    .iter()
    .zip(bytes2.iter())
    .map(|(&b1, &b2)| b1 ^ b2)
    .collect();
    Ok(hex::encode(xor_bytes))
}

#[cfg(test)]
mod tests {
    use crate::set1::problem2::xor;
    #[test]
    fn test_xor(){
        let input1 = "1c0111001f010100061a024b53535009181c";
        let input2 = "686974207468652062756c6c277320657965";
        let expected_output = "746865206b696420646f6e277420706c6179";

        assert_eq!(xor(input1, input2).unwrap(), expected_output);
    }

    #[test]
    #[should_panic]
    fn test_unwrap_xor(){
        let input1 = "1c0111001f010100061a024b53535009181c";
        let input2 = "686974207468652062756c6c2773206";
        //let expected_output = "746865206b696420646f6e277420706c6179";

       xor(input1, input2).unwrap();
    }
}