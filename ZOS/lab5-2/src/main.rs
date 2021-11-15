use iced::{
    button, Button, Column, Container, Element, Image, Length, Row, Sandbox, Settings, Text,
};
use image::io::Reader as ImageReader;
use image::*;

pub fn main() -> iced::Result {
    CorrelationState::run(Settings::default())
}

#[derive(Default)]
struct CorrelationState {
    button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

impl Sandbox for CorrelationState {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Correlation")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                let img = ImageReader::open("./examples/f.JPG")
                    .unwrap()
                    .decode()
                    .unwrap()
                    .grayscale()
                    .to_luma8()
                    .clone();

                let template = ImageReader::open("./examples/g.JPG")
                    .unwrap()
                    .decode()
                    .unwrap()
                    .grayscale()
                    .to_luma8()
                    .clone();

                let rimg = correlate(img, template);
                rimg.save("./examples/r.png").unwrap();
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut canvass = Row::new();

        let img_f =
            Container::new(Image::new("./examples/f.JPG").width(Length::Units(300))).center_x();
        canvass = canvass.push(img_f);
        let img_g =
            Container::new(Image::new("./examples/g.JPG").width(Length::Units(300))).center_x();
        canvass = canvass.push(img_g);

        let mut blanket = Column::new();
        let img_c =
            Container::new(Image::new("./examples/r.png").width(Length::Units(300))).center_x();
        blanket = blanket.push(img_c);
        blanket = blanket.push(
            Button::new(&mut self.button, Text::new("Draw!")).on_press(Message::ButtonPressed),
        );

        canvass = canvass.push(blanket);

        Container::new(canvass).into()
    }
}

fn convolute(img_f: DynamicImage, img_g: DynamicImage) -> GrayImage {
    let (fwidth, fheight) = img_f.dimensions();
    let (gwidth, gheight) = img_g.dimensions();

    let mut img_c = GrayImage::new(fwidth, fheight);
    let mut img_res = vec![vec![0.; fheight as usize]; fwidth as usize];
    let mut max = 0.;
    let mut min = f32::MAX;
    for x in 0..fwidth - gwidth {
        for y in 0..fheight - gheight {
            let mut res = 0.;

            for i in 0..gwidth {
                for j in 0..gheight {
                    // get surrounding px coordinates
                    let fi = x + i;
                    let fj = y + j;

                    // calculate mask
                    let px = img_g.get_pixel(i, j).0;
                    let (r, g, b) = (px[0] as f32, px[1] as f32, px[2] as f32);

                    let (fr, fg, fb) = if fi < fwidth || fj < fheight {
                        let px = img_f.get_pixel(fi, fj).0;
                        (px[0] as f32, px[1] as f32, px[2] as f32)
                    } else {
                        (0., 0., 0.)
                    };
                    res += r * fr + g * fg + b * fb;
                }
            }
            max = f32::max(max, res);
            min = f32::min(min, res);
            img_res[x as usize][y as usize] = res;
        }
    }
    for x in 0..fwidth - gwidth {
        for y in 0..fheight - gheight {
            let luma = 255. * (img_res[x as usize][y as usize] - min) / (max - min);
            let luma = luma as u8;
            img_c.put_pixel(x, y, [luma].into());
        }
    }
    img_c
}

pub fn mean(img: &GrayImage) -> f32 {
    let (width, height) = img.dimensions();
    let m = (width * height) as f32;
    let mut dr = 0.;

    for px in img.pixels() {
        let r = px.0[0];
        dr += r as f32;
    }
    dr / m
}

pub fn deviation(img: &GrayImage, mean: f32) -> f32 {
    let (width, height) = img.dimensions();
    let m = (width * height) as f32;
    let mut dr = 0.;

    for px in img.pixels() {
        let r = px.0[0] as f32;
        dr += (r - mean).powf(2.);
    }
    (dr / (m - 1.)).sqrt()
}

pub fn correlate(img: GrayImage, template: GrayImage) -> GrayImage {
    let tmean = mean(&template);
    let tdevi = deviation(&template, tmean);

    let (twidth, theight) = template.dimensions();

    let (iwidth, iheight) = img.dimensions();

    let mut rimg = GrayImage::new(iwidth, iheight);

    let mut rvec = vec![vec![0.; iheight as usize]; iwidth as usize];
    let mut max = 0.;
    let mut min = f32::MAX;

    for i in 0..iwidth {
        for j in 0..iheight {
            let itwidth = u32::min(i + twidth, iwidth);
            let itheight = u32::min(j + theight, iheight);
            let itsize = (itwidth * itheight) as f32;

            let itmean = {
                let mut dr = 0.;
                for i in i..itwidth {
                    for j in j..itheight {
                        let px = img.get_pixel(i, j);
                        let r = px.0[0] as f32;
                        dr += r;
                    }
                }
                dr / itsize
            };
            let itdevi = {
                let mut dr = 0.;
                for i in i..itwidth {
                    for j in j..itheight {
                        let px = img.get_pixel(i, j);
                        let r = px.0[0] as f32;
                        dr += (r - itmean).powf(2.);
                    }
                }
                (dr / (itsize - 1.)).sqrt()
            };

            let cross = {
                let mut dr = 0.;
                for (x, i) in (i..itwidth).enumerate() {
                    for (y, j) in (j..itheight).enumerate() {
                        let px = img.get_pixel(i, j);
                        let r = px.0[0] as f32;

                        let tpx = template.get_pixel(x as u32, y as u32);
                        let tr = tpx[0] as f32;

                        dr += (r - itmean) * (tr - tmean);
                    }
                }
                dr / itsize
            };
            let px = cross / (itdevi * tdevi);
            max = f32::max(max, px);
            min = f32::min(min, px);
            rvec[i as usize][j as usize] = px;
        }
    }
    for i in 0..iwidth {
        for j in 0..iheight {
            let px = 255. * (rvec[i as usize][j as usize] - min) / (max - min);
            rimg.put_pixel(i, j, [px as u8].into());
        }
    }
    rimg
}
