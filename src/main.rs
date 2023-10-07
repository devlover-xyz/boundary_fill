extern crate image;

use std::collections::VecDeque;

use image::{ImageBuffer, Rgb};

fn boundary_fill(
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    x: i32,
    y: i32,
    fill_color: Rgb<u8>,
    boundary_color: Rgb<u8>,
) {
    let width = image.width() as i32;
    let height = image.height() as i32;

    let mut queue = VecDeque::new();
    queue.push_back((x, y));

    while let Some((cur_x, cur_y)) = queue.pop_front() {
        if cur_x < 0 || cur_x >= width || cur_y < 0 || cur_y >= height {
            continue;
        }

        let pixel = *image.get_pixel(cur_x as u32, cur_y as u32);

        if pixel == boundary_color {
            continue; // Skip boundary pixels
        }

        if pixel != fill_color {
            image.put_pixel(cur_x as u32, cur_y as u32, fill_color);

            // Add neighboring pixels to the queue
            queue.push_back((cur_x + 1, cur_y));
            queue.push_back((cur_x - 1, cur_y));
            queue.push_back((cur_x, cur_y + 1));
            queue.push_back((cur_x, cur_y - 1));
        }
    }

}

fn main() {
    let width = 400;
    let height = 400;

    // Create an image initialized with the boundary color
    let mut img = ImageBuffer::from_fn(width, height, |x, y| {
        if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
            Rgb([0, 0, 0]) // Set boundary color for border pixels
        } else {
            Rgb([255, 255, 255]) // Set a different color for non-boundary pixels
        }
    });

    // Set the fill color to yellow (RGB values: [255, 255, 0])
    let fill_color = Rgb([255, 100, 100]);

    // Call the boundary_fill function to fill a closed area starting at coordinates (200, 200)
    boundary_fill(&mut img, 200, 200, fill_color, Rgb([0, 0, 0]));

    // Save the image to a file (e.g., "filled_image.png")
    img.save("filled_image.png").unwrap();
}
