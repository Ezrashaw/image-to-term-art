#![feature(iter_array_chunks)]

use std::env;

use image::{io::Reader, DynamicImage};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() == 1 {
        "walnut.jpg"
    } else {
        &args[1][..]
    };
    let img = Reader::open(path).unwrap().decode().unwrap();

    print_image(img);
}

fn print_image(img: DynamicImage) {
    for [top_row, btm_row] in img.into_rgb8().rows().array_chunks::<2>() {
        for (&top, &btm) in top_row.zip(btm_row) {
            print_cell(top.0, btm.0);
        }
        println!("\x1B[0m");
    }
}

fn print_cell(top: [u8; 3], btm: [u8; 3]) {
    print!("\x1B[38;2;{};{};{}m", top[0], top[1], top[2]);
    print!("\x1B[48;2;{};{};{}m", btm[0], btm[1], btm[2]);
    print!("▀");
}
