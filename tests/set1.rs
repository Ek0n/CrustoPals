extern crate base64;
extern crate hex;
extern crate crustopals;

#[cfg(test)]
mod tests {
    use hex;
    use base64;
    use crustopals::*;
    use std::collections::BTreeMap;


    fn setup_corpus() -> BTreeMap<char, f64> {
        let mut corpus :BTreeMap<char, f64> = BTreeMap::new();
        build_corpus_from_file("tests/data/prideandprejudice.txt", &mut corpus);
        return corpus.clone();
    }

    #[test]
    fn problem1() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let out = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        let enc = hex::decode(input).ok();

        match enc {
            Some(val) => assert_eq!(base64::encode(&val), out),
            None => {
                println!("Mistakes were made");
                assert!(false)
            }
        }
    }

    #[test]
    fn problem2() {
        let a = "1c0111001f010100061a024b53535009181c";
        let b = "686974207468652062756c6c277320657965";

        assert_eq!(hex::encode(x_or(&hex::decode(a).expect("Getting a"), &hex::decode(b).expect("Getting b"))), "746865206b696420646f6e277420706c6179");
    }

    #[test]
    #[ignore]
    fn problem3() {
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        let c = setup_corpus();
        let (res, key, score) = find_single_key_x_or(hex::decode(input).expect("Getting input"), &c);

        println!("Result {:?}", String::from_utf8(res).unwrap());
        println!("Key {}", key);
        println!("Score {}", score);
    }

    #[test]
    #[ignore]
    fn problem4() {
        let mut best_score :f64 = 0.0;
        let mut best_text :Vec<u8> = Vec::new();

        let lines = read_split_lines("tests/data/4.txt");
        let c = setup_corpus();

        for l in lines {
            let (t, _, s) = find_single_key_x_or(hex::decode(l).expect("Getting input"), &c);
            if s > best_score {
                best_text = t;
                best_score = s
            }
        }
        
        println!("Result {:?}", String::from_utf8(best_text).unwrap());
    }

    #[test]
    fn problem5() {
        let input = String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
        let key = String::from("ICE");

        assert_eq!(x_or(&input.into_bytes(), &key.into_bytes()),
         hex::decode("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").expect("Getting Solution"));
    }
}