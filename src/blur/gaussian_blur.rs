use std::{
    cmp::{max, min},
    f64::consts::PI,
};

use crate::util::image::Image;

pub fn get_gaussian_kernel(size: usize, sigma: f64) -> Vec<f64> {
    let mut kernel = vec![0.0; size];
    let mut sum = 0.0;
    let mid = size as i32 / 2;

    for i in -mid..=mid {
        let val = f64::exp(-(i * i) as f64 / (2.0 * sigma * sigma) / (PI * 2.0 * sigma * sigma));
        kernel[(i + mid) as usize] = val;
        sum += val;
    }

    for v in &mut kernel {
        *v /= sum;
    }

    kernel
}

impl Image {
    pub fn horizontal_convolution(self, kernel: &Vec<f64>) -> Image {
        let height = self.height as i32;
        let width = self.width as i32;
        let pixels = self.pixels;
        let mut output_pixels = vec![0u8; (height * width * 3) as usize];
        let mid = kernel.len() as i32 / 2;

        for i in 0..height as i32 {
            for j in 0..width as i32 {
                let mut sum_red = 0.0;
                let mut sum_green = 0.0;
                let mut sum_blue = 0.0;
                for k in -mid..=mid {
                    let px = min(max(j + k, 0), width - 1);
                    sum_blue +=
                        pixels[(3 * (i * width + px)) as usize] as f64 * kernel[(k + mid) as usize];
                    sum_green += pixels[(3 * (i * width + px) + 1) as usize] as f64
                        * kernel[(k + mid) as usize];
                    sum_red += pixels[(3 * (i * width + px) + 2) as usize] as f64
                        * kernel[(k + mid) as usize];
                }

                let pixel_index = i * width + j;
                output_pixels[(3 * pixel_index) as usize] = sum_blue.max(0.0).min(255.0) as u8;
                output_pixels[(3 * pixel_index + 1) as usize] = sum_green.max(0.0).min(255.0) as u8;
                output_pixels[(3 * pixel_index + 2) as usize] = sum_red.max(0.0).min(255.0) as u8;
            }
        }

        Image {
            width: self.width,
            height: self.height,
            pixels: output_pixels,
            format: self.format.clone(),
        }
    }

    pub fn vertical_convolution(self, kernel: &Vec<f64>) -> Image {
        let height = self.height as i32;
        let width = self.width as i32;
        let pixels = self.pixels;
        let mut output_pixels = vec![0u8; (height * width * 3) as usize];
        let mid = kernel.len() as i32 / 2;

        for i in 0..height as i32 {
            for j in 0..width as i32 {
                let mut sum_red = 0.0;
                let mut sum_green = 0.0;
                let mut sum_blue = 0.0;
                for k in -mid..=mid {
                    let py = min(max(i + k, 0), height - 1);
                    sum_blue +=
                        pixels[(3 * (py * width + j)) as usize] as f64 * kernel[(k + mid) as usize];
                    sum_green += pixels[(3 * (py * width + j) + 1) as usize] as f64
                        * kernel[(k + mid) as usize];
                    sum_red += pixels[(3 * (py * width + j) + 2) as usize] as f64
                        * kernel[(k + mid) as usize];
                }

                let pixel_index = i * width + j;
                output_pixels[(3 * pixel_index) as usize] = sum_blue.max(0.0).min(255.0) as u8;
                output_pixels[(3 * pixel_index + 1) as usize] = sum_green.max(0.0).min(255.0) as u8;
                output_pixels[(3 * pixel_index + 2) as usize] = sum_red.max(0.0).min(255.0) as u8;
            }
        }

        Image {
            width: self.width,
            height: self.height,
            pixels: output_pixels,
            format: self.format.clone(),
        }
    }

    pub fn gaussian_blur(self, kernel_size: usize, sigma: f64) -> Image {
        let kernel = get_gaussian_kernel(kernel_size, sigma);
        self.horizontal_convolution(&kernel)
            .vertical_convolution(&kernel)
    }
}
