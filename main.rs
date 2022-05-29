use std::fs::File;
use std::io::{self, Write};

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

const FOREGROUND: u32 = 0xFF00FF;
const BACKGROUND: u32 = 0x000000;

fn save_as_ppm(file_path: &str, pixels: &[u32], width: usize, height: usize) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    write!(file, "P6\n{} {} 255\n", width, height)?;

    for y in 0..height {
        for x in 0..width {
            //get pixel location in the 2-D array
            let pixel = pixels[y * width + x];

            // shift 0xff0000 , 2 bytes to the right and take a bitmask -> yields FF(255/red color)
            // do this for each byte in a pixel ( R,G,B )
            let color = [
                ((pixel >> 8 * 2) & 0xFF) as u8,
                ((pixel >> 8 * 1) & 0xFF) as u8,
                ((pixel >> 8 * 0) & 0xFF) as u8,
            ];
            file.write(&color)?;
        }
    }
    Ok(())
}

fn striped_pattern(
    pixels: &mut [u32],
    width: usize,
    height: usize,
    tile_size: usize,
    foreground: u32,
    background: u32,
) {
    for y in 0..height {
        for x in 0..width {
            pixels[y * width + x] = if ((x + y) / tile_size) % 2 == 0 {
                background
            } else {
                foreground
            }
        }
    }
}

fn checker_pattern(
    pixels: &mut [u32],
    width: usize,
    height: usize,
    tile_size: usize,
    foreground: u32,
    background: u32,
) {
    for y in 0..height {
        for x in 0..width {
            pixels[y * width + x] = if (x / tile_size + y / tile_size) % 2 == 0 {
                background
            } else {
                foreground
            }
        }
    }
}

fn solid_circle(
    pixels: &mut [u32],
    width: usize,
    height: usize,
    radius: usize,
    foreground: u32,
    background: u32,
) {
    //centr of the solid_circle
    let cx = (width / 2) as i32;
    let cy = (height / 2) as i32;

    for y in 0..height {
        for x in 0..width {
            let dx = cx - x as i32;
            let dy = cy - y as i32;
            pixels[y * width + x] = if dx * dx + dy * dy <= radius as i32 * radius as i32 {
                foreground
            } else {
                background
            }
        }
    }
}

/// NOTE: In solid_circle func we are doing all the computations in signed but we are accepting the
/// parameters as unsigned , hence we could go to negative domain inside out computation

fn main() {
    const OUTPUT_PATH: &str = "output.ppm";

    //initialize an array of pixels of size 64 * 64;
    let mut pixels = [0u32; WIDTH * HEIGHT];

    //pixels.fill(0x00FF00);
    //striped_pattern(&mut pixels, WIDTH, HEIGHT, 10, FOREGROUND, BACKGROUND);
    //checker_pattern(&mut pixels, WIDTH, HEIGHT, 10, FOREGROUND, BACKGROUND);
    solid_circle(
        &mut pixels,
        WIDTH,
        HEIGHT,
        WIDTH / 2,
        FOREGROUND,
        BACKGROUND,
    );

    save_as_ppm(OUTPUT_PATH, &pixels, WIDTH, HEIGHT).unwrap();
}
