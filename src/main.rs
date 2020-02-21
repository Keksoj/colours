use rand::Rng;
// use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};
use std::{thread, time};
use termion::color;
use termion::raw::IntoRawMode;
use termion::terminal_size;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut rainbow = Rainbow::new();
    show(&mut stdout, &rainbow);
    quicksort(&mut stdout, &mut rainbow);
    show(&mut stdout, &rainbow);
}

// This struct contains all three vectors of RGB values
#[derive(Debug)]
pub struct Rainbow {
    rgb: Vec<u32>,
    width: u16,
    height: u16,
    length: u16,
}

impl Rainbow {
    // create the red, green and blue vectors, each of the shape [0, 1, 2, ..255]
    pub fn new() -> Rainbow {
        let mut rgb: Vec<u32> = Vec::new();

        let width: u16 = terminal_size().unwrap().0 - 2;
        let height: u16 = terminal_size().unwrap().1 - 2;
        let length = width * height;

        for _n in 0..length {
            // 256*256*256 = 16_777_216
            let random_value: u32 = rand::thread_rng().gen_range(0, 0xFFFFFF);
            rgb.push(random_value);
        }
        Rainbow {
            rgb,
            width,
            height,
            length,
        }
    }
}

// This is Termion's way of filling the terminal with colours
pub fn show<W: Write>(stdout: &mut W, rainbow: &Rainbow) {
    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All
    )
    .unwrap();

    for index in 0..rainbow.length {
        
        if index % rainbow.width == 0 {
            write!(stdout, "\n\r").unwrap();
        }

        // converts the u32 value to a set of rgb values
        // The number of the form 0xFF_DD_EE is split in little endian order:
        // EE: red, DD: green, FF: blue
        let bytes: [u8; 4] = rainbow.rgb[index as usize].to_le_bytes();
        // 0 => random, 1 => waves, 2 => overall trend
        let red = bytes[0];
        let green = bytes[1];
        let blue = bytes[2];

        write!(
            stdout,
            "{} ", // mind the space
            termion::color::Bg(termion::color::Rgb(red, green, blue)),
        )
        .unwrap();
    }
    writeln!(stdout, "{}.", termion::style::Reset).unwrap();
}

////////////////////////////////////////////////////////////////////////////////
//      ___    _   _   ___    ____   _  __  ____     ____    ____    _____    //
//     / _ \  | | | | |_ _|  / ___| | |/ / / ___|   / __ \  |  _ \  |_   _|   //
//    | | | | | | | |  | |  | |     | ' /  \___ \  | |  | | | |_) |   | |     //
//    | |_| | | |_| |  | |  | |___  | . \   ___) | | |__| | |  _ <    | |     //
//     \__\_\  \___/  |___|  \____| |_|\_\ |____/   \____/  |_| \_\   |_|     //
//                                                                            //
////////////////////////////////////////////////////////////////////////////////

// This code is redundant because I needed to use the show() method inside it
// If, like me, you REALLY like too much comments, go check the same
// implementation of quicksort in sorting-colours
fn quicksort<W: Write>(mut stdout: W, rainbow: &mut Rainbow) {
    let mut pivots: Vec<usize> = Vec::new();

    let mut pivot_i = rainbow.rgb.len() - 1;
    let mut index = 0;

    while index < pivot_i {
        if rainbow.rgb[index] > rainbow.rgb[pivot_i] {
            let swaped = rainbow.rgb.remove(index);
            rainbow.rgb.insert(pivot_i, swaped);
            pivot_i -= 1;
        } else {
            index += 1;
        }
    }
    pivots.push(pivot_i);
    show(&mut stdout, &rainbow);

    loop {
        if pivots.len() == rainbow.rgb.len() {
            break;
        }

        let mut temp_pivots: Vec<usize> = Vec::new();
        let mut start_index = 0;
        for existing_pivot_index in pivots.iter() {
            if start_index == *existing_pivot_index {
                start_index += 1;
                continue;
            } else if start_index + 1 == *existing_pivot_index {
                temp_pivots.push(start_index);
                start_index += 2;
                continue;
            }

            let mut new_pivot_index = *existing_pivot_index - 1;
            while start_index < new_pivot_index {
                if rainbow.rgb[start_index] > rainbow.rgb[new_pivot_index] {
                    let swaped = rainbow.rgb.remove(start_index);
                    rainbow.rgb.insert(new_pivot_index, swaped);
                    new_pivot_index -= 1;
                } else {
                    start_index += 1;
                }
            }
            temp_pivots.push(new_pivot_index);
            start_index = existing_pivot_index + 1;
        }

        pivots.append(&mut temp_pivots);
        pivots.sort(); // this kinda defeats the whole purpose, does it?
        show(&mut stdout, &rainbow);

        let mut last_pivot_index: usize = rainbow.rgb.len() - 1;
        if !pivots.contains(&last_pivot_index) {
            while start_index < last_pivot_index {
                if rainbow.rgb[start_index] > rainbow.rgb[last_pivot_index] {
                    let swaped = rainbow.rgb.remove(start_index);
                    rainbow.rgb.insert(last_pivot_index, swaped);
                    last_pivot_index -= 1;
                } else {
                    start_index += 1;
                }
            }

            pivots.push(last_pivot_index);
        }
    }
}
