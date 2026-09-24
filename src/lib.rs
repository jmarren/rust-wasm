use wasm_bindgen::prelude::*;

mod dom;

/// Returns a greeting built in Rust.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    let name = if name.trim().is_empty() { "stranger" } else { name.trim() };
    format!("Hello, {name}! This string was built in Rust.")
}

/// Computes the nth Fibonacci number iteratively.
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        (a, b) = (b, a.wrapping_add(b));
    }
    a
}

/// Counts the primes below `limit` with a sieve of Eratosthenes.
#[wasm_bindgen]
pub fn count_primes(limit: u32) -> u32 {
    let limit = limit as usize;
    if limit < 2 {
        return 0;
    }
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i < limit {
        if is_prime[i] {
            for j in (i * i..limit).step_by(i) {
                is_prime[j] = false;
            }
        }
        i += 1;
    }
    is_prime.iter().filter(|&&p| p).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn primes() {
        assert_eq!(count_primes(10), 4);
        assert_eq!(count_primes(100), 25);
    }
}
