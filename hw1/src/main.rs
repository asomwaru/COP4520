use std::time::Instant;

fn main() {
    let max_n: usize = 100_000_000;
    let sqrt = f64::sqrt(max_n as f64).ceil() as usize + 1;

    let start = Instant::now();
    let small_sieve = get_sieve(sqrt);

    let mut primes: Vec<usize> = small_sieve.clone();

    let segments = 8;
    let segment_size: usize = (max_n - sqrt) / segments;

    for i in 0..segments {
        let start = sqrt + i * segment_size;
        let end = if sqrt + segment_size * (i + 1) < max_n {
            sqrt + segment_size * (i + 1)
        } else {
            max_n
        };

        let mut sieve = vec![true; segment_size];

        for prime in &small_sieve {
            let lowest_multiple = if (start / prime) * prime >= start {
                (start / prime) * prime
            } else {
                (start / prime) * prime + prime
            };

            for i in (lowest_multiple..(end + 1)).step_by(*prime as usize) {
                if i - start < segment_size {
                    sieve[(i - start) as usize] = false;
                }
            }
        }
        primes.append(&mut convert_to_num(&sieve, start));
    }

    let end = start.elapsed();
    println!("Time taken: {:?}", end);
    println!("Primes: {:?}", primes.len());
    println!("Sum: {:?}", primes.iter().sum::<usize>());
    println!("Last 10: {:#?}", &primes[primes.len() - 10..]);
}

fn convert_to_num(sieve: &Vec<bool>, start: usize) -> Vec<usize> {
    sieve
        .iter()
        .enumerate()
        .filter_map(|(num, prime)| if *prime { Some(num + start) } else { None })
        .collect::<Vec<usize>>()
}

fn get_sieve(max: usize) -> Vec<usize> {
    let mut sieve = vec![true; max as usize];

    sieve[0] = false;
    sieve[1] = false;
    for i in 2..max {
        if sieve[i as usize] {
            let mut j = i * i;
            while j < max {
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
