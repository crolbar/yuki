use stm32f4xx_hal as hal;

use {
    embedded_graphics::{
        mono_font::{iso_8859_16::FONT_10X20 as FONT, MonoTextStyleBuilder},
        pixelcolor::BinaryColor,
        prelude::*,
        text::{Baseline, Text},
    },
    hal::{
        prelude::*,
        i2c::{Mode, I2c},
        pac::I2C2,
        gpio
    },
    ufmt::uwrite,
};

use ssd1306::{mode::BufferedGraphicsMode, prelude::*, Ssd1306};

pub type Display = Ssd1306<I2CInterface<I2c<I2C2>>, DisplaySize128x32, BufferedGraphicsMode<DisplaySize128x32>>;

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
        let mut display = Ssd1306::new(
            ssd1306::I2CDisplayInterface::new(
                I2c::new(
                    i2c2, (scl, sda),
                    Mode::Standard { frequency: 400_u32.kHz() }, 
                    &clocks
                )
            ),
            DisplaySize128x32,
            DisplayRotation::Rotate90,
        ).into_buffered_graphics_mode();

        {
            display.init().unwrap();

            let text_style = MonoTextStyleBuilder::new()
                .font(&FONT)
                .text_color(BinaryColor::On)
                .build();

            Text::with_baseline("yo", Point::new(10, 10), text_style, Baseline::Top)
                .draw(&mut display)
                .unwrap();

            display.flush().unwrap();
        }

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

            display.init().unwrap();

            let text_style = MonoTextStyleBuilder::new()
                .font(&FONT)
                .text_color(BinaryColor::On)
                .build();

            let mut txt: heapless::String<32> = heapless::String::new();
            let _ = uwrite!(&mut txt, "L {}", curr_layer);

            Text::with_baseline(&txt, Point::new(2, 50), text_style, Baseline::Top)
                .draw(display)
                .unwrap();

            txt = heapless::String::new();
                let _ = uwrite!(&mut txt, "{}", 
                    if uru { "-->" } else { "<--" }
                );

            Text::with_baseline(&txt, Point::new(2, 5), text_style, Baseline::Top)
                .draw(display)
                .unwrap();

            display.flush().unwrap();

        }
    }
}
