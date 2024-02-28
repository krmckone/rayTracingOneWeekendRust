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
            let r = i as f64 / (image_width as f64 - 1.0);
            let g = j as f64 / (image_height as f64 - 1.0);
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{ir} {ig} {ib}");
            i += 1;
        }
        j += 1;
    }
}