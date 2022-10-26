mod mandelbrot_gen;
use mandelbrot_gen::mandelbrot_smooth;
use mandelbrot_gen::mandelbrot;

fn main() {
    let output: &str = "with_smooth_71";
    let width = 1920 ;
    let height = 1080;
    let iterations = 100.0;
    let zoom: f64 = 1.0;  //STANDARD 1.0
    let xmove: f64 = 1.5;  //STANDARD 1.0
    let ymove: f64 = 2.5;  //STANDARD 1.0
    let save_image = true;

    let mandel = mandelbrot_smooth::RunMandelbrot::init(output, width, height, iterations, save_image, zoom, xmove, ymove);
    mandel.runalgo();
}


