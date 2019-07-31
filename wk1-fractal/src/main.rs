extern crate num;
use num::Complex;

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 { // -2 is the largest (by mag) number that doesn't escape
            return Some(i);
        }
    }

    None
}

fn escape_time_julia(mut z: Complex<f64>, limit: u32) -> Option<u32> {
    let c = Complex { re: -0.4, im: 0.6 };

    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 { // -2 is the largest (by mag) number that doesn't escape
            return Some(i);
        }
    }

    None
}

#[test]
fn test_escape_time() {
    assert_eq!(escape_time(Complex { re: 1.0, im: 0.0 }, 100), Some(2));
    assert_eq!(escape_time(Complex { re: 0.2, im: 0.0 }, 100), None);
}


fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) -> Complex<f64> {
    let width = lower_right.re - upper_left.re;
    let height = upper_left.im - lower_right.im;

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point((100, 100), (25,75), Complex { re: -1.0, im: 1.0 }, Complex { re: 1.0, im: -1.0 }),
        Complex { re: -0.5, im: -0.5 }
    );
}

fn render(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = escape_time_julia(point, 255).map_or(0, |count| 255 - count as u8);
        }
    }
}

extern crate image;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}

use std::str::FromStr;

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    // I would kill for do notation or at least sequenceM here
    s.find(separator)
      .and_then(|index| {
          T::from_str(&s[..index])
            .ok()
            .and_then(|l| {
                T::from_str(&s[index + 1..])
                  .ok()
                  .map(|r| (l,r))
            })
      })
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10,20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5,1.5)));
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    parse_pair(s, ',').map(|(re, im)| Complex { re, im })
}

extern crate crossbeam;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        panic!("args: FILE PIXELS UPPERLEFT LOWERRIGHT");
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    // nonconcurrent
    //render(&mut pixels, bounds, upper_left, lower_right);
    let threads = 32;
    let rows_per_band = bounds.1/threads + 1;
    //{
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_ul = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lr = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move || render(band, band_bounds, band_ul, band_lr));
            }
        });
    //}

    write_image(&args[1], &pixels, bounds).expect("error writing png file");
}
