
fn main() {
    println!("{}", sum_list(get_first_1000_primes()));
}

fn sum_list(v: Vec<int>) -> int {
    return v.iter().fold(0, |a, &b| a + b);
}

fn get_first_1000_primes() -> Vec<int> {
    let mut prime_counter: int = 0i;
    let mut current_number: int = 2i;
    let mut primes = vec![];

    while prime_counter < 1000 {
        if is_prime(current_number) {
            primes.push(current_number);
            prime_counter += 1;
        }
        current_number += 1;
    }

    return primes;
}

fn is_prime(n: int) -> bool {
    for k in range(2, n) {
        if div_by_k(n, k) {
            return false;
        }
    }
    return true;
}

fn div_by_k(n: int, k: int) -> bool {
    n % k == 0
}
