mod color;
mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{image_width} {image_height}\n255\n");
    let mut j = 0;
    while j < image_height {
        let lines_remaining = image_height - j;
        eprint!("Scanlines remaining: {lines_remaining}\r");
        let mut i = 0;
        while i < image_width {
            let pixel_color = color::make_color(
                i as f64 / (image_width as f64 - 1.0),
                j as f64 / (image_height as f64 - 1.0),
                0.0,
            );
            color::write_color(pixel_color);
            i += 1;
        }
        j += 1;
    }
}
