#[cfg(test)]
mod tests {
    use std::fs::File;
    use log::{debug, error, info};
    use std::io::Write;
    use std::time;

    #[test]
    fn write() {
        let table_path = "./test.txt";

        let mut file = File::create(table_path).unwrap();
        let bytes_array = [0 as u8; 409600000];
        let now = time::Instant::now();
        file.write(&bytes_array);
        let file_size = file.metadata().unwrap().len();
        println!("write finished, file size: {}, time spent: {}", file_size, now.elapsed().as_millis());
        // assert read success

        // continue write, confirm write success
        let bytes_array = [0 as u8; 409600000];
        let now = time::Instant::now();
        file.write(&bytes_array);
        let file_size = file.metadata().unwrap().len();
        println!("write finished, file size: {}, time spent: {}", file_size, now.elapsed().as_millis());

        // create smaller file, confirm that the file content is truncated
        let mut file = File::create(table_path).unwrap();
        let bytes_array = [0 as u8; 40960000];
        let now = time::Instant::now();
        file.write(&bytes_array);
        let file_size = file.metadata().unwrap().len();
        println!("write finished, file size: {}, time spent: {}", file_size, now.elapsed().as_millis());

        // wait a second, confirm file length
    }
}
