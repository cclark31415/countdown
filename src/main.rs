extern crate schedule_recv;
extern crate clearscreen;
use std::io::Write;

fn main() {
    clearscreen::clear().unwrap();

    let tick = schedule_recv::periodic_ms(1000);
    let mut x = 5;
    
    loop {
        //println!("{}", x);
        
        print!("\r{}    ", x);
        std::io::stdout().flush().expect("error message");
        x -= 1;
        tock(&tick);
        if x < 0 {
            println!("\rLIFTOFF!");
            break;
        }
    }
}

fn tock(tick: &std::sync::mpsc::Receiver<()>){
    tick.recv().unwrap();
}