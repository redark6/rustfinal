use std::io::Bytes;
use std::ops::Deref;
use std::thread;
use rand::Rng;
use md5;
use rand::rngs::ThreadRng;
use challenge_trait::ChallengeTrait;
pub (crate) mod challenge_trait;
use std::str;
use md5::Digest;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashInput {
    pub complexity: u32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String,
}

pub struct HashCash {
    pub input: MD5HashCashInput
}

impl HashCash {
    fn get_leading_zeros(byte_arrays: &[u8]) -> u32{
        let mut leading_zeros: u32 = 0;
        let mut current_leading_zeros: u32 = 0;
        for i in 0..byte_arrays.len() {
            current_leading_zeros = byte_arrays[i].leading_zeros();
            leading_zeros += current_leading_zeros;
            if current_leading_zeros < 8 { return leading_zeros; }
        }
        return leading_zeros;
    }

    fn digest(seed: &u64, message: &String) -> Digest {
        let mut hexa_seed = format!("{:01$X}", seed, 16);
        md5::compute((hexa_seed.to_string() + message).as_bytes())
    }
}

impl ChallengeTrait for HashCash {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        "hashCash".to_string()
    }

    fn new(input: Self::Input) -> Self {
        return HashCash { input };
    }

    fn solve(&self) -> Self::Output {
        let mut output: MD5HashCashOutput = MD5HashCashOutput { seed: 0, hashcode: "".to_string() };
        let mut found = false;
        while !found {
            let seed = rand::thread_rng().gen::<u64>();
            let digest = HashCash::digest(&seed, &self.input.message);
            output = MD5HashCashOutput { seed, hashcode: format!("{:X}", digest) };
            found = HashCash::verify(self, &output)
        } {}
        output
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        let digest = HashCash::digest(&answer.seed, &self.input.message);
        let zeros= HashCash::get_leading_zeros(digest.as_slice());
        return zeros >= self.input.complexity && answer.hashcode == format!("{:X}", digest);
    }
}

#[cfg(test)]
mod tests_hash_cash {
    use super::*;

    #[test]
    fn is_hash_cash_name() {
        assert_eq!(HashCash::name(), String::from("hashCash"));
    }

    #[test]
    fn is_hash_cash_new() {
        let new_has_cash = HashCash::new(MD5HashCashInput{complexity: 9, message: "hello".to_string()});
        assert_eq!(new_has_cash.input.message, String::from("hello"));
        assert_eq!(new_has_cash.input.complexity, 9);
    }

    #[test]
    fn is_hash_cash_get_leading_zeros() {
        let bytes_array= [0, 80, 139, 24, 242, 10, 109, 203, 203, 90, 106, 97, 186, 192, 120, 168];
        let zeros = HashCash::get_leading_zeros(&bytes_array);
        assert_eq!(zeros, 9);
    }

    #[test]
    fn is_hash_cash_digest() {
        let digest = HashCash::digest(&844, &String::from("hello"));
        let hash = format!("{:X}", digest);
        assert_eq!(hash, "00441745D9BDF8E5D3C7872AC9DBB2C3");
    }

    #[test]
    fn is_hash_cash_verify() {
        let new_has_cash = HashCash::new(MD5HashCashInput{complexity: 9, message: String::from("hello")});
        let output = MD5HashCashOutput{seed: 844 ,hashcode: String::from("00441745D9BDF8E5D3C7872AC9DBB2C3")};
        assert_eq!(HashCash::verify(&new_has_cash,&output), true);
    }
}
