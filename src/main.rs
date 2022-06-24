mod led;

use std::{f64::consts::PI, thread, time::Duration};

use esp_idf_sys as _;
use led::{RGB8, WS2812RMT};

const MAX: i32 = 1280;

fn main() -> anyhow::Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // let red = RGB8::new(255, 0, 0);
    // let green = RGB8::new(0, 255, 0);
    // let blue = RGB8::new(0, 0, 255);

    let mut rgb_led = WS2812RMT::new()?;
    loop {
        for n in 0..(MAX + 256) {
            let n = n as f64;
            let m = MAX as f64;
            let a = (5.0 * PI * n) / (3.0 * m) + PI / 2.0;
            // r = sin(a) * 256 + 128
            let r = a.sin() * 256.0 + 128.0;
            let r = r.clamp(0.0, 255.0) as u8;
            // g = sin(a - 2π/3) * 256 + 128
            let g = (a - 2.0 * PI / 3.0).sin() * 256.0 + 128.0;
            let g = g.clamp(0.0, 255.0) as u8;
            // b = sin(a - 4π/3) * 256 + 128
            let b = (a - 4.0 * PI / 3.0).sin() * 256.0 + 128.0;
            let b = b.clamp(0.0, 255.0) as u8;
            let color = RGB8 { r, g, b };
            rgb_led.set_pixel(color)?;
            println!("Color: {color}");
            thread::sleep(Duration::from_millis(50));
        }
    }
}
