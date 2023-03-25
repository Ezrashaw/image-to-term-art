#![feature(iter_array_chunks)]

use std::{
    env,
    io::{self, Write},
    io::{stdout, StdoutLock},
};

use image::{imageops::FilterType, io::Reader, DynamicImage};
use termsize::Size;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() == 1 {
        "walnut.jpg"
    } else {
        &args[1][..]
    };

    let Size { cols, rows } = termsize::get().unwrap();
    let rows = rows * 2 - 5;

    let img = Reader::open(path).unwrap().decode().unwrap().resize(
        cols as u32,
        rows as u32,
        FilterType::Triangle,
    );

    print_image(img).unwrap();
}

fn print_image(img: DynamicImage) -> io::Result<()> {
    let mut handle = stdout().lock();

    for [top_row, btm_row] in img.into_rgb8().rows().array_chunks::<2>() {
        for (&top, &btm) in top_row.zip(btm_row) {
            print_cell(&mut handle, top.0, btm.0)?;
        }
        writeln!(handle, "\x1B[0m")?;
    }

    Ok(())
}

fn print_cell(handle: &mut StdoutLock, top: [u8; 3], btm: [u8; 3]) -> io::Result<()> {
    write!(handle, "\x1B[38;2;{};{};{}m", top[0], top[1], top[2])?;
    write!(handle, "\x1B[48;2;{};{};{}m", btm[0], btm[1], btm[2])?;

    write!(handle, "▀")
}
