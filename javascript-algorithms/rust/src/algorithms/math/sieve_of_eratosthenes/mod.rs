/// Each prime up to `max_number`: each number not crossed out as a multiple
/// of a smaller one.
pub fn sieve_of_eratosthenes(max_number: usize) -> Vec<usize> {
    let mut is_prime = vec![true; max_number + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut primes = Vec::new();
    for number in 2..=max_number {
        if is_prime[number] {
            primes.push(number);
            // Its multiples below its square were crossed out by smaller primes.
            let mut next = number * number;
            while next <= max_number {
                is_prime[next] = false;
                next += number;
            }
        }
    }
    primes
}

js::export_default!(sieve_of_eratosthenes);
