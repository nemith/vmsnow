use console::{Term, style};
use rand::Rng;
use rand::seq::{IteratorRandom, SliceRandom};
use std::io::{stdout, Write};
use std::{thread, time};

struct Flake {
    x: u16,
    y: u16,
    r#char: char,
    color: u8,
}

const FLAKES: &str = "*❄.҉֍۞";

fn main() {
    let colors: Vec<u8> = vec!(7,8,14,15,31,39,51,74,75,87,105,110,111,117,147,153,249,250,251,252,253,254,255);

    let term = Term::stdout();
    term.hide_cursor().unwrap();
    term.clear_screen().unwrap();

    let (h, w) = term.size();
    let mut rng = rand::thread_rng();
    let mut flakes: Vec<Flake> = Vec::new();

    loop {
        flakes.push(Flake {
            x: rng.gen_range(0..w),
            y: 0,
            r#char: FLAKES.chars().choose(&mut rng).unwrap(),
            color: *colors.choose(&mut rng).unwrap(),
        });

        flakes.retain_mut(|flake| {
            // print new flake location
            term.move_cursor_to(flake.x as usize, flake.y as usize)
                .unwrap();
            print!("{}", style(flake.r#char).color256(flake.color));

            // clear old flake
            if flake.y > 0  {
            term.move_cursor_to(flake.x as usize, flake.y as usize - 1)
                .unwrap();
                term.clear_chars(1).unwrap();
            }

            // move the flake
            flake.y += 1;

            // remove any flake that has reached the bottom
            flake.y != h
        });

        stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
    }
}
