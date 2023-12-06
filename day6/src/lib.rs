// d(t) = t(T - t) = Tt - t^2
// D = Tt - t^2
// 0 = -t^2 + Tt - D
// t = -T +- sqrt(T^2 - 4(-1)(-D)) / 2(-1)
// t = (-T +- sqrt(T^2 - 4D)) / -2
// t = (T +- sqrt(T^2 - 4D)) / 2
// (T + sqrt(T^2 - 4D)) / 2 - (T - sqrt(T^2 - 4D)) / 2
// 2sqrt(T^2 - 4D) / 2
// sqrt(T^2 - 4D)

use std::io::BufRead;

pub fn num_winning_ways(time: f64, dist: f64) -> u32 {
    let sqrt_part = (time.powf(2f64) - 4f64 * dist).sqrt();
    let top = ((time + sqrt_part) / 2f64).floor() as u32;
    let bot = ((time - sqrt_part) / 2f64).ceil() as u32;
    top - bot + 1
}

pub fn winning_ways_product(reader: impl BufRead) -> u32 {
    let mut lines = reader.lines();
    let times = lines
        .next()
        .expect("missing times")
        .expect("failed to read line")
        .split_whitespace()
        .flat_map(|l| l.parse())
        .collect::<Vec<f64>>();
    let distances = lines
        .next()
        .expect("missing distances")
        .expect("failed to read line")
        .split_whitespace()
        .flat_map(|d| d.parse())
        .collect::<Vec<f64>>();
    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, dist)| num_winning_ways(time, dist))
        .product()
}

pub fn winning_ways(reader: impl BufRead) -> u32 {
    let mut lines = reader.lines();
    let time = lines
        .next()
        .expect("missing time")
        .expect("failed to read line")
        .chars()
        .filter(|c| c.is_digit(/*radix=*/ 10))
        .collect::<String>()
        .parse::<f64>().unwrap();
    let dist = lines
        .next()
        .expect("missing distance")
        .expect("failed to read line")
        .chars()
        .filter(|c| c.is_digit(/*radix=*/ 10))
        .collect::<String>()
        .parse::<f64>().unwrap();
    num_winning_ways(time, dist)
}
