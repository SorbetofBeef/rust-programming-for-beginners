// Topic: Channels
//
// Summary:
// Create a program that simulates an internet-of-things remote control light bulb.
// The color of the light can be changed remotely. Use threads and channels to
// communicate what color the light bulb should display.
//
// Requirements:
// * Create a separate thread representing the light bulb
// * Use a channel to communicate with the thread
// * Display a color change message using the println! macro
// * The light bulb must also be able to turn on and off
//
// Notes:
// * Remember to add `crossbeam-channel` to your Cargo.toml file
// * Use the `colored` crate if you want to get fancy and display actual colors
// * The docs.rs site can be used to read documentation for third-party crates

use colored::*;
use crossbeam_channel::unbounded;
use std::thread;

enum LightMsg {
    ChangeColor(u8, u8, u8),
    Disconnect,
    Off,
    On,
}

fn main() {
    let (s, r) = unbounded();

    let light = thread::spawn(move || loop {
        if let Ok(msg) = r.recv() {
            match msg {
                LightMsg::ChangeColor(r, g, b) => {
                    println!("{}", "               ".on_truecolor(r, g, b))
                }
                LightMsg::Disconnect => {
                    println!("disconnecting...");
                    break;
                }
                LightMsg::Off => println!("light off"),
                LightMsg::On => println!("light on"),
            }
        } else {
            println!("channel disconnected");
        }
    });

    s.send(LightMsg::ChangeColor(255, 0, 0));
    s.send(LightMsg::ChangeColor(0, 128, 0));
    s.send(LightMsg::ChangeColor(0, 0, 255));
    s.send(LightMsg::Disconnect);

    light.join();
}
