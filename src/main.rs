mod components;
use crate::components::racer::*;
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
    usize,
};

use crossterm::{cursor, terminal, ExecutableCommand};
use rand::Rng;

fn main() {
    let mut stdout = stdout();
    let horse_count = 4;
    let race_length = 40.0;
    let mut horse_disabled = vec![(false, false); horse_count];
    let mut horses = vec![Horse::new(); horse_count];
    let mut pace_dog = 0 as f64;

    // Initialize terminal
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    stdout.execute(cursor::Hide).unwrap();
    println!("Start Race");

    loop {
        // TODO: letztes pferd wird weggenommen -> retry button (mit 1 pferd weniger)
        //
        // TODO: disabled als objekt attribut

        for i in 0..horse_count {
            let broken_leg = rand::thread_rng().gen_bool(0.005);
            if !horse_disabled[i].0 {
                horse_disabled[i].0 = broken_leg;
            } else if !horse_disabled[i].1 {
                horse_disabled[i].1 = broken_leg;
            }

            if !horse_disabled[i].0 && !horse_disabled[i].1 {
                let mut step = rand::thread_rng().gen_range(1..=3) as f64;
                if horse_disabled[i].0 {
                    step /= 2.0;
                }
                horses[i].add_to_distance(step);
                if horses[i].get_distance() >= race_length {
                    horses[i].set_distance(race_length);
                }
            }
        }

        pace_dog += 3.0;
        if pace_dog >= race_length {
            pace_dog = race_length;
        }

        stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();

        let separator = "─".repeat((race_length + 10.0) as usize);
        println!("╭{}╮", separator);
        for i in 0..horse_count {
            let horse = format!("│ Pferd {}: ", i + 1);
            let track = format!(
                "{:<width$}🐎",
                "",
                width = horses[i].get_distance() as usize
            );
            stdout.execute(cursor::MoveTo(0, i as u16 * 2 + 1)).unwrap();
            print!("{}{}", horse, track);

            stdout.execute(cursor::MoveTo(0, i as u16 * 2 + 2)).unwrap();
            println!("├{}┤", separator);
        }
        let dog = format!("│ Hund:    ");
        let track = format!("{:<width$}🦮", "", width = pace_dog as usize);
        println!("{}{}", dog, track);
        println!("╰{}╯", separator);

        stdout.flush().unwrap();

        // Check for winner
        if horses
            .iter()
            .any(|horse| horse.get_distance() >= race_length)
        {
            break;
        }

        // Sleep to simulate time between steps
        thread::sleep(Duration::from_millis(500));
    }

    // Announce winner
    let winner = horses
        .iter()
        .position(|horse| horse.get_distance() >= race_length)
        .unwrap()
        + 1;
    stdout
        .execute(cursor::MoveTo(0, horse_count as u16 * 2 + 3))
        .unwrap();
    println!("Pferd {} gewinnt!", winner);

    stdout.execute(cursor::Show).unwrap();
}
