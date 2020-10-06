use pixel_canvas::{Canvas, Color};

// what real coordinates to include
const RMIN: f64 = -1.7;
const RMAX: f64 = 0.7;

// what imaginary coordinates to include
const IMIN: f64 = -1.3;
const IMAX: f64 = 1.3;

// how many iterations before including a number in the set
const ITERATIONS: i32 = 1000;

// the "brightness" of the area just outside the set
const COLORFACTOR: i32 = 300;

// calculates how "wide" the views are on the axi
const RWIDTH: f64 = RMAX - RMIN;
const IWIDTH: f64 = IMAX - IMIN;

fn main () {

    // create the canvas to draw on
    let canvas = Canvas::new(800, 800)  // set the size
        .render_on_change(true) // only render one time, there is no state change anyway (because not listening for mouse events)
        .title("mandelbrot");   // set the title

    canvas.render(|_, image| { // don't need the mouse argument

        // get width and height of window
        let width = image.width() as i32; 
        let height = image.height() as i32;
        
        // for every row and collumn, thuse have coordinate per pixels in x and y
        for (y, row) in image.chunks_mut(width as usize).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                 
                // calculate real and imaginary coordinate on grid
                let r = x as f64 * RWIDTH / width as f64 + RMIN;
                let i = y as f64 * IWIDTH / height as f64 + IMIN;
                
                // does the actual calculation, result is in a (is in set: bool, iterations before
                // being excluded: i32) tuple
                let isinset = inset(r, i);

                // calculate brightness
                let grayscale = if isinset.0 {
                    0 // black if in set
                } else {
                    // the more iterations it "survived" before being excluded, the brighter it is,
                    // resulting in a cool glow effect
                    (((ITERATIONS - isinset.1) as f64 / COLORFACTOR as f64).sqrt() * 255.0) as u8
                } as u8;
                
                // set the actual color from grayscale
                *pixel = Color {
                    r: grayscale,
                    g: grayscale,
                    b: grayscale
                };
            }
        }
    });
}

// checks if a given complex number is in the set
fn inset (r: f64, i: f64) -> (bool, i32) {
    mandelbrot(0.0, 0.0, r, i, ITERATIONS)
}

// one iteration of the mandelbrot set. (p, q): complex number z, (a, b): complex number c
fn mandelbrot (p: f64, q: f64, a: f64, b: f64, i: i32) -> (bool, i32) {
    
    // has reached the max amount of iterations
    if i == 0 { 
        return (true, 0);
    }
    
    // not in set
    if infinite(p, q) { 
        return (false, i);
    }

    // try another iteration
    mandelbrot(a + p.powf(2.0) - q.powf(2.0), 2.0 * p * q + b, a, b, i - 1)
}

// if absolute value < 2 it wanders off to infinity
fn infinite (r: f64, i: f64) -> bool {

   // not squaring for performance reasons, it's not needed anyway
   r.powf(2.0) + i.powf(2.0) > 4.0
}
