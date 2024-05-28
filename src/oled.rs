use stm32f4xx_hal as hal;

use {
    embedded_graphics::{
        mono_font::{iso_8859_16::FONT_10X20 as FONT, MonoTextStyleBuilder, MonoTextStyle},
        pixelcolor::BinaryColor,
        prelude::*,
        image::{Image, ImageRaw},
        text::{Baseline, Text},
    },
    hal::{
        prelude::*,
        i2c::{Mode, I2c},
        pac::I2C2,
        gpio
    },
    ufmt::uwrite,

    ssd1306::{mode::BufferedGraphicsMode, prelude::*, Ssd1306}
};

pub type Display = Ssd1306<I2CInterface<I2c<I2C2>>, DisplaySize128x32, BufferedGraphicsMode<DisplaySize128x32>>;

const TEXT_STYLE: MonoTextStyle<BinaryColor> = MonoTextStyleBuilder::new()
    .font(&FONT)
    .text_color(BinaryColor::On)
    .build();

#[allow(dead_code)]
pub struct OLED {
    display: Display,
    prev_layer: usize,
    prev_uru: bool,
}

#[allow(dead_code)]
impl OLED {
    pub fn new(
        scl: gpio::gpiob::PB10<gpio::Alternate<4, gpio::OpenDrain>>,
        sda: gpio::gpiob::PB3<gpio::Alternate<9, hal::gpio::OpenDrain>>,
        i2c2: I2C2,
        clocks: &hal::rcc::Clocks
    ) -> Self {
        let display = Ssd1306::new(
            ssd1306::I2CDisplayInterface::new(
                I2c::new(
                    i2c2,
                    (scl, sda),
                    Mode::fast(
                        400.kHz(),
                        hal::i2c::DutyCycle::Ratio2to1
                    ),
                    &clocks
                )
            ),
            DisplaySize128x32,
            DisplayRotation::Rotate90,
        ).into_buffered_graphics_mode();

        Self {
            display,
            prev_layer: 0,
            prev_uru: false,
        }
    }

    pub fn draw(&mut self, curr_layer: usize, uru: bool) {
        if 
            curr_layer != self.prev_layer ||
            uru != self.prev_uru
        {
            self.prev_layer = curr_layer;
            self.prev_uru = uru;

            let display = &mut self.display;
            let mut txt: heapless::String<32> = heapless::String::new();


            {
                let img: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("../img.raw"), 32);
                Image::new(&img, Point::new(0, 0)).draw(display).unwrap();
            }

            {
                let _ = uwrite!(&mut txt, "L {}", curr_layer);

                Text::with_baseline(
                    &txt,
                    Point::new(2, 55),
                    TEXT_STYLE,
                    Baseline::Top
                ).draw(display).unwrap();
            }

            {
                txt.clear(); 
                let _ = uwrite!(&mut txt, "{}", 
                    if uru { "-->" } else { "<--" }
                );

                Text::with_baseline(&txt,
                    Point::new(2, 100),
                    TEXT_STYLE,
                    Baseline::Top
                ).draw(display).unwrap();
            }

            display.flush().unwrap();
        }
    }
}
