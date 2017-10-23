extern crate orbclient;
extern crate orbimage;
extern crate stdsimd;

use std::mem::transmute;

use orbclient::{EventIter, Renderer, EventOption};
use orbclient::color::Color;
use orbimage::{Image};

fn main() {
    println!("Hello, world!");

    let flags = vec![orbclient::WindowFlag::Async];
    let title = "simd";
    let width = 800;
    let height = 600;
    let mut basepath = "";

    let mut orb_window = orbclient::Window::new_flags(100, 100, width, height, title, &flags).unwrap();
    let mut image = Image::from_path(basepath.to_string() + "assets/cat.jpg").unwrap();

    'event: loop {
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

//        let a = 0xff;
//        let r = 0x22;
//        let g = 0x0c;
//        let b = 0xe8;
//
//        let mut reset_color = a << 24 | r << 16 | g << 8 | b;
//        orb_window.set(orbclient::Color { data: reset_color});

        //cast Color array to raw u32 data
        //let raw_image_data = unsafe { std::mem::transmute::<&mut[Color], &mut[u32]>(image.data_mut()) };
        let raw_image_data = image.data_mut();

        //todo: do cool stuff here
        {
            //raw_image_data[3204] = 0xff << 24 | 0xff << 16 | 0x00 << 8 | 0x00;//Color::rgba(255, 12, 12, 255);

//            for pixel in raw_image_data.iter_mut() {
//                *pixel = 0xff << 24 | 0xff << 16 | 0x00 << 8 | 0x00;
//            }

            let tint_a = 10;
            let tint_r = 255;
            let tint_g = 0;
            let tint_b = 0;

            for pixel in raw_image_data.iter_mut() {

                //( c.r * c.x + d.r * ( 255 - c.x ) ) / 256;
                let res_r: u16 = (tint_r * tint_a + pixel.r() as u16 *( 255 - tint_a)) / 255;
                let res_g: u16 = (tint_g * tint_a + pixel.g() as u16 *( 255 - tint_a)) / 255;
                let res_b: u16 = (tint_b * tint_a + pixel.b() as u16 *( 255 - tint_a)) / 255;

                *pixel = Color::rgba(res_r as u8, res_g as u8, res_b as u8, tint_a as u8);
//                let raw_pixel = transmute::<&mut Color, &mut u32>(pixel);
//                let t: u32 = tint_a << 24;
//                *raw_pixel = t | res_r | res_g | res_b;
            }
        }

        //cast raw u32 data to Color
        //let raw_image_data = unsafe { std::mem::transmute::<&mut[u32], &mut[Color]>(raw_image_data) };
        orb_window.image(0, 0, 800, 600, raw_image_data);
        orb_window.sync();
    }
}
