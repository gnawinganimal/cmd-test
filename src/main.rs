
use std::time::Duration;

use rand::Rng;
use smol::Timer;
use system::{System, Lock};

use drivetrain::Drivetrain;

mod system;
mod drivetrain;

#[tokio::main]
async fn main() {
    let mut dt = System::new(Drivetrain);

    loop {
        if rand::thread_rng().gen_range(0..25) == 0 {
            dt.exec(|dt| drive_for_n_secs(dt, 100)).await;
        }

        Timer::after(Duration::from_millis(200)).await;
    }
}

pub async fn drive_for_n_secs(mut dt: Lock<Drivetrain>, n: usize) {
    for i in 0..n {
        println!("{}", i);
        dt.tank(1, 1);
        Timer::after(Duration::from_millis(1000)).await;
    }
}
