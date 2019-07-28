extern crate image;
extern crate num;

use image::png::PNGEncoder;
use image::ColorType;
use num::complex::Complex;
use std::fs::File;

// if recursion formula is stable (does not tend to infinity) with current displacement,
// the function returns None, else it returns number of iterations needed to 
// leave the stable region |point| <= 2
fn escape_time(displacement: Complex<f32>, limit: usize) -> Option<usize> {
    let mut point = Complex::new(0f32, 0f32);
    for i in 0..limit+1 {
        if point.norm_sqr() > 4f32 {
            return Some(i);
        }
        point = point * point + displacement;
    }
    None
}

fn pixel_to_complex(pixel: (usize, usize), screen_bounds: (usize, usize), complex_bounds: (Complex<f32>, Complex<f32>)) -> Complex<f32> {
    let upper_left = complex_bounds.0;
    let lower_right = complex_bounds.1;
    Complex {
        re: upper_left.re + pixel.0 as f32 / screen_bounds.0 as f32 * (lower_right.re - upper_left.re),
        im: upper_left.im + pixel.1 as f32 / screen_bounds.1 as f32 * (lower_right.im - upper_left.im)
    }
}

fn render_mandelbrot(screen_bounds: (usize, usize), complex_bounds: (Complex<f32>, Complex<f32>)) -> Vec<u8> {
    let mut screen = vec![0; screen_bounds.0 * screen_bounds.1];
    let limit = u8::max_value();
    for x in 0..screen_bounds.0 {
        for y in 0..screen_bounds.1 {
            let point = pixel_to_complex((x, y), screen_bounds, complex_bounds);
            screen[y * screen_bounds.0 + x] = match escape_time(point, limit as usize) {
                Some(degree) => limit - degree as u8,
                None => 0u8
            };
        }
    }
    screen
}

fn write_image(filename: &str, buffer: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let file = File::create(filename)?;
    let encoder = PNGEncoder::new(file);
    encoder.encode(buffer, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
    Ok(())
}

fn main() {
    let screen_bounds = (1000, 750);
    let complex_bounds = (
        Complex { re: -1.20f32, im: 0.35f32 },
        Complex { re: -1.00f32, im: 0.20f32 }
    );
    let screen = render_mandelbrot(screen_bounds, complex_bounds);
    write_image("mandelbrot.png", &screen, screen_bounds).unwrap();
}
