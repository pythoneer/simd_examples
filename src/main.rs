#![feature(toowned_clone_into)]

extern crate orbclient;
extern crate orbimage;
extern crate stdsimd;
extern crate time;

use std::mem::transmute;
use std::slice;

use orbclient::{EventIter, Renderer, EventOption};
use orbclient::color::Color;
use orbimage::{Image};
use time::PreciseTime;

use stdsimd::simd::{i32x4, u32x4, i16x8, i8x16, u8x16};

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
    let width = 1440;
    let height = 1080;
    let mut basepath = "";

    let mut orb_window = orbclient::Window::new_flags(100, 100, width, height, title, &flags).unwrap();
    let mut image = Image::from_path(basepath.to_string() + "assets/cat_big.jpg").unwrap();

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
//        tint2(raw_image_data, &mut new_image_data, &td);
        tint_sse(raw_image_data, &mut new_image_data, &td);

        let end = PreciseTime::now();
        if fcnt % 100 == 0 {
            println!("{} seconds ", start.to(end));
        }

        orb_window.image(0, 0, width, height, &new_image_data);
        orb_window.sync();
    }

    #[inline(never)]
    fn tint1(raw_image_data: &mut [Color], new_image_data: &mut Vec<Color>, td: &TintData) {
        for (old_pixel, new_pixel) in raw_image_data.iter().zip(new_image_data.iter_mut()) {

            let res_r: u16 = (td.tint_r * td.tint_a + old_pixel.r() as u16 *( 255 - td.tint_a)) / 255;
            let res_g: u16 = (td.tint_g * td.tint_a + old_pixel.g() as u16 *( 255 - td.tint_a)) / 255;
            let res_b: u16 = (td.tint_b * td.tint_a + old_pixel.b() as u16 *( 255 - td.tint_a)) / 255;
            *new_pixel = Color::rgba(res_r as u8, res_g as u8, res_b as u8, 255);
        }
    }

    #[inline(never)]
    fn tint2(raw_image_data: &mut [Color], new_image_data: &mut Vec<Color>, td: &TintData) {
        for (old_pixel, new_pixel) in raw_image_data.iter().zip(new_image_data.iter_mut()) {

            let res_r: u16 = (td.tint_r * td.tint_a + old_pixel.r() as u16 *( 255 - td.tint_a)) >> 8;
            let res_g: u16 = (td.tint_g * td.tint_a + old_pixel.g() as u16 *( 255 - td.tint_a)) >> 8;
            let res_b: u16 = (td.tint_b * td.tint_a + old_pixel.b() as u16 *( 255 - td.tint_a)) >> 8;
            *new_pixel = Color::rgba(res_r as u8, res_g as u8, res_b as u8, 255);
        }
    }

//    fn u32_as_simd(x: &mut [u32]) -> &mut [u32x4] {
//        slice::from_raw_parts(x, x.len() / 4)
//    }

    fn color_as_simd(x: &mut [Color]) -> &mut [i8x16] {
        unsafe {
            let len = x.len();
            let y = transmute::<&mut[Color], &mut[i8x16]>(x);
            slice::from_raw_parts_mut(y.as_mut_ptr(), len / 4)
        }
    }

    #[inline(never)]
    fn tint_sse(raw_image_data: &mut [Color], new_image_data: &mut Vec<Color>, td: &TintData) {
        use stdsimd::vendor;

        let mut color: u32 = 0x00000000;

        color += td.tint_b as u32;
        color += (td.tint_g as u32) << 8;
        color += (td.tint_r as u32) << 16;
        color += (td.tint_a as u32) << 24;

        unsafe {
            let zero = vendor::_mm_setzero_si128();
            let ones = vendor::_mm_set1_epi16(0x00ff);
            let color = vendor::_mm_set1_epi32(color as i32);
            let color = transmute::<i32x4, u32x4>(color);

            let raw = color_as_simd(raw_image_data);
            let new = color_as_simd(new_image_data);

            for (old_pixel_pack, new_pixel_pack) in raw.iter().zip(new.iter_mut()) {
//                println!("{:X} {:X} {:X} {:X}", pixel_pack.extract(0),pixel_pack.extract(1),pixel_pack.extract(2),pixel_pack.extract(3) );

                let dst_pixels = vendor::_mm_load_si128(old_pixel_pack);
                let dst_lo_16 = vendor::_mm_unpacklo_epi8(dst_pixels, zero);
                let dst_hi_16 = vendor::_mm_unpackhi_epi8(dst_pixels, zero);
                let color_16 = vendor::_mm_unpacklo_epi8(transmute::<u32x4, i8x16>(color), zero);

                let alpha = vendor::_mm_shufflelo_epi16(transmute::<i8x16, i16x8>(color_16), 0x02020202);
                let alpha = vendor::_mm_shufflehi_epi16(alpha, 0x02020202);

                let calpha = vendor::_mm_sub_epi16(ones, alpha);
                let mdst_lo_16 = vendor::_mm_mullo_epi16(transmute::<i8x16, i16x8>(dst_lo_16), calpha);
                let mdst_hi_16 = vendor::_mm_mullo_epi16(transmute::<i8x16, i16x8>(dst_hi_16), calpha);
                let mcolor_16 = vendor::_mm_mullo_epi16(transmute::<i8x16, i16x8>(color_16), alpha);

                let rslt_lo_16 = vendor::_mm_add_epi16(mdst_lo_16, mcolor_16);
                let rslt_hi_16 = vendor::_mm_add_epi16(mdst_hi_16, mcolor_16);
                let rslt_lo_16 = vendor::_mm_srli_epi16(rslt_lo_16, 8);
                let rslt_hi_16 = vendor::_mm_srli_epi16(rslt_hi_16, 8);

                let packed_rslt = vendor::_mm_packus_epi16(rslt_lo_16, rslt_hi_16);
                vendor::_mm_store_si128(new_pixel_pack, transmute::<u8x16, i8x16>(packed_rslt));

            }
        }
    }
}
