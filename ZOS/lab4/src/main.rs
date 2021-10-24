use image::io::Reader as ImageReader;
use image::*;

use lazy_static::lazy_static;
use std::collections::HashMap;

type Kernel = Vec<Vec<f32>>;

// I don't know the full answer, but for some I can explain. For sharpen & Sobel for example,
// the goal is to see the edge better. So a difference operator (would be an approximation to derivative here)
// would help you see the change in neighboring pixels. So a partial derivative can be across or horizontal,
// so can the difference operator. For example, horizontal difference operator using central difference would be [1 0 -1].
// In the case of Sobel, it differentiates and average, hence [1 2 1]^T * [1 0 -1] where [1 2 1] is an averaging operator
// with more emphasis in the center pixel. A second derivative, approximated with central difference
// would take on 5 star stencil , [ [ 0 1, 0] [1 -4 1] [0 1 0] ]. Similarly for 3x3 mean smoothing,
// you average all your neighbors, so it will be 3x3 matrix of 1's divided by 9.
// These then serve as a motivation for the shape/pattern of the kernel, you can then do something fancier,
// like [ [ 0 1, 0] [1 -5 1] [0 1 0] ] instead of [ [ 0 1, 0] [1 -4 1] [0 1 0] ] to emphasize mid pixel's
// difference to neighbor or a mean smoothing that is less affected by neighbors,
// so has kernel [ [ 0.1 0.1, 0.1] [0.1 1 0.1] [0.1 0.1 0.1] ] / 1.8.
// Others kernel probably has it's own logic and interpretation as well.

lazy_static! {
    #[rustfmt::skip]
    static ref KERNELS: HashMap<String, Kernel> = {
        vec![
            (
                "Id".to_string(),
                vec![
                    vec![0., 0., 0.],
                    vec![0., 1., 0.],
                    vec![0., 0., 0.]
                ],
            ),
            (
                "Blur".to_string(),
                vec![
                    vec![1, 1, 1],
                    vec![1, 1, 1],
                    vec![1, 1, 1],
                ].into_iter().map(|v| v.into_iter().map(|v| v as f32 / 9.).collect()).collect(),
             ),
             (
                "Edge".to_string(),
                vec![
                    vec![-1., -1., -1.],
                    vec![-1.,  8., -1.],
                    vec![-1., -1., -1.],
                ],
             ),
             (
                "Sharpen".to_string(),
                vec![
                    vec![ 0., -1.,  0.],
                    vec![-1.,  5., -1.],
                    vec![ 0., -1.,  0.],
                ],
             ),
             (
                "Gaussian".to_string(),
                vec![
                    vec![1, 2, 1],
                    vec![2, 4, 2],
                    vec![1, 2, 1],
                ].into_iter().map(|v| v.into_iter().map(|v| v as f32 / 16.).collect()).collect(),
             ),
             (
                 "Edge2".to_string(),
                 vec![
                    vec![ 1., 0., -1.],
                    vec![ 0., 0.,  0.],
                    vec![-1., 0.,  1.],
                 ],
             ),
             (
                 "Gaussian2".to_string(),
                 vec![
                    vec![1, 4, 6, 4, 1],
                    vec![4, 16, 24, 16, 4],
                    vec![6, 24, 36, 24, 6],
                    vec![4, 16, 24, 16, 4],
                    vec![1, 4, 6, 4, 1],
                 ].into_iter().map(|v| v.into_iter().map(|v| v as f32 / 256.).collect()).collect(),
             ),
             (
                 "Unsharp".to_string(),
                 vec![
                    vec![1, 4, 6, 4, 1],
                    vec![4, 16, 24, 16, 4],
                    vec![6, 24, -476, 24, 6],
                    vec![4, 16, 24, 16, 4],
                    vec![1, 4, 6, 4, 1],
                 ].into_iter().map(|v| v.into_iter().map(|v| v as f32 / -256.).collect()).collect(),
             ),
             (
                 "Emboss".to_string(),
                 vec![
                    vec![-2., -1., 0.],
                    vec![-1.,  1., 1.],
                    vec![ 0.,  1., 2.],
                 ],
             ),
             (
                 "TopSobel".to_string(),
                 vec![
                    vec![1, 2, 1],
                    vec![0, 0, 0],
                    vec![-1, -2, -1],
                 ].into_iter().map(|v| v.into_iter().map(|v| v as f32).collect()).collect(),
             ),
             (
                 "RightSobel".to_string(),
                 vec![
                    vec![-1, 0, 1],
                    vec![-2, 0, 2],
                    vec![-1, 0, 1],
                 ].into_iter().map(|v| v.into_iter().map(|v| v as f32).collect()).collect(),
             ),
             (
                 "LeftSobel".to_string(),
                 vec![
                    vec![1, 0, -1],
                    vec![2, 0, -2],
                    vec![1, 0, -1],
                 ].into_iter().map(|v| v.into_iter().map(|v| v as f32).collect()).collect(),
             ),
             (
                 "BottomSobel".to_string(),
                 vec![
                    vec![-1, -2, -1],
                    vec![0, 0, 0],
                    vec![1, 2, 1],
                 ].into_iter().map(|v| v.into_iter().map(|v| v as f32).collect()).collect(),
             ),
             (
                 "Concussion".to_string(),
                 vec![
                    vec![1, 0, 0],
                    vec![0, 0, 0],
                    vec![0, 0, 1],
                 ].into_iter().map(|v| v.into_iter().map(|v| v as f32).collect()).collect(),
             ),
             (
                 "Bonkers".to_string(),
                 vec![
                    vec![1, -10, 1],
                    vec![10, 0, -10],
                    vec![1, 10, 1],
                 ].into_iter().map(|v| v.into_iter().map(|v| v as f32).collect()).collect(),
             ),
        ]
        .into_iter()
        .collect()
    };
}

fn fail_with(progname: &str, msg: &str) -> ! {
    eprintln!("Error: {}", msg);
    fail_usage(progname);
}

fn fail_usage(progname: &str) -> ! {
    let kernels = &*KERNELS.iter().map(|x| x.0.clone()).collect::<Vec<String>>();
    eprintln!("USAGE: {} PATH KERNEL\n  KERNEL: {:?}", progname, kernels);
    std::process::exit(1);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();

    let progname = args
        .next()
        .unwrap_or_else(|| fail_with("program-name", "Couldn't get name of the program"));
    let path = args
        .next()
        .unwrap_or_else(|| fail_with(&progname, "No path provided"));
    let kernel = args
        .next()
        .unwrap_or_else(|| fail_with(&progname, "No kernel provided"));

    let kernel = &*KERNELS
        .get(&kernel)
        .unwrap_or_else(|| fail_with(&progname, "No such kernel exists"));

    let img = ImageReader::open(path)?.decode()?;
    let rimg = convolute(img, kernel.to_vec());
    rimg.save("result.jpg")?;

    Ok(())
}

fn convolute(img: DynamicImage, kernel: Kernel) -> RgbImage {
    let (kwidth, kheight) = (kernel.len() as u32, kernel[0].len() as u32);
    let (width, height) = img.dimensions();

    let mut rimg = RgbImage::new(width, height);

    for (x, y, _px) in img.pixels() {
        let (mut rk, mut gk, mut bk, mut k) = (0., 0., 0., 0.);

        for i in 0..kwidth {
            for j in 0..kheight {
                // get surrounding px coordinates
                let ki = (i + x).checked_sub(kwidth / 2);
                let kj = (j + y).checked_sub(kheight / 2);
                let (ki, kj) = match (ki, kj) {
                    (Some(ki), Some(kj)) if ki < width && kj < height => (ki, kj),
                    _ => continue,
                };

                // calculate mask
                let kv = kernel[i as usize][j as usize];
                let px = img.get_pixel(ki, kj).0;
                let (r, g, b) = (px[0] as f32, px[1] as f32, px[2] as f32);
                rk += r * kv;
                gk += g * kv;
                bk += b * kv;

                k += kv;
            }
        }

        let (rk, gk, bk) = (rk / k, gk / k, bk / k);
        #[rustfmt::skip]
        let (r, g, b) = (
            if rk > 255. { 255 } else if rk < 0. { 0 } else { rk as u8 },
            if gk > 255. { 255 } else if gk < 0. { 0 } else { gk as u8 },
            if bk > 255. { 255 } else if bk < 0. { 0 } else { bk as u8 },
        );

        rimg.put_pixel(x, y, Rgb([r, g, b]))
    }
    rimg
}
