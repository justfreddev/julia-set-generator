use image::{ ImageBuffer, Rgb };
use num_complex::Complex;

fn main() {
    // An array of different complex numbers that result in cool Julia Sets
    // when used as the C value.
    static JULIA_SETS: [Complex<f32>; 12] = [
        Complex::new(-0.4, 0.6),
        Complex::new(-0.835, -0.321),
        Complex::new(0.285, 0.0),
        Complex::new(0.285, 0.01),
        Complex::new(0.45, 0.1428),
        Complex::new(-0.70176, -0.3842),
        Complex::new(-0.835, -0.2321),
        Complex::new(-0.8, 0.156),
        Complex::new(-0.7269, 0.1889),
        Complex::new(0.0, 0.8),
        Complex::new(0.35, 0.35),
        Complex::new(0.4, 0.4),
    ];

    // Set the constants for the width and height of the output image
    const WIDTH: u32 = 8000;
    const HEIGHT: u32 = 6000;

    // Sets the scale values for the image
    // Because julia sets are often in the range of -1.5 -> 1.5 on the complex plane
    // The 3.0 must be divided by the width, to calculate how much each pixel translates
    // to the complex plane
    const SCALE_X: f32 = 3.0 / (WIDTH as f32);
    const SCALE_Y: f32 = 3.0 / (HEIGHT as f32);

    const MAX_ITERATIONS: u32 = 2000; // The max number of iterations before the z value is deemed 'bounded'

    const ESCAPE_RADIUS: f32 = 2.0; // The radius which if |z| exceeds, z is deemed 'infinite' and 'unbounded'

    // Sets the constant for c using a complex number of choice from the array
    let c: Complex<f32> = JULIA_SETS[1];

    // Initialises the image buffers
    let mut blue_image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(WIDTH, HEIGHT);
    let mut white_image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(WIDTH, HEIGHT);

    // Loops through all the pixels in the image buffers, and sets them all to the RGB value (0, 0, 100)
    for (_x, _y, pixel) in blue_image_buffer.enumerate_pixels_mut() {
        *pixel = image::Rgb([0, 0, 100]);
    }

    // Loops through each pixel in the image
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            // Gets the x and y value relative to the complex plane
            let zx = (x as f32) * SCALE_X - 1.5;
            let zy = (y as f32) * SCALE_Y - 1.5;

            let mut z = Complex::new(zx, zy); // Sets the z value from the x (a) and y (b) value where z=a+bi

            let mut i = 0;

            // Creates new z values using the formula z(n+1) = z(n)**2 + c whilst the number of max iterations
            // has not been exceeded, and the point of z remains within the circle with the radius of 2
            while i < MAX_ITERATIONS && z.norm() < ESCAPE_RADIUS {
                z = z * z + c;
                i += 1;
            }

            // Uses escape time colouring to colour each pixel by how many iterations it takes for the
            // point to escape.
            let pixel = blue_image_buffer.get_pixel_mut(x, y);
            let a = ((i % 64) * 4) as u8;
            *pixel = image::Rgb([a, a, 255]);

            let pixel = white_image_buffer.get_pixel_mut(x, y);
            let b = ((i % 64) * 4) as u8;
            *pixel = image::Rgb([b, b, b]);
        }
    }

    // Saves both the image buffers to a .png file
    blue_image_buffer.save("fractal-blue.png").unwrap();
    white_image_buffer.save("fractal-white.png").unwrap();
}
