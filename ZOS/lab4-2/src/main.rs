use iced::text_input::{self, TextInput};
use iced::{
    button, Button, Column, Container, Element, Image, Length, Row, Sandbox, Settings, Text,
};
use image::io::Reader as ImageReader;
use image::*;

pub fn main() -> iced::Result {
    Kernels::run(Settings::default())
}

type Kernel = [[f32; 5]; 5];

#[derive(Default)]
struct Kernels {
    button: button::State,
    text: [[text_input::State; 5]; 5],

    idiot: usize,
    input: Kernel,
    image_path: String,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
    InputChanged(usize, usize, String),
}

impl Sandbox for Kernels {
    type Message = Message;

    fn new() -> Self {
        let mut me = Self::default();
        me.image_path = "./examples/result.png".to_string();
        me
    }

    fn title(&self) -> String {
        String::from("Kernels")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                let img = ImageReader::open("./examples/cln1.png")
                    .unwrap()
                    .decode()
                    .unwrap();
                let new_path = format!("./examples/result{}.png", self.idiot);

                let rimg = convolute(img, self.input);
                rimg.save(&new_path).unwrap();

                self.image_path = new_path;
                self.idiot += 1;
            }
            Message::InputChanged(i, j, value) => {
                if let Ok(v) = value.parse::<f32>() {
                    self.input[i][j] = v;
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut canvass = Row::new();

        let img = Container::new(Image::new(&self.image_path).width(Length::Units(300))).center_x();
        canvass = canvass.push(img);

        let mut blanket = Column::new();
        for i in 0..5 {
            let mut row = Row::new();
            for j in 0..5 {
                row = row.push(TextInput::new(
                    unsafe {
                        let row =
                            &mut *(self.text.get_unchecked_mut(i) as *mut [text_input::State]);
                        let cell = &mut *(row.get_unchecked_mut(j) as *mut _);
                        cell
                    },
                    "",
                    &self.input[i][j].to_string(),
                    move |s| Message::InputChanged(i, j, s),
                ))
            }
            blanket = blanket.push(row);
        }
        blanket = blanket.push(
            Button::new(&mut self.button, Text::new("Draw!")).on_press(Message::ButtonPressed),
        );
        canvass = canvass.push(blanket);

        Container::new(canvass).into()
    }
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
