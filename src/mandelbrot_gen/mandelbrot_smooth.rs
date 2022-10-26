use std::path::Path;
// use num_complex::{Complex64, ComplexFloat,};
use std::time::Instant;
use std::io::{self, Write };
use palette::Pixel;
use palette::encoding::Linear;
 use prisma::{Rgb as prism_Rgb, Hsv, FromColor};
use palette::rgb::{Srgb, Rgb};
extern crate angular_units as angle;
 use angle::Deg;




pub struct RunMandelbrot {
    name: String,
    img_width: u32,
    img_height: u32,
    max_iterations: f64,
    save_image: bool,
    zoom: f64,
    xmove: f64,
    ymove: f64,
}




impl RunMandelbrot {

    pub fn init(output_name: &str, w: u32, h: u32, iter: f64, save: bool, zoom: f64, xmove: f64, ymove: f64) -> Self {
        RunMandelbrot { name: output_name.to_string(),  img_width: w, img_height: h, max_iterations: iter, save_image: save, zoom: zoom, xmove: xmove, ymove: ymove}
    }
    
    pub fn runalgo(&self) {
        print!("Executing {}... ", self.name);
        io::stdout().flush().unwrap();
        let now = Instant::now();
        let img = self.mandelbrot_native();
        let elapsed = now.elapsed().as_millis() as f32 / 1000.0;
        if self.save_image {
            let fname = format!("mandelbrot_{}.png", self.name);
    
            img.save_with_format(Path::new("E:/Rust/Mandelbrot/Training/mandelbrot_native/output").join(fname), image::ImageFormat::Png)
                .unwrap();
        }
        println!("{}s", elapsed);
        
    }


    fn mandelbrot_native(&self) -> image::RgbImage {
        let mut img = image::RgbImage::new(self.img_width as u32, self.img_height as u32);
        for(xx, yy, pixel) in img.enumerate_pixels_mut() {
            // let x0 = ((xx as f64) / (self.img_width as f64 / 1.5 )) * 3.5 - 3.5;
            // let y0 = ((yy as f64) / (self.img_height as f64 / 2.0) ) * 1.5 - 1.5;

            let x0 = ((xx as f64) / ((self.img_width as f64 * self.zoom) / 1.5 )) * 3.5 - 3.5;
            let y0 = ((yy as f64) / ((self.img_height as f64 * self.zoom )/ 2.0) ) * 1.5 - 1.5;

            let mut x = 0.0;
            let mut y = 0.0;
            let mut x2 = 0.0;
            let mut y2 = 0.0;
            let mut iteration: f64 = 0.0;
            let mut xold: f64 = 0.0; 
            let mut yold: f64 = 0.0; 
            let mut period: u32 = 0;

            while x * x + y * y <= 4.0 * 4.0 && iteration < self.max_iterations {

                y = x * y * 2.0 + y0;
                x = x2 - y2 + x0;
                x2 = x * x;
                y2 = y * y;

                iteration += 1.0;

                if f64::abs(x + y) >= 40.0 {
                    break;
                } 
               
                if x.eq(&xold) && y.eq(&yold) {
                    iteration = self.max_iterations;
                    break;
                }

                period += 1;
                if period > 20 {
                    period = 0;
                    xold = x;
                    yold = y;
                }
            }
                

            if iteration < self.max_iterations{
                let smooth: f64 = iteration + 2.0 - f64::ln(f64::ln((x2 + y2) * 2.0)) / 75.0 - 71.0;

                let sda = self.map_color(smooth, x2, y2);

                *pixel = sda;
            }
            else {
                 *pixel = image::Rgb([0,0,0]);
            }
        }
        img
    }

    /// https://imgur.com/a/HUlUz#0
    /// https://www.reddit.com/r/math/comments/2abwyt/smooth_colour_mandelbrot/
    /// https://github.com/cslarsen/mandelbrot-js/blob/master/mandelbrot.js

    fn map_color(&self, iter: f64, r: f64, c:f64) -> image::Rgb<u8> {
        let zn:f64;
        let mut hue:f64;
    
        zn = f64::sqrt(r + c);
        hue = iter + 1.0 - f64::ln(f64::ln(f64::abs(zn))) / f64::ln(2.0);
        hue = 0.95 + 20.0 * hue;

        // println!("Hue 0:  {}", hue);
    
        while hue > 360.0 {
            hue -= 360.0;
        }
    
        while hue < 0.0 {
            hue += 360.0;
        }

        let color = self.rgb_from_hsv(hue, 0.8, 0.9);
        return color;
    }
    

    fn rgb_from_hsv(&self, hue: f64, saturation: f64, mut value: f64) -> image::Rgb<u8> {
        let hi: i64 = f64::floor(hue / 60.0) as i64 % 6;
        let f: f64 = hue / 60.0  - f64::floor(hue / 60.0);
    
        value = value * 255.0;
    
        let v = value as i64;
        let p = (value * (1.0 - saturation)) as i64;
        let q = (value * (1.0 - f * saturation)) as i64;
        let t = (value * (1.0 - (1.0 - f) * saturation)) as i64;


        //Probaly just need to re place letter varibles. right now its primarly green

        if hi == 0
        {
            //  println!("HI 0:  {}   |   {}   |   {}", v, t, p);
            return image::Rgb([v as u8, t as u8, p as u8 ]);
        }
        else if hi == 1 {
            // println!("HI 1:  {}   |   {}   |   {}", v, t, p);
            return image::Rgb([q as u8, v as u8, p as u8]);
        }
        else if hi == 2 {
            return image::Rgb([p as u8, v as u8, t as u8]);
        }
        else if hi == 3 {
            return image::Rgb([p as u8, q as u8, v as u8]);
        }
        else if hi == 4 {
            return image::Rgb([t as u8, p as u8, v as u8]);
        }
        else{
            return image::Rgb([p as u8, p as u8, q as u8]);
        }
    }
    
    
}





