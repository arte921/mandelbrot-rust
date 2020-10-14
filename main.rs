// to allow for multithreading
use std::thread;

// pixel_canvas for graphics
use pixel_canvas::{Canvas, Color};

// width and height of the rendered image
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

// what part of the set to render
const RMIN: f64 = -1.8;
const RMAX: f64 = -1.2;
const ICENTER: f64 = 0.0;

// calculates how "wide" the views are on the axi
const RWIDTH: f64 = RMAX - RMIN;
const IWIDTH: f64 = RWIDTH * HEIGHT as f64 / WIDTH as f64;

const HALFIWIDTH: f64 = IWIDTH / 2.0;

// what imaginary coordinates to include
const IMIN: f64 = ICENTER - HALFIWIDTH;

// how many iterations before including a number in the set
const ITERATIONS: u32 = 1000;

// the "brightness" of the area just outside the set
const COLORFACTOR: u32 = 300;

// amount of threads to use
const THREADS: u32 = 8;

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

        // for every row and collumn
        for (y, row) in image.chunks_mut(WIDTH as usize).enumerate() {
            let resultrow = results[y % THREADS as usize][y / THREADS as usize];
            for (x, pixel) in row.iter_mut().enumerate() {
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

    for y in 0..THREADLINES {
        let mut line: [u8; WIDTH as usize] = [0; WIDTH as usize];

        for x in 0..WIDTH {
            // calculate real and imaginary coordinate on grid
            let r = x as f64 * RRES + RMIN;
            let i = (y * THREADS + n) as f64 * IRES + IMIN;
            
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
fn mandelbrot (p: f64, q: f64, a: f64, b: f64, n: u32) -> (bool, u32) {
    // iterative solution runs faster than recursive, but less elegant

    let mut p = p;
    let mut q = q;

    for i in 0..n {

        if infinite(p, q) { 
            return (false, i);
        }

        let t = p;

        p = a + p.powf(2.0) - q.powf(2.0);
        q = 2.0 * t * q + b;

    }

    (true, 0)
/*  
    // has reached the max amount of iterations
    if i == 0 { 
        return (true, 0);
    }
    
    // not in set
    if infinite(p, q) { 
        return (false, i);
    }

    // try another iteration
    mandelbrot(a + p.powf(2.0) - q.powf(2.0), 2.0 * p * q + b, a, b, i - 1)*/
}

// if absolute value < 2 it wanders off to infinity
fn infinite (r: f64, i: f64) -> bool {

   // not squaring for performance reasons, it's not needed anyway
   r.powf(2.0) + i.powf(2.0) > 4.0
}
