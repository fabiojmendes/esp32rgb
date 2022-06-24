mod led;

use std::{f64::consts::PI, thread, time::Duration};

use esp_idf_sys as _;
use led::{RGB8, WS2812RMT};

const MAX: i32 = 1280;

fn main() -> anyhow::Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let mut rgb_led = WS2812RMT::new()?;
    loop {
        for n in 0..MAX {
            let n = n as f64;
            let m = MAX as f64;
            let scale = 192.0;
            let shift = 96.0;
            let a = (2.0 * PI * n) / m + PI / 2.0;
            // r = sin(a) * scale + shift
            let r = a.sin() * scale + shift;
            let r = r.clamp(0.0, 255.0) as u8;
            // g = sin(a - 2π/3) * scale + shift
            let g = (a - 2.0 * PI / 3.0).sin() * scale + shift;
            let g = g.clamp(0.0, 255.0) as u8;
            // b = sin(a - 4π/3) * scale + shift
            let b = (a - 4.0 * PI / 3.0).sin() * scale + shift;
            let b = b.clamp(0.0, 255.0) as u8;

            let color = RGB8 { r, g, b };
            rgb_led.set_pixel(color)?;
            println!("Color: {color}");
            thread::sleep(Duration::from_millis(50));
        }
    }
}
