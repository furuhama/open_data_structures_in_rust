#![feature(box_into_raw_non_null)]
extern crate open_data_structures_in_rust;

mod modules;

fn main() {
    modules::read_file::read_file_and_write_from_bottom();
}
