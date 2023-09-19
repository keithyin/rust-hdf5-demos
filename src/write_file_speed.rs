use std::collections::HashSet;
use std::io::{Seek, SeekFrom};
use std::thread;
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::time;


fn dump_a_file(filename: String, num_bytes: usize) {
    // buffer 越大，写的速度越快
    let dump_data = vec!['A' as u8; 1024 * 256];
    let now = time::Instant::now();
    let mut file = File::create(filename).unwrap();
    for _ in 0..(num_bytes / dump_data.len()) {
        file.write_all(&dump_data);
    }

    let millis = now.elapsed().as_millis();

    let mB_per_secs = (num_bytes as f64) / (millis as f64 / 1000f64) / 1024f64 / 1024f64;

    println!("dump elapsed:{:?}, mB/s:{:?}", now.elapsed().as_millis(), mB_per_secs);
}

// parallel write to a file is feasible
fn write_to_file(filename: String, offset: usize, value: u8, buffer_size: usize, num_bytes: usize) {
    let mut file = File::options().write(true).open(filename).unwrap();
    let dump_data = vec![value as u8; buffer_size]; 
    file.seek(SeekFrom::Start(offset as u64));
    for _ in 0..(num_bytes / buffer_size) {
        // file.write_all();
        file.write_all(&dump_data);
    }
}


fn write_to_file_parallel(filename: &str, num_threads: usize, num_bytes: usize, buffer_size: usize) {
    let now = time::Instant::now();
    let mut threads = vec![];
    let mut offset = 0usize;
    let per_thread_bytes = num_bytes / num_threads;
    for i in 0..num_threads {
        let f = filename.to_string();
        threads.push(
            thread::spawn(move || {
                write_to_file(f, offset, 'A' as u8 + (i as u8), buffer_size, per_thread_bytes);
            })
        );

        offset += per_thread_bytes;
    }

    for t in threads {
        t.join();
    }
    let millis = now.elapsed().as_millis();

    let mB_per_secs = (num_bytes as f64) / (millis as f64 / 1000f64) / 1024f64 / 1024f64;

    println!("parallel elapsed:{:?}, mB/s:{:?}", now.elapsed().as_millis(), mB_per_secs);
}


fn write_to_file_single(filename: &str, num_bytes: usize, buffer_size: usize) {
    let now = time::Instant::now();
    let f1 = filename.to_string();
    write_to_file(f1, 0, 'G' as u8, buffer_size, num_bytes);

    let millis = now.elapsed().as_millis();

    let mB_per_secs = (num_bytes as f64) / (millis as f64 / 1000f64) / 1024f64 / 1024f64;

    println!("single thread elapsed:{:?}, mB/s:{:?}", now.elapsed().as_millis(), mB_per_secs);
}

pub fn speed_test_main() {
    let filename = "b.txt";
    let tot_bytes: usize = 1024 * 1024 * 1024 * 4; //20G

    // 1161mB/s
    dump_a_file(filename.to_string(), tot_bytes);
    
    // 137mB/s
    write_to_file_parallel(filename, 2, tot_bytes, 1024);
    // 262mB/s
    write_to_file_single(filename, tot_bytes, 1024);
}
