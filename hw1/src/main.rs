use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;
use std::time::Instant;

const MAX_N: usize = 100000000;
const THREAD_COUNT: usize = 8;

fn main() {
    let sqrt = f64::sqrt(MAX_N as f64).ceil() as usize;

    let start = Instant::now();
    let small_sieve = get_sieve(sqrt);

    let primes = Arc::new(Mutex::new(small_sieve.clone()));

    let segments = THREAD_COUNT as usize;
    let segment_size: usize = (MAX_N - sqrt) / segments;
    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..segments {
        let arc_sieve = Arc::new(small_sieve.clone());
        let prime_clone = primes.clone();

        let thread = std::thread::spawn(move || {
            let sqrt_sieve = arc_sieve.clone();

            let start = sqrt + i * segment_size;
            let end = if sqrt + segment_size * (i + 1) < MAX_N {
                sqrt + segment_size * (i + 1)
            } else {
                MAX_N
            };

            let mut sieve = vec![true; segment_size];

            for prime in sqrt_sieve.iter() {
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
            prime_clone
                .lock()
                .unwrap()
                .append(&mut convert_to_num(&sieve, start));
        });

        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let end = start.elapsed();

    let mut new_primes = primes.lock().unwrap().clone();
    new_primes.sort();

    let mut out_file = File::create("primes.txt").unwrap();

    [
        format!("Time taken: {:?}", end),
        format!("Primes: {:?}", new_primes.len()),
        format!("Sum: {:?}", new_primes.iter().sum::<usize>()),
        format!("Last 10: {:#?}", &new_primes[(new_primes.len() - 10)..]),
    ]
    .iter()
    .for_each(|msg| {
        out_file
            .write_all((msg.to_owned() + "\n").as_bytes())
            .unwrap()
    });
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
