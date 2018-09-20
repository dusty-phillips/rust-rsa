extern crate num_bigint;
extern crate num_traits;
extern crate rand;

pub mod math {
    use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt, ToBigUint};
    use num_traits::{One, Signed, Zero};
    extern crate rand;

    /// Check if a number is (probably) prime using the miller-rabin test
    fn is_prime(number: &BigUint) -> bool {
        let rng = &mut rand::thread_rng();

        let first_primes = [
            2.to_biguint().unwrap(),
            3.to_biguint().unwrap(),
            5.to_biguint().unwrap(),
            7.to_biguint().unwrap(),
            11.to_biguint().unwrap(),
            13.to_biguint().unwrap(),
            17.to_biguint().unwrap(),
            19.to_biguint().unwrap(),
            23.to_biguint().unwrap(),
            29.to_biguint().unwrap(),
            31.to_biguint().unwrap(),
            37.to_biguint().unwrap(),
        ];
        let zero = BigUint::zero();
        let one = BigUint::one();
        let two = &first_primes[0];
        match number {
            _ if number.is_zero() => return false,
            _ if number.is_one() => return false,
            _ if first_primes.contains(number) => return true,
            _ => (),
        }

        for prime in first_primes.iter() {
            if (number % prime).is_zero() {
                return false;
            }
        }

        let mut odd_factor = number - &one;
        let mut mod_levels = 0;

        while &odd_factor % two == zero {
            odd_factor = odd_factor / two;
            mod_levels += 1;
        }

        'outer: for _ in 1..42 {
            let mut witness = rng.gen_biguint_range(two, number);
            let mut mod_pow = witness.modpow(&odd_factor, number);
            if mod_pow.is_one() {
                continue;
            }
            for _ in 0..mod_levels {
                if mod_pow == number - &one {
                    break 'outer;
                } else {
                    mod_pow = mod_pow.modpow(two, number)
                }
            }
            return false;
        }
        true
    }

    /// Calculate the modular inverse of a mod m
    ///
    /// # Example
    /// ```
    /// extern crate num_bigint;
    /// extern crate rsa_rust;
    /// extern crate num_traits;
    /// use num_bigint::{BigUint, ToBigUint};
    /// use rsa_rust::math::mod_inverse;
    /// use num_traits::One;
    /// let three = 3.to_biguint().unwrap();
    /// let four = 4.to_biguint().unwrap();
    /// let seven = 7.to_biguint().unwrap();
    /// let nine = 9.to_biguint().unwrap();
    /// assert_eq!(mod_inverse(&three, &four).unwrap(), three);
    /// assert_eq!(mod_inverse(&nine, &four).unwrap(), BigUint::one());
    /// assert_eq!(mod_inverse(&seven, &four).unwrap(), three);
    /// ```
    pub fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
        let mut u: BigInt;
        let mut v: BigInt;
        let mut r: BigInt;
        let mut s: BigInt;
        let signed_m = m.clone().to_bigint().unwrap();

        if a < m {
            u = m.clone().to_bigint().unwrap();
            v = a.clone().to_bigint().unwrap();
            r = BigInt::zero();
            s = BigInt::one();
        } else {
            u = a.clone().to_bigint().unwrap();
            v = m.clone().to_bigint().unwrap();
            r = BigInt::one();
            s = BigInt::zero();
        }

        while v.bits() > 1 {
            let f = u.bits() - v.bits();
            if u.sign() == v.sign() {
                u -= &v << f;
                r -= &s << f;
            } else {
                u += &v << f;
                r += &s << f;
            }
            if u.bits() < v.bits() {
                ::std::mem::swap(&mut u, &mut v);
                ::std::mem::swap(&mut r, &mut s);
            }
        }

        if v.is_zero() {
            return None;
        }

        if v.is_negative() {
            s = -s;
        }

        if s > signed_m {
            return Some((s - signed_m).to_biguint().unwrap());
        }
        if s.is_negative() {
            return Some((s + signed_m).to_biguint().unwrap());
        }

        s.to_biguint()
    }
    /// Generate a random prime number with given number of bits
    pub fn random_prime(num_bits: usize) -> BigUint {
        let rng = &mut rand::thread_rng();

        loop {
            let shift = BigUint::one() << (num_bits - 1);
            let candidate = rng.gen_biguint(num_bits) | shift;
            if is_prime(&candidate) {
                return candidate;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use math;
        const PRIMES: [i32; 168] = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599,
            601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
            709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823,
            827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941,
            947, 953, 967, 971, 977, 983, 991, 997,
        ];
        #[test]
        fn first_primes() {
            for prime in PRIMES.iter() {
                let big_prime = prime.to_biguint().unwrap();
                assert!(math::is_prime(&big_prime));
            }
        }
        #[test]
        fn large_prime() {
            let large_prime = BigUint::parse_bytes(b"1959327544016233395069686482461216460684630727745975435114055162373738312337776238595282447579477913014225889548142035850219175927847942348358604491853073", 10).unwrap();
            assert!(math::is_prime(&large_prime));
        }
        #[test]
        fn modular_inverse() {
            let three = 3.to_biguint().unwrap();
            let four = 4.to_biguint().unwrap();
            let seven = 7.to_biguint().unwrap();
            let nine = 9.to_biguint().unwrap();
            assert_eq!(mod_inverse(&three, &four).unwrap(), three);
            assert_eq!(mod_inverse(&nine, &four).unwrap(), BigUint::one());
            assert_eq!(mod_inverse(&seven, &four).unwrap(), three);

            for i in 1..13 {
                let big_i = i.to_biguint().unwrap();
                let eleven = 11.to_biguint().unwrap();
                let thirteen = 13.to_biguint().unwrap();
                assert_eq!(
                    mod_inverse(&big_i, &thirteen).unwrap(),
                    big_i.modpow(&eleven, &thirteen)
                );
            }
        }
    }
}
