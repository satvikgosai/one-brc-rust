use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::multi::{calculate_hash, parse_int};

pub fn start(file_path: &String) {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::with_capacity(1024 * 1024 * 64, file); // 64 MB buffer
    let mut buffer = Vec::new();
    let mut cites: HashMap<u64, (i64, i64, i64, u32, Vec<u8>)> = HashMap::with_capacity(9000);
    while reader.read_until(b'\n', &mut buffer).unwrap() != 0 {
        let s = buffer.iter().position(|&byte| byte == 59).unwrap();
        let city_bytes = &buffer[0..s];
        let temp = parse_int(&buffer, s + 1);
        let city = calculate_hash(city_bytes);
        cites
            .entry(city)
            .and_modify(|v| {
                v.0 = v.0.min(temp);
                v.1 += temp;
                v.2 = v.2.max(temp);
                v.3 += 1
            })
            .or_insert((temp, temp, temp, 1, city_bytes.to_vec()));
        buffer.clear();
    }
    // Abha=-23.0/18.0/59.2
    let mut final_cities: Vec<String> = cites
        .into_iter()
        .map(|(_, (min, sum, max, count, city_bytes))| {
            format!(
                "{}={:.1}/{:.1}/{:.1}",
                String::from_utf8(city_bytes).unwrap(),
                min as f64 / 10.0,
                sum as f64 / (count as f64 * 10.0),
                max as f64 / 10.0
            )
        })
        .collect();
    final_cities.sort();
    for value in final_cities {
        println!("{}", value);
    }
}
