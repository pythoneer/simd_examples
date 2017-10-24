#![feature(toowned_clone_into)]

extern crate orbclient;
extern crate orbimage;
extern crate stdsimd;
extern crate time;

use std::mem::transmute;

use orbclient::{EventIter, Renderer, EventOption};
use orbclient::color::Color;
use orbimage::{Image};
use time::PreciseTime;

struct TintData {
    tint_a: u16,
    tint_r: u16,
    tint_g: u16,
    tint_b: u16,
}

fn main() {
    println!("Hello, world!");

    let flags = vec![orbclient::WindowFlag::Async];
    let title = "simd";
    let width = 800;
    let height = 600;
    let mut basepath = "";

    let mut orb_window = orbclient::Window::new_flags(100, 100, width, height, title, &flags).unwrap();
    let mut image = Image::from_path(basepath.to_string() + "assets/cat.jpg").unwrap();

    let td = TintData{
        tint_a: 100,
        tint_r: 255,
        tint_g: 0,
        tint_b: 0,
    };

    let raw_image_data = image.data_mut();
    let mut new_image_data:Vec<Color> = vec![Color::rgba(0,0,0,0); raw_image_data.len()];

    let mut fcnt = 0;

    'event: loop {

        fcnt += 1;

        for orbital_event in orb_window.events() {
            match orbital_event.to_option() {
                EventOption::Key(key_event) => {
                    match key_event.scancode {
//                        orbclient::K_W => move_forward = key_event.pressed,

                        _ => ()
                    }
                },
                EventOption::Quit(_quit_event) => break 'event,
                _ => (),
            };
        }

        let start = PreciseTime::now();

        //todo: do cool stuff here
        tint1(raw_image_data, &mut new_image_data, &td);

        let end = PreciseTime::now();
        if fcnt % 100 == 0 {
            println!("{} seconds ", start.to(end));
        }

        orb_window.image(0, 0, 800, 600, &new_image_data);
        orb_window.sync();
    }

    fn tint1(raw_image_data: &mut [Color], new_image_data: &mut Vec<Color>, td: &TintData) {
        for (old_pixel, new_pixel) in raw_image_data.iter().zip(new_image_data.iter_mut()) {

            let res_r: u16 = (td.tint_r * td.tint_a + old_pixel.r() as u16 *( 255 - td.tint_a)) / 255;
            let res_g: u16 = (td.tint_g * td.tint_a + old_pixel.g() as u16 *( 255 - td.tint_a)) / 255;
            let res_b: u16 = (td.tint_b * td.tint_a + old_pixel.b() as u16 *( 255 - td.tint_a)) / 255;
            *new_pixel = Color::rgba(res_r as u8, res_g as u8, res_b as u8, 255);
        }
    }
}
