use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::thread::{self, JoinHandle};

pub fn start(file_path: &String) {
    let mut file = File::open(file_path).expect("Error opening file");
    let total_size = file.metadata().expect("Error opening file").len();
    let max_workers = thread::available_parallelism().unwrap().get() as u64;
    let intervals = total_size / max_workers as u64;
    let mut start = 0u64;
    let mut handles: Vec<JoinHandle<HashMap<u64, (i64, i64, i64, u32, Vec<u8>)>>> = vec![];
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
            let mut file = File::open(file_path).expect("Error opening file");
            let _ = file.seek(SeekFrom::Start(start));
            let mut reader = BufReader::with_capacity(1024 * 1024 * 64, file); // 64 MB buffer
            let mut buffer = Vec::new();
            let mut cites: HashMap<u64, (i64, i64, i64, u32, Vec<u8>)> =
                HashMap::with_capacity(9000);
            while start < end {
                start += reader.read_until(b'\n', &mut buffer).unwrap() as u64;
                let s = buffer.iter().position(|&byte| byte == 59).unwrap();
                let city_bytes = &buffer[0..s];
                let temp = parse_int(&buffer, s + 1);
                let city = calculate_hash(city_bytes);
                cites
                    .entry(city)
                    .and_modify(|v| {
                        v.0 = v.0.min(temp);
                        v.1 = v.1.max(temp);
                        v.2 += temp;
                        v.3 += 1
                    })
                    .or_insert((temp, temp, temp, 1, city_bytes.to_vec()));
                buffer.clear();
            }
            return cites;
        });
        handles.push(hand);
        start = end;
    }
    let mut cites: HashMap<u64, (i64, i64, i64, u32, Vec<u8>)> = HashMap::with_capacity(9000);
    for hand in handles {
        for (city, (min, max, sum, count, city_bytes)) in hand.join().unwrap() {
            cites
                .entry(city)
                .and_modify(|v| {
                    v.0 = v.0.min(min);
                    v.1 = v.1.max(max);
                    v.2 += sum;
                    v.3 += count
                })
                .or_insert((min, max, sum, count, city_bytes));
        }
    }
    // Abha=-23.0/18.0/59.2
    let mut final_cities: Vec<String> = cites
        .into_iter()
        .map(|(_, (min, max, sum, count, city_bytes))| {
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

pub fn parse_int(byte_array: &[u8], s: usize) -> i64 {
    if byte_array[s] == 45 {
        if byte_array[s + 2] == 46 {
            return -(byte_array[s + 1] as i64 * 10 + byte_array[s + 3] as i64 - 528);
        }
        return -(byte_array[s + 1] as i64 * 100
            + byte_array[s + 2] as i64 * 10
            + byte_array[s + 4] as i64
            - 5328);
    }
    if byte_array[s + 1] == 46 {
        return byte_array[s] as i64 * 10 + byte_array[s + 2] as i64 - 528;
    }
    return byte_array[s] as i64 * 100 + byte_array[s + 1] as i64 * 10 + byte_array[s + 3] as i64
        - 5328;
}

pub fn calculate_hash(data: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}
