use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::thread::{self, JoinHandle};

pub fn start(file_path: &String) {
    let mut file = File::open(file_path).unwrap();
    let total_size = file.metadata().unwrap().len();
    let max_workers = thread::available_parallelism().unwrap().get() as u64;
    let intervals = total_size / max_workers as u64;
    let mut start = 0u64;
    let mut handles: Vec<JoinHandle<HashMap<String, (f64, f64, f64, u32)>>> = vec![];
    for i in 1..max_workers + 1 {
        let mut end = i * intervals;
        if i < max_workers {
            let _ = file.seek(SeekFrom::Start(end));
            let mut buffer = [0; 1];
            while buffer[0] != 10 {
                let _ = file.read_exact(&mut buffer);
                end += 1;
            }
        } else {
            end = total_size
        }
        let file_path = file_path.clone();
        let hand = thread::spawn(move || {
            let mut file = File::open(file_path).expect("Error");
            let _ = file.seek(SeekFrom::Start(start));
            let mut reader = BufReader::new(file);
            let mut buffer = String::new();
            let mut cites: HashMap<String, (f64, f64, f64, u32)> = HashMap::with_capacity(9000);
            while start < end {
                start += reader.read_line(&mut buffer).unwrap() as u64;
                let (city_str, temp_str) = buffer.trim_end().split_once(';').unwrap();
                let temp = temp_str.parse::<f64>().unwrap();
                cites
                    .entry(city_str.to_string())
                    .and_modify(|v| {
                        v.0 = v.0.min(temp);
                        v.1 = v.1.max(temp);
                        v.2 += temp;
                        v.3 += 1
                    })
                    .or_insert((temp, temp, temp, 1));
                buffer.clear();
            }
            return cites;
        });
        handles.push(hand);
        start = end;
    }
    let mut cites: HashMap<String, (f64, f64, f64, u32)> = HashMap::with_capacity(9000);
    for hand in handles {
        for (city, (min, max, sum, count)) in hand.join().unwrap() {
            cites
                .entry(city)
                .and_modify(|v| {
                    v.0 = v.0.min(min);
                    v.1 = v.1.max(max);
                    v.2 += sum;
                    v.3 += count
                })
                .or_insert((min, max, sum, count));
        }
    }
    // Abha=-23.0/18.0/59.2
    let mut final_cities: Vec<String> = cites
        .into_iter()
        .map(|(city, (min, max, sum, count))| {
            format!("{}={:.1}/{:.1}/{:.1}", city, min, sum / count as f64, max)
        })
        .collect();
    final_cities.sort();
    for value in final_cities {
        println!("{}", value);
    }
}
