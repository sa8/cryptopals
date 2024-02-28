const LETTER_FREQ: [f64; 27] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, // A-G
    0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, // H-N
    0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, // O-U
    0.00978, 0.02360, 0.00150, 0.01974, 0.00074, 0.19181, // V-Z & space char
];

pub fn bruteforce(ciphertext: &str) -> String {
    let bytes_cipher = hex::decode(ciphertext).unwrap();
    let decipher: Vec<u8> = bytes_cipher.iter().map(|&byte| byte ^ (0 as u8 )).collect();
    let msg = String::from_utf8_lossy(&decipher);
    let mut bigger_score = score(&msg);
    let mut word = String::new();
    // bruteforce over all characters
    for c in 0..255 {
        let decipher: Vec<u8> = bytes_cipher.iter().map(|&byte| byte ^ (c as u8 )).collect();
        let msg = String::from_utf8_lossy(&decipher);
        //compute the scrore of the deciphered text
        let mut new_score = score(&msg);
        if new_score<bigger_score{
            bigger_score = new_score;
           word = String::from(msg);
        }

    }

    word
}
fn score(text: &str) -> f64 {
    // for each letter in string count their frequency
    let mut word_count: [u8; 27] = [0; 27];

    for c in text.chars(){
        match c {
            'a'..='z' | 'A'..='Z' => {
                let idx = (c.to_ascii_lowercase() as u8 - b'a') as usize ;
                word_count[idx] += 1;
            }
            ' ' => {
                word_count[26]+=1;
            }
            _=> {}
        }
    }
    

    let mut word_freq: [f64; 27] = [0.0;27];
    for i in 0..27 {
        word_freq[i] = word_count[i] as f64 / text.len() as f64;
    }
   // println!("{:?}",word_freq);
    return chi_square_test(&word_freq, &LETTER_FREQ)

}

fn chi_square_test(observed: &[f64; 27], expected: &[f64;27]) -> f64 {
    let mut chi_square = 0.0;

    for i in 0..27 {
        if expected[i] != 0.0 {
            chi_square += (observed[i]-expected[i]).powi(2) / expected[i];
        }
    }
    chi_square
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decryption(){
        let hex_input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let expected_base64_output = "Cooking MC's like a pound of bacon";

        assert_eq!(bruteforce(hex_input), expected_base64_output);
    }
}