// allow for multithreading
use std::thread;

// pixel_canvas for graphics
use pixel_canvas::{Canvas, Color};

// width and height of the rendered image
const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

// what part of the set to render
const RMIN: f64 = -1.8;
const RMAX: f64 = 0.8;
const ICENTER: f64 = 0.0;

// how many iterations before including a number in the set
const ITERATIONS: u32 = 1000;

// the "darkness" of the area just outside the set
const COLORFACTOR: u32 = 300;

// amount of threads to use
const THREADS: u32 = 8;

// calculates how "wide" the views are on the axi
const RWIDTH: f64 = RMAX - RMIN;
const IWIDTH: f64 = RWIDTH * HEIGHT as f64 / WIDTH as f64;

// what imaginary coordinates to include
const IMIN: f64 = ICENTER - IWIDTH / 2.0;

// amount of lines one thread will compute
const THREADLINES: u32 = HEIGHT / THREADS;

const RRES: f64 = RWIDTH / WIDTH as f64;
const IRES: f64 = IWIDTH / HEIGHT as f64;

fn main () {
    // create the canvas to draw on
    let canvas = Canvas::new(WIDTH as usize, HEIGHT as usize)  // set the size 
        .render_on_change(true) // only render one time, there is no state change anyway (because not listening for mouse events)
        .title("mandelbrot");   // set the title

    canvas.render(|_, image| { // don't need the mouse argument

        let mut mandelbrotthreads: Vec<std::thread::JoinHandle<Vec<[u8; WIDTH as usize]>>> = Vec::new();
        for i in 0..THREADS {
            mandelbrotthreads.push(thread::spawn(move || {
                mandelbrotrow(i)
            }));
        }

        let mut results: Vec<Vec<[u8; WIDTH as usize]>> = Vec::new();

        for (_, thread) in mandelbrotthreads.into_iter().enumerate() {
            results.push(thread.join().unwrap());
        }

        // for every row
        for (y, row) in image.chunks_mut(WIDTH as usize).enumerate() {

            // prevent index out of bounds when you can't divide width by threads nicely
            if y as u32 / THREADS >= THREADLINES {
                continue;
            }

            // get the current row from results
            //                     thread number         line number in thread
            let resultrow = results[y % THREADS as usize][y / THREADS as usize];
            
            // for every pixel
            for (x, pixel) in row.iter_mut().enumerate() {

                // get result from row
                let grayscale = resultrow[x as usize];
                
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

fn mandelbrotrow (n: u32) -> Vec<[u8; WIDTH as usize]> {
    let mut result: Vec<[u8; WIDTH as usize]> = Vec::new();

    // for every line this thread has to calculates
    for y in 0..THREADLINES {
        let mut line: [u8; WIDTH as usize] = [0; WIDTH as usize];

        // get this line's imaginary coordinate
        let i = (y * THREADS + n) as f64 * IRES + IMIN;

        for x in 0..WIDTH {
            // calculate real coordinate. uselessly happens every single line.
            let r = x as f64 * RRES + RMIN;
            
            // does the actual calculation, result is in a (is in set: bool, iterations before
            // being excluded: u32) tuple
            let isinset = inset(r, i);


            // calculate brightness
            line[x as usize] = if isinset.0 {
                0 // black if in set
            } else {
                // the more iterations it "survived" before being excluded, the brighter it is,
                // resulting in a cool glow effect
                ((isinset.1 as f64 / COLORFACTOR as f64).sqrt() * 255.0) as u8
            }
        }
        result.push(line);
    }
    result
}


// checks if a given complex number is in the set
fn inset (r: f64, i: f64) -> (bool, u32) {
    mandelbrot(0.0, 0.0, r, i, ITERATIONS)
}

// one iteration of the mandelbrot set. (p, q): complex number z, (a, b): complex number c
fn mandelbrot (p: f64, q: f64, a: f64, b: f64, i: u32) -> (bool, u32) {
    if i == 0 { // reached the max amount of iterations
        (true, 0)
    } else if infinite(p, q) { // is infinite
        (false, ITERATIONS - i)
    } else {    // go for another iteration
        mandelbrot(a + p.powf(2.0) - q.powf(2.0), 2.0 * p * q + b, a, b, i - 1)
    }
}

// if absolute value < 2 it wanders off to infinity
fn infinite (r: f64, i: f64) -> bool {

   // 2 squared = 4
   r.powf(2.0) + i.powf(2.0) > 4.0
}
