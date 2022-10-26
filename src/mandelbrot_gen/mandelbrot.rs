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
    max_iterations: u64,
    save_image: bool,
    zoom: f64,
    xmove: f64,
    ymove: f64,
}

struct Complex64 {
    
}




impl RunMandelbrot {

    pub fn init(output_name: &str, w: u32, h: u32, iter: u64, save: bool, zoom: f64, xmove: f64, ymove: f64) -> Self {
        RunMandelbrot { name: output_name.to_string(),  img_width: w, img_height: h, max_iterations: iter, save_image: save, zoom: zoom, xmove: xmove, ymove: ymove}
    }
    
    pub fn runalgo(&self, algo: &str) {
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
            let x0 = ((xx as f64) / (self.img_width as f64 / 1.5)) * 3.5 - 3.5;
            let y0 = ((yy as f64) / (self.img_height as f64 / 2.0) ) * 1.5 - 1.5;

            let mut x = 0.0;
            let mut y = 0.0;
            let mut x2 = 0.0;
            let mut y2 = 0.0;
            let mut iteration: u64 = 0;
            let mut xold: f64 = 0.0; 
            let mut yold: f64 = 0.0; 
            let mut period: u32 = 0;

            while x * x + y * y <= 1.0 * 1.0 && iteration < self.max_iterations {
                // let xtemp = x * x - y * y + x0;
                // y = 2.0 * x * y + y0;
                // x = xtemp;
                y = x * y * 2.0 + y0;
                x = x2 - y2 + x0;
                x2 = x * x;
                y2 = y * y;
                iteration += 1;
               
                // if x.eq(&xold) && y.eq(&yold) {
                //     iteration = self.max_iterations;
                //     break;
                // }

                // period = period + 1;
                // if period > 20 {
                //     period = 0;
                //     xold = x;
                //     yold = y;
                // }

                
            }
                

            if iteration < self.max_iterations {
                let magnitude = x * x + y * y;

                let smooth: f64 = iteration as f64 + 2.0 - f64::ln(f64::ln(magnitude)) / 75.0;
                let smoooth: f64 = iteration as f64 - f64::ln(0.5 * f64::ln(magnitude)) - f64::ln(0.5 * f64::ln(1000.0)) / f64::ln(2.0);
                // println!("smooth: {}", smooth);
                // println!("smooth 2nd: {}", smoooth);

                let srgb1 = self.get_color_int(smooth as u64 * 1.5 as u64);
                

                *pixel = image::Rgb([srgb1[0], srgb1[1], srgb1[2]]);
            }
            else {
                 *pixel = image::Rgb([0,0,0]);
            }

         
        }
        img
        
    }

    fn mandelbrot_NOT_native(&self) -> image::RgbImage {
        let mut img = image::RgbImage::new(self.img_width as u32, self.img_height as u32);
        for(xx, yy, pixel) in img.enumerate_pixels_mut() {
            let x0 = ((xx as f64) / (self.img_width as f64 / 1.5)) * 3.5 - 3.5;
            let y0 = ((yy as f64) / (self.img_height as f64 / 2.0) ) * 1.5 - 1.5;

            let mut x = 0.0;
            let mut y = 0.0;
            let mut x2 = 0.0;
            let mut y2 = 0.0;
            let mut iteration: u64 = 0;
            let mut xold: f64 = 0.0; 
            let mut yold: f64 = 0.0; 
            let mut period: u32 = 0;

            while x * x + y * y <= 2.0 * 2.0 && iteration < self.max_iterations {
                // let xtemp = x * x - y * y + x0;
                // y = 2.0 * x * y + y0;
                // x = xtemp;
                y = x * y * 2.0 + y0;
                x = x2 - y2 + x0;
                x2 = x * x;
                y2 = y * y;
                iteration += 1;
               
                if x.eq(&xold) && y.eq(&yold) {
                    iteration = self.max_iterations;
                    break;
                }

                period = period + 1;
                if period > 20 {
                    period = 0;
                    xold = x;
                    yold = y;
                }

                
            }
                
            if iteration == self.max_iterations {
                // println!("black");
                *pixel = image::Rgb([0, 0, 0])
            }
            else {
                let float_iter = (iteration as f64).floor();
                
                let c1 = self.get_color_int(iteration);
                let c2 = self.get_color_int(iteration + 1);
                *pixel = color_interpolation(c1, c2, iteration % 1);
                //  println!("{},  |  {}  |  {}", test.red() as u32, test.green() as u32, test.blue() as u32 );
                //  let red = test.red().to_bits();
                // println!("bits: {}    |     float: {}", red as u8, test.red());

                // *pixel = image::Rgb([test.red().to_bits() as u8, test.green().to_bits() as u8, test.blue().to_bits() as u8])
            }
            // if iteration < self.max_iterations {

            //     let log_zn = (x * x + y + y).ln() / 2f64;
            //     let nu = (log_zn / f64::ln(2.0)).ln() / f64::ln(2f64) ;

            //     iteration = iteration + 1 - nu as u64; 

            //     let srgb1 = self.test_color(iteration);
            //     let srgb2 = self.get_color_float(iteration + 1);
                
            //     let new = srgb2.into_linear() + srgb1.into_linear();
            //     let pixels: [u8; 3] = Srgb::from_linear(new)
            //                 .into_format()
            //                 .into_raw();
    
            //     // *pixel = color_interpolation(srgb1, srgb2);
            //     *pixel = image::Rgb([pixels[0], pixels[1], pixels[2]]);
            // }
            // else {
            //     *pixel = image::Rgb([0,0,0]);
            // }

         
        }
        img
        
    }

    fn get_color_float(&self, iteration: u64) -> Srgb {
        if iteration < self.max_iterations && iteration > 0 {
            let i = iteration % 16;
            let vec:[Srgb; 16] = [
                Srgb::new(0.28627, 0.11765, 0.05882),
                Srgb::new(0.09804, 0.02745, 0.10196),
                Srgb::new(0.03529, 0.00392, 0.18431),
                Srgb::new(0.01569, 0.01569, 0.28627),
                Srgb::new(0.00000, 0.02745, 0.39216),
                Srgb::new(0.04706, 0.17255, 0.54118),
                Srgb::new(0.09804, 0.32157, 0.69412),
                Srgb::new(0.22353, 0.49020, 0.81961),
                Srgb::new(0.52549, 0.70980, 0.89804),
                Srgb::new(0.82745, 0.92549, 0.97255),
                Srgb::new(0.94510, 0.91373, 0.74902),
                Srgb::new(0.97255, 0.78824, 0.37255),
                Srgb::new(1.00000, 0.66667, 0.00000),
                Srgb::new(0.80000, 0.50196, 0.00000),
                Srgb::new(0.60000, 0.34118, 0.000000),
                Srgb::new(0.41569, 0.20392, 0.01176),
            ];
            return vec[i as usize];
        }
        else {
            return Srgb::new(0.0, 0.6, 0.0)
        }
   }

       //    RGB TO HSV TO RGB      CHANING HUE OH HSV

     fn test_color(&self, i: u64, r: f64, c: f64) -> prism_Rgb<f64> {
         let di: f64 = i as f64;
         let zn: f64;
         let mut hue: f64;

         zn = f64::sqrt(r + c);
         hue = di + 1.0 - f64::ln(f64::ln(f64::abs(zn))) / f64::ln(2.0);
         hue = 0.95 + 20.0 * hue;

         while hue > 360.0 {
            //  println!("2:  {}", hue);
             hue -= 360.0;
          
         }
         while hue < 0.0 {
             hue += 360.0;
         }
         
         let mut rgb = prism_Rgb::new(0.0, 0.0, 0.0);
         // println!("1");
         let mut hsv = Hsv::from_color(&rgb);
        //   println!("2:  {}", hue);
         hsv.set_hue(Deg(hue));
         hsv.set_saturation(0.8);
         hsv.set_value(1.0);
         let new_rgb = prism_Rgb::from_color(&hsv);
         // println!("3");
         new_rgb
     }


     fn get_color_int(&self, iteration: u64) -> image::Rgb<u8> {
         if iteration < self.max_iterations && iteration > 0 {
             let i = iteration % 16;
             let vec:[image::Rgb<u8>; 16] = [
                 image::Rgb([66, 30, 15]),
                 image::Rgb([25, 7, 26]),
                 image::Rgb([9, 1, 47]),
                 image::Rgb([4, 4, 73]),
                 image::Rgb([0, 7, 100]),
                 image::Rgb([12, 44, 138]),
                 image::Rgb([24, 82, 177]),
                 image::Rgb([57, 125, 209]),
                 image::Rgb([134, 181, 229]),
                 image::Rgb([211, 236, 248]),
                 image::Rgb([241, 233, 191]),
                 image::Rgb([248, 201, 95]),
                 image::Rgb([255, 170, 0]),
                 image::Rgb([204, 128, 0]),
                 image::Rgb([153, 87, 0]),
                 image::Rgb([106, 52, 3]),
             ];
             return vec[i as usize];
         }
         else {
             return image::Rgb([0, 0, 0]);
         }
    }

}




fn color_interpolation(c1: image::Rgb<u8>, c2: image::Rgb<u8>, ratio: u64) -> image::Rgb<u8> {
    let r = 0;
    let c1r = c1[0] as f32;      let c2r = c2[0] as f32; 
    let c1g = c1[1] as f32;      let c2g = c2[1] as f32; 
    let c1b = c1[2] as f32;      let c2b = c2[2] as f32; 

    let frac = ratio as f32;

    let r = ((c2r - c1r) * frac + c1r).floor();
    let g = ((c2g - c1g) * frac + c1g).floor();
    let b = ((c2b - c1b) * frac + c1b).floor();
    
    return image::Rgb([r as u8, g as u8, b as u8]);
}












    //    RGB TO HSV TO RGB      CHANING HUE OH HSV

    // fn test_color(&self, i: u64, r: f64, c: f64) -> prism_Rgb<f64> {
    //     let mut di: f64 = i as f64;
    //     let mut zn: f64;
    //     let mut hue: f64;

    //     zn = f64::sqrt(r + c);
    //     hue = di + 1.0 - f64::ln(f64::ln(f64::abs(zn))) / f64::ln(2.0);
    //     hue = 0.95 + 20.0 * hue;

    //     while hue > 360.0 {
    //         println!("2:  {}", hue);
    //         hue -= 360.0;
            
    //     }
    //     while hue < 0.0 {
    //         hue += 360.0;
    //     }

    //     let mut rgb = prism_Rgb::new(0.0, 0.0, 0.0);
    //     // println!("1");
    //     let mut hsv = Hsv::from_color(&rgb);
    //     println!("2:  {}", hue);
    //     hsv.set_hue(Deg(hue));
    //     hsv.set_saturation(0.8);
    //     hsv.set_value(1.0);

    //     rgb = prism_Rgb::from_color(&hsv);
    //     // println!("3");
    //     rgb
    // }





    
    // fn mandelbrot_test(&self) -> image::RgbImage {
    //     let mut img = image::RgbImage::new(self.img_width as u32, self.img_height as u32);

    //     for(xx, yy, pixel) in img.enumerate_pixels_mut() {
    //         let x0 = ((xx as f64) / (self.img_width as f64 / 1.5)) * 3.5 - 3.5;
    //         let y0 = ((yy as f64) / (self.img_height as f64 / 2.0) ) * 1.5 - 1.5;

    //         let c = Complex64::new(x0, y0);
    //         let mut z = Complex64::new(0.0, 0.0);
    //         let mut zx: f64;
    //         let mut zy: f64;
    //         let mut zx2: f64 = 0.0;
    //         let mut zy2: f64 = 0.0;

    //         let mut iter:u64 = 0;
    //         let escape_radius = 100.0;
            
    //         while 0 < self.max_iterations {
    //             z = z.powu(2) + c;
    //             iter += 1;
    //             let modulus = (z.re * z.re) + (z.im * z.im);

    //             // zy = z.re * z.im * 2.0 + c.im;
    //             // zx = zx2 - zy2 + c.re;
    //             // zx2 = zx * zx;
    //             // zy2 = zy * zy2;

                

    //             if modulus > escape_radius {
    //                 break
    //             }

    //             if iter > self.max_iterations {
    //                 break
    //             }
    //         }

    //         // if iter == self.max_iterations {
    //         //     *pixel = image::Rgb([0, 0, 0]);
    //         // }
    //         // else {
               
    //         //     let color = self.test_color(iter, z.re, z.im);
    //         //     *pixel = image::Rgb([color.red() as u8, color.green() as u8, color.blue() as u8]);
    //         // }

    //         let nsmooth = iter + 1- f64::ln(f64::ln(z.abs())) as u64;
    //         // println!("{}", nsmooth);
    //         let color = self.get_color_smooth(nsmooth);

    //         *pixel = color;
    //     }

    //     img
        
    // }
    