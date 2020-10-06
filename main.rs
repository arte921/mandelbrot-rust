use pixel_canvas::{Canvas, Color};

const RMIN: f64 = -1.7;
const RMAX: f64 = 0.7;

const IMIN: f64 = -1.3;
const IMAX: f64 = 1.3;

const ITERATIONS: i32 = 1000;
const COLORFACTOR: i32 = 500;

const RWIDTH: f64 = RMAX - RMIN;
const IWIDTH: f64 = IMAX - IMIN;

fn main () {
    let canvas = Canvas::new(1000, 1000)
        .title("mandelbrot");

    canvas.render(|_, image| {
        let width = image.width() as i32;
        let height = image.height() as i32;
        
        for (y, row) in image.chunks_mut(width as usize).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let r = x as f64 * RWIDTH / width as f64 + RMIN;
                let i = y as f64 * IWIDTH / height as f64 + IMIN;
                let isinset = inset(r, i);
                let gs = if isinset.0 {
                    0
                } else {
                    (((ITERATIONS - isinset.1) as f64 / COLORFACTOR as f64).sqrt() * 255.0 ) as u8
                } as u8;
                *pixel = Color {
                    r: gs,
                    g: gs,
                    b: gs
                };
            }
        }
    });

}

fn inset (r: f64, i: f64) -> (bool, i32) {
    mandelbrot(0.0, 0.0, r, i, ITERATIONS)
}

fn mandelbrot (p: f64, q: f64, a: f64, b: f64, i: i32) -> (bool, i32) {
    if i == 0 {
        return (true, 0);
    }
    if infinite(p, q) {
        return (false, i);
    }
    mandelbrot(a + p.powf(2.0) - q.powf(2.0), 2.0 * p * q + b, a, b, i - 1)
}

// checks if absolute value < 2 it wanders off to infinity
fn infinite (r: f64, i: f64) -> bool {
   r.powf(2.0) + i.powf(2.0) > 4.0
}
