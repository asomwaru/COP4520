fn main() {
    let max_n: usize = 10_000_000;
    let sieve = get_sieve(max_n);

    println!("Primes up to {}:", max_n);
    println!("{:#?}", sieve);
}

fn get_sieve(max_n: usize) -> Vec<usize> {
    let mut sieve = vec![true; max_n as usize];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..max_n {
        if sieve[i as usize] {
            let mut j = i * i;
            while j < max_n {
                sieve[j as usize] = false;
                j += i;
            }
        }
    }

    sieve
        .iter()
        .enumerate()
        .filter_map(|(num, prime)| if *prime { Some(num) } else { None })
        .collect::<Vec<usize>>()
}
