use std::{cmp::min, collections::VecDeque, f64::consts::PI};

use crate::util::image::Image;

pub struct Kernel {
    pub kx: &'static [i32],
    pub ky: &'static [i32],
    pub size: usize,
}

pub const SOBEL: Kernel = Kernel {
    kx: &[-1, 0, 1, -2, 0, 2, -1, 0, 1],
    ky: &[-1, -2, -1, 0, 0, 0, 1, 2, 1],
    size: 3,
};

pub const PREWITT: Kernel = Kernel {
    kx: &[1, 1, 1, 0, 0, 0, -1, -1, -1],
    ky: &[1, 0, -1, 1, 0, -1, 1, 0, -1],
    size: 3,
};

pub const ROBERTS: Kernel = Kernel {
    kx: &[1, 0, 0, -1],
    ky: &[0, 1, -1, 0],
    size: 2,
};

pub enum EdgeKernel {
    Sobel,
    Prewitt,
    Roberts,
}

impl EdgeKernel {
    pub fn kernel(&self) -> &'static Kernel {
        match self {
            EdgeKernel::Sobel => &SOBEL,
            EdgeKernel::Prewitt => &PREWITT,
            EdgeKernel::Roberts => &ROBERTS,
        }
    }
}

impl Image {
    pub fn non_maximum_suppression(self, kernel: EdgeKernel) -> Image {
        let height = self.height as i32;
        let width = self.width as i32;
        let pixels = self.pixels;
        let kernel = kernel.kernel();
        let k = kernel.size as i32;
        let kx = kernel.kx;
        let ky = kernel.ky;
        let radius = k as i32 / 2;
        let mut magnitudes = vec![0.0; (height * width) as usize];
        let mut angles = vec![0.0; (height * width) as usize];
        let mut output_pixels = vec![0u8; (width * height * 3) as usize];

        for y in radius..height - radius {
            for x in radius..width - radius {
                let mut gx = 0;
                let mut gy = 0;
                for dy in 0..k {
                    for dx in 0..k {
                        let y0 = y + dy - radius;
                        let x0 = x + dx - radius;
                        let gray = pixels[((y0 * width + x0) * 3) as usize] as i32;
                        let w = dy * k + dx;
                        gx += kx[w as usize] * gray;
                        gy += ky[w as usize] * gray;
                    }
                }
                let idx = (y * width + x) as usize;
                magnitudes[idx] = ((gx * gx + gy * gy) as f64).sqrt();
                let angle = (gy as f64).atan2(gx.into()) * (180.0 / PI);
                angles[idx] = if angle < 0.0 { angle + 180.0 } else { angle };
            }
        }

        for y in radius..height - radius {
            for x in radius..width - radius {
                let idx = (y * width + x) as usize;
                let mag = magnitudes[idx];
                let angle = angles[idx];
                let sector = if angle <= 22.5 || angle > 157.5 {
                    0
                } else if angle <= 67.5 {
                    1
                } else if angle <= 112.5 {
                    2
                } else {
                    3
                };
                let (m1, m2) = match sector {
                    0 => (magnitudes[idx - 1], magnitudes[idx + 1]),
                    1 => (
                        magnitudes[idx - width as usize + 1],
                        magnitudes[idx + width as usize - 1],
                    ),
                    2 => (
                        magnitudes[idx - width as usize],
                        magnitudes[idx + width as usize],
                    ),
                    _ => (
                        magnitudes[idx - width as usize - 1],
                        magnitudes[idx + width as usize + 1],
                    ),
                };

                let pixel = if mag >= m1 && mag >= m2 {
                    min(255, mag as u8)
                } else {
                    0
                };
                output_pixels[idx * 3] = pixel;
                output_pixels[idx * 3 + 1] = pixel;
                output_pixels[idx * 3 + 2] = pixel;
            }
        }

        Image {
            width: self.width,
            height: self.height,
            pixels: output_pixels,
            format: self.format.clone(),
        }
    }

    pub fn hysteresis_thresholding(self, lower: u8, upper: u8) -> Image {
        let height = self.height;
        let width = self.width;
        let pixels = self.pixels;
        let mut visited = vec![false; (width * height) as usize];
        let mut bfs: VecDeque<(u32, u32)> = VecDeque::new();
        let mut output_pixels = vec![0u8; (width * height * 3) as usize];

        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize;
                let val = pixels[3 * idx];
                if val >= upper {
                    visited[idx] = true;
                    output_pixels[3 * idx] = 255;
                    output_pixels[3 * idx + 1] = 255;
                    output_pixels[3 * idx + 2] = 255;
                    bfs.push_back((x, y));
                }
            }
        }

        const DX: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
        const DY: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
        while let Some((x, y)) = bfs.pop_front() {
            for k in 0..8 {
                let nx = x as i32 + DX[k];
                let ny = y as i32 + DY[k];
                if nx < 0 || nx >= width as i32 || ny < 0 || ny >= height as i32 {
                    continue;
                }
                let n_idx = (ny * width as i32 + nx) as usize;
                if visited[n_idx] {
                    continue;
                }

                let n_val = pixels[3 * n_idx];
                if n_val >= lower {
                    visited[n_idx] = true;
                    output_pixels[3 * n_idx] = 255;
                    output_pixels[3 * n_idx + 1] = 255;
                    output_pixels[3 * n_idx + 2] = 255;
                    bfs.push_back((nx as u32, ny as u32));
                }
            }
        }

        Image {
            width: self.width,
            height: self.height,
            pixels: output_pixels,
            format: self.format.clone(),
        }
    }

    pub fn canny(self, size: usize, sigma: f64, kernel: EdgeKernel, lower: u8, upper: u8) -> Image {
        self.gaussian_blur(size, sigma)
            .non_maximum_suppression(kernel)
            .hysteresis_thresholding(lower, upper)
    }
}
