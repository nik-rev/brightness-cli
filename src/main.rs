mod cli;
use ::brightness::blocking as brightness;
use ::brightness::blocking::Brightness as _;

fn main() {
    let all = brightness::brightness_devices().next().unwrap().unwrap();

    let current = all.get();

    println!("Hello, world!");
}
