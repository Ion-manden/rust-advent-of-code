mod reader;
mod year_2022;
mod year_2023;

use std::{fmt::Display, time::Instant};

use reader::reader::create_reader_from_file;

fn main() {
    year_2022::solve();
    year_2023::solve();
}
