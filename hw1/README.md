# Homework 1

This homework assignment is about computing all primes numbers up to 10^8 on 8 threads. My implementation utilizes the Segmented Sieve of Eratosthenes algorithm. The algorithm is described in detail in the [Wikipedia article](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Segmented_sieve). I started out with implementing a regular sieve just to get an idea of how long it takes to compute for 10^8. I noticed that it has a pretty good runtime but it would be hard to parallelize. That's when I decided to implement the segmented sieve. I started by creating the segmented sieve but only sequential to get an idea of the algorithm. Afterwards, I was able to quickly and easily parallelize it due to the processing done in chunks.

## Building/Running

You can build/run the project in two ways:

### First:

```
cargo run --release -- <YOUR_ARGS_HERE>
```

### Second:

```
cargo build --release
./target/release/hw1 <YOUR_ARGS_HERE>
```
