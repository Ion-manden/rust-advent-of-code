pub mod reader {
    use std::fs::File;
    use std::io::BufReader;

    pub fn create_reader_from_file(filepath: String) -> BufReader<File> {
        let f = File::open(filepath).unwrap();
        BufReader::new(f)
    }
}
