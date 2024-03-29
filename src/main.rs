extern crate schedule_recv;
use std::io::Write;
use rand::Rng;

fn main() {
    // This clears the screen and puts the cursor at the top
    println!("\r\x1b[2J\r\x1b[H");

    let tick = schedule_recv::periodic_ms(1000);
    let mut x = 5;
    
    loop {
        let color = rand::thread_rng().gen_range(41..48);
        match color {
            // Red bg
            41 => print!("\r\x1b[30;41m    {}    \x1b[0m", x),
            // Green bg
            42 => print!("\r\x1b[30;42m    {}    \x1b[0m", x),
            // Yellow bg
            43 => print!("\r\x1b[30;43m    {}    \x1b[0m", x),
            // Blue bg
            44 => print!("\r\x1b[33;44m    {}    \x1b[0m", x),
            // Magenta bg
            45 => print!("\r\x1b[37;45m    {}    \x1b[0m", x),
            // Cyan bg
            46 => print!("\r\x1b[30;46m    {}    \x1b[0m", x),
            // White bg
            47 => print!("\r\x1b[30;47m    {}    \x1b[0m", x),
            // Default
            _ => print!("\r{}    ", x)
        }
        
        std::io::stdout().flush().expect("error message");
        x -= 1;
        tock(&tick);
        if x < 0 {
            println!("\r\x1b[33;41m LIFTOFF! \x1b[0m");
            break;
        }
    }
}

fn tock(tick: &std::sync::mpsc::Receiver<()>){
    tick.recv().unwrap();
}