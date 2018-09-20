extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate rsa_rust;

use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::One;
use rsa_rust::math;

#[derive(Debug)]
struct RSAKey {
    modulus: BigUint,
    pub_exponent: BigUint,
    priv_exponent: BigUint,
}

/// Generate an RSA key from two large primes of size `num_bits`.
///
/// Generally follows the RSA algorithm on [wikipedia](https://simple.wikipedia.org/wiki/RSA_algorithm)
fn generate_key(num_bits: usize) -> RSAKey {
    let one = BigUint::one();

    let prime1 = math::random_prime(num_bits / 2);
    let mut prime2 = math::random_prime(num_bits / 2);
    while prime1 == prime2 {
        prime2 = math::random_prime(num_bits / 2);
    }
    let modulus = &prime1 * &prime2;
    let totient = (prime1 - &one) * (prime2 - &one);
    let mut pub_exponent = math::random_prime(num_bits / 2);
    while pub_exponent >= totient || pub_exponent.gcd(&totient) != one {
        pub_exponent = math::random_prime(num_bits / 2);
    }
    let priv_exponent = math::mod_inverse(&pub_exponent, &totient).unwrap();

    RSAKey {
        modulus: modulus,
        pub_exponent: pub_exponent,
        priv_exponent: priv_exponent,
    }
}

fn encrypt(key: &RSAKey, message: &BigUint) -> BigUint {
    message.modpow(&key.pub_exponent, &key.modulus)
}

fn decrypt(key: &RSAKey, ciphertext: &BigUint) -> BigUint {
    ciphertext.modpow(&key.priv_exponent, &key.modulus)
}

fn main() {
    let key = generate_key(512);
    let message = math::random_prime(256);
    let ciphertext = encrypt(&key, &message);
    let deciphered = decrypt(&key, &ciphertext);
    println!("{:?} {} {} {}", key, message, ciphertext, deciphered);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rsa_round_trip() {
        let key = generate_key(512);
        let message = math::random_prime(256);
        let ciphertext = encrypt(&key, &message);
        let deciphered = decrypt(&key, &ciphertext);
        assert_eq!(message, deciphered);
    }
}
