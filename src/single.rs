use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn start(file_path: &String) {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut cites: HashMap<String, (f64, f64, f64, u32)> = HashMap::with_capacity(9000);
    while reader.read_line(&mut buffer).unwrap() != 0 {
        let (city_str, temp_str) = buffer.trim_end().split_once(';').unwrap();
        let temp = temp_str.parse::<f64>().unwrap();
        cites
            .entry(city_str.to_string())
            .and_modify(|v| {
                v.0 = v.0.min(temp);
                v.1 += temp;
                v.2 = v.2.max(temp);
                v.3 += 1
            })
            .or_insert((temp, temp, temp, 1));
        buffer.clear();
    }
    // Abha=-23.0/18.0/59.2
    let mut final_cities: Vec<String> = cites
        .into_iter()
        .map(|(city, (min, sum, max, count))| {
            format!("{}={:.1}/{:.1}/{:.1}", city, min, sum / count as f64, max)
        })
        .collect();
    final_cities.sort();
    for value in final_cities {
        println!("{}", value);
    }
}
