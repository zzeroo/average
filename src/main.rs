extern crate chrono;
extern crate rand;

use chrono::prelude::*;
use rand::Rng;
use std::time::{Duration, Instant};

fn main() {
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
