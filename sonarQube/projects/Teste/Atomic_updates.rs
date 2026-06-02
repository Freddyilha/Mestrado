extern crate rand;

use std::sync::{Arc, Mutex};
use std::thread;
use std::cmp;
use std::time::Duration;

use rand::RngExt;

trait Buckets {
    fn equalize<R: RngExt>(&mut self, rng: &mut R);
    fn randomize<R: RngExt>(&mut self, rng: &mut R);
    fn print_state(&self);
}

impl Buckets for [i32] {
    fn equalize<R: RngExt>(&mut self, rng: &mut R) {
        let src = rng.random_range(0..self.len());
        let dst = rng.random_range(0..self.len());

        if dst != src {
            let amount = cmp::min(((dst + src) / 2) as i32, self[src]);
            let multiplier = if amount >= 0 { -1 } else { 1 };

            self[src] += amount * multiplier;
            self[dst] -= amount * multiplier;
        }
    }

    fn randomize<R: RngExt>(&mut self, rng: &mut R) {
        let src = rng.random_range(0..self.len());
        let dst = rng.random_range(0..self.len());

        if dst != src {
            let amount = cmp::min(rng.random_range(0..20), self[src]);

            self[src] -= amount;
            self[dst] += amount;
        }
    }

    fn print_state(&self) {
        println!("{:?} = {}", self, self.iter().sum::<i32>());
    }
}

fn main() {
    let e_buckets = Arc::new(Mutex::new([10; 10]));
    let r_buckets = e_buckets.clone();
    let p_buckets = e_buckets.clone();

    thread::spawn(move || {
        let mut rng = rand::rng();

        loop {
            let mut buckets = e_buckets.lock().unwrap();
            buckets.equalize(&mut rng);
        }
    });

    thread::spawn(move || {
        let mut rng = rand::rng();

        loop {
            let mut buckets = r_buckets.lock().unwrap();
            buckets.randomize(&mut rng);
        }
    });

    let sleep_time = Duration::new(1,0);

    loop {
        {
            let buckets = p_buckets.lock().unwrap();
            buckets.print_state();
        }

        thread::sleep(sleep_time);
    }
}
