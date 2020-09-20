use log_update::LogUpdate;
use std::thread::sleep;
use std::time::Duration;

use std::io;

/// Write text in console.
/// Inputs:
///     @text: Text to print.
///     @speed: Speed of which the letters appear. (millisec)
fn write(text: &str, speed: u64) {
    // Creato output.
    let mut out = LogUpdate::new(io::stdout()).unwrap();
    // Loop for each byte of the text
    for i in 0..text.len() {
        // Render the text
        out.render(&format!("{}", &text[..i])).unwrap();
        // Sleep for the speed
        sleep(Duration::from_millis(speed));
    }
    // Render the end mesage
    out.render(&format!("{}", text)).unwrap();
}

/// Read console input
fn read() -> String {
    let mut res = String::new();
    // Read input
    io::stdin()
        .read_line(&mut res)
        .expect("Failed to read line");
    return res;
}

/// Set constraint to the correct values, and
/// print the start game text
fn start(bot: &mut u8, top: &mut u8) {
    write("Please think a number between 1 and 100.", 50);
    write("I will wait", 50);
    write("===========", 500);
    write("Done?", 50);
    write("Now i am going to guess it!", 50);
    *bot = 1;
    *top = 100;
}

/// Given a constraint print the average as a guess
fn thinking(bot: u8, top: u8) {
    let res: u8 = (bot + (top - bot) / 2) as u8;
    write(&format!("Your number is: {}", res), 50);
}

/// Logic of the guessing
fn check(bot: &mut u8, top: &mut u8) -> bool {
    let mut res;
    res = read();
    if res == "yes\n" {
        write("Yayyy!", 50);
        write("Do you want to play again?", 50);
        res = read();
        if res == "yes\n" {
            start(bot, top);
        } else {
            write("Ohhhh, I hope to see you soon =D", 50);
            return true;
        }
    } else {
        write("Dang, your number is (g)reater or (l)ower?", 5);
        res = read();
        if res == "g\n" {
            *bot = (*bot + (*top - *bot) / 2) as u8;
        } else if res == "l\n" {
            *top = (*top - (*top - *bot) / 2) as u8;
        }
    }
    return false;
}

fn main() {
    // Constraints
    let mut bot: u8 = 0;
    let mut top: u8 = 100;
    // Hollo
    write("Hi! My name is Miraculous", 50);
    // Set and print variables
    start(&mut bot, &mut top);
    loop {
        // Ask if is correct
        thinking(bot, top);
        // Logic
        if check(&mut bot, &mut top) {
            break;
        }
    }
}
