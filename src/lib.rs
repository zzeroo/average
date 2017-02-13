#![feature(test)]

extern crate chrono;
extern crate rand;
extern crate test;

use chrono::prelude::*;
use rand::Rng;
use std::time::{Duration, Instant};


fn sum(values: &Vec<(u32, Instant)>) -> Option<f64> {
    let len = values.len();
    let mut weight = len;
    let mut sum: f64 = 0.0;
    for &(x, _) in values {
        sum += x as f64 * (weight as f64/ len as f64);
        weight -= 1;
    }

    Some(sum)
}

pub fn run() {
    let thirty_minutes = Duration::new(1800, 0);
    let three_second = Duration::new(3, 0);
    let start = Instant::now();
    let mut values: Vec<(u32, Instant)> = vec![];

    // Main Loop
    loop {
        // Sensorewerte auslesen
        // For the test, generate some random values
        let random_value = rand::thread_rng().gen_range(0, 1023);
        // ADC/ random_values mit dem aktuellen Zeitstempel in einm Tuppel in ein Vector speichern
        values.push((random_value, Instant::now()));

        // Nur die Werte der letzten 30 Minuten behalten
        values.retain(|&(_, x)| x.elapsed() <= thirty_minutes);

        let bits = values.len() * ::std::mem::size_of::<Vec<(u32, Instant)>>();
        let bytes = bits/8;
        let kbytes = bytes/1024;
        // Mittelwert aus diesen berechnen
        println!("elapsed: {}s, values: {}, Num Values: {}, {}B, {}kB",
            start.elapsed().as_secs(),
            values.len(),
            bits,
            bytes,
            kbytes,
        );

        // // Sleep random
        // ::std::thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(300, 1000)));
        // Sleep 1Sek
        ::std::thread::sleep(Duration::from_millis(1000));
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn gen_random_values() -> Vec<(u32, Instant)> {
        let mut values: Vec<(u32, Instant)> = vec![];

        for i in 0..1801 {
            let random_value = rand::thread_rng().gen_range(0, 1023);
            // ADC/ random_values mit dem aktuellen Zeitstempel in einm Tuppel in ein Vector speichern
            values.push((random_value, Instant::now()));
        }

        values
    }

    fn gen_3values() -> Vec<(u32, Instant)> {
        let mut values: Vec<(u32, Instant)> = vec![];

        for i in 1..4 {
            values.push((i, Instant::now()));
        }

        values
    }

    #[test]
    fn test_sum() {
        let values = gen_3values();
        assert_eq!(sum(&values).unwrap(), 3.333333333333333);
    }

    #[bench]
    fn bench_sum(b: &mut Bencher) {
        let values = gen_random_values();
        b.iter(|| sum(&values));
    }
}
