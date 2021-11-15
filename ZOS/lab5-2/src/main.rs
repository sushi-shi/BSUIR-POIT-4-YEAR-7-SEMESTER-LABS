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
                let img_f = ImageReader::open("./examples/f.JPG")
                    .unwrap()
                    .decode()
                    .unwrap()
                    .grayscale()
                    .as_luma8()
                    .unwrap()
                    .clone();

                let img_g = ImageReader::open("./examples/g.JPG")
                    .unwrap()
                    .decode()
                    .unwrap()
                    .grayscale()
                    .as_luma8()
                    .unwrap()
                    .clone();

                let rimg = convolute(img_f, img_g);
                rimg.save("./examples/r.png").unwrap();
                println!("COMOL");
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
        blanket = blanket.push(
            Button::new(&mut self.button, Text::new("Draw!")).on_press(Message::ButtonPressed),
        );
        let img_c =
            Container::new(Image::new("./examples/r.png").width(Length::Units(300))).center_x();
        blanket = blanket.push(img_c);

        canvass = canvass.push(blanket);

        Container::new(canvass).into()
    }
}

fn convolute(img_f: GrayImage, img_g: GrayImage) -> GrayImage {
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
                    let v = px[0] as f32;

                    let fv = if fi < fwidth || fj < fheight {
                        let px = img_f.get_pixel(fi, fj).0;
                        px[0] as f32
                    } else {
                        0.
                    };
                    res += v * fv / (255. * 255.);
                }
            }
            max = f32::max(max, res);
            min = f32::min(min, res);
            print!("{}", res);
            img_res[x as usize][y as usize] = res;
        }
    }
    println!("{}", min);
    println!("{}", max);
    for x in 0..fwidth - gwidth {
        for y in 0..fheight - gheight {
            // println!("GOD DIS {}", img_res[x as usize][y as usize]);
            let luma = 255. * (img_res[x as usize][y as usize] - min) / (max - min);
            let luma = luma as u8;
            // println!("LUMED {}", luma);
            img_c.put_pixel(x, y, [luma].into());
        }
    }
    img_c
}

pub fn correlate(signal_f: &[f64], signal_g: &[f64]) -> Vec<f64> {
    let mut res = Vec::new();
    for tau in 0..signal_f.len() {
        let mut rho = 0.;
        for t in 0..signal_g.len() {
            rho += signal_g[t]
                * if t + tau < signal_f.len() {
                    signal_f[t + tau]
                } else {
                    0.
                };
        }
        res.push(rho);
    }
    res
}
