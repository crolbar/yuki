#![no_std]
#![no_main]

use panic_halt as _;
use stm32f4xx_hal as hal;
mod layout;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [TIM1_CC])]
mod app {
    use super::*;

    use embedded_graphics::{
        mono_font::{ascii::FONT_6X10, iso_8859_3::FONT_10X20, MonoTextStyleBuilder},
        pixelcolor::BinaryColor,
        prelude::*,
        text::{Baseline, Text},
    };
    use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

    use hal::{
        gpio::{EPin, Input, Output, PC13}, i2c::{self, DutyCycle, I2c, Mode}, otg_fs::{UsbBus, UsbBusType, USB}, pac::I2C2, prelude::*, timer::fugit::Rate
    };

    use usb_device::prelude::*;

    use keyberon::debounce::Debouncer;
    use keyberon::key_code::KbHidReport;
    use keyberon::layout::{Event, Layout};
    use keyberon::matrix::DirectPinMatrix;

    use crate::layout::LAYERS;

    pub struct Leds {}
    impl keyberon::keyboard::Leds for Leds {}


    #[shared]
    struct Shared {
        usb_dev: UsbDevice<'static, UsbBusType>,
        usb_class: keyberon::Class<'static, UsbBusType, Leds>,
    }

    #[local]
    struct Local {
        layout: Layout<12, 4, 1, core::convert::Infallible>,
        matrix: DirectPinMatrix<EPin<Input>, 6, 4>,
        debouncer: Debouncer<[[bool; 6]; 4]>,
        timer: hal::timer::counter::CounterHz<hal::pac::TIM2>,
        led: PC13<Output>,
    }


    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        static mut EP_MEMORY: [u32; 1024] = [0; 1024];
        static mut USB_BUS: Option<usb_device::bus::UsbBusAllocator<UsbBusType>> = None;

        let mut clocks = ctx
            .device
            .RCC
            .constrain()
            .cfgr
            .use_hse(25.MHz())
            .sysclk(84.MHz())
            .require_pll48clk()
            .freeze();

        let gpioa = ctx.device.GPIOA.split();
        let gpiob = ctx.device.GPIOB.split();
        let gpioc = ctx.device.GPIOC.split();


        let mut timer = ctx.device.TIM2.counter_hz(&mut clocks);
        timer.start(1.kHz()).unwrap();
        timer.listen(hal::timer::Event::Update);


        let mut led = gpioc.pc13.into_push_pull_output();
        led.set_low();


        let usb = USB {
            usb_global: ctx.device.OTG_FS_GLOBAL,
            usb_device: ctx.device.OTG_FS_DEVICE,
            usb_pwrclk: ctx.device.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into(),
            pin_dp: gpioa.pa12.into_alternate().into(),
            hclk: clocks.hclk(),
        };



        unsafe {
            USB_BUS.replace(UsbBus::new(usb, &mut EP_MEMORY));
        }

        let usb_class = keyberon::new_class(
            unsafe {USB_BUS.as_ref().unwrap()},
            Leds{}
        );

        let usb_dev = UsbDeviceBuilder::new(
            unsafe { USB_BUS.as_ref().unwrap() },
            UsbVidPid(0x16c0, 0x27dd),
        )
        .strings(&[StringDescriptors::default()
            .manufacturer("Fake Company")
            .product("Product")
            .serial_number("TEST")])
        .unwrap()
        .build();


        let matrix_pins = [
            [
                Some(gpioa.pa9.into_pull_up_input().erase()),
                Some(gpioa.pa8.into_pull_up_input().erase()),
                Some(gpiob.pb15.into_pull_up_input().erase()),
                Some(gpiob.pb14.into_pull_up_input().erase()),
                Some(gpiob.pb13.into_pull_up_input().erase()),
                Some(gpiob.pb12.into_pull_up_input().erase()),
            ],
            [
                Some(gpiob.pb9.into_pull_up_input().erase()),
                Some(gpiob.pb8.into_pull_up_input().erase()),
                Some(gpiob.pb5.into_pull_up_input().erase()),
                Some(gpiob.pb4.into_pull_up_input().erase()),
                Some(gpioa.pa15.into_pull_up_input().erase()),
                Some(gpioa.pa10.into_pull_up_input().erase()),
            ],
            [
                Some(gpiob.pb1.into_pull_up_input().erase()),
                Some(gpiob.pb0.into_pull_up_input().erase()),
                Some(gpioa.pa7.into_pull_up_input().erase()),
                Some(gpioa.pa6.into_pull_up_input().erase()),
                Some(gpioa.pa5.into_pull_up_input().erase()),
                Some(gpioa.pa4.into_pull_up_input().erase()),
            ],
            [
                None,
                None,
                Some(gpioa.pa3.into_pull_up_input().erase()),
                Some(gpioa.pa2.into_pull_up_input().erase()),
                Some(gpioa.pa1.into_pull_up_input().erase()),
                Some(gpioa.pa0.into_pull_up_input().erase()),
            ],
        ];
        let matrix = cortex_m::interrupt::free(move |_cs| DirectPinMatrix::new(matrix_pins));

        let layout = Layout::new(&LAYERS);


        #[cfg(feature = "right")]
        {
            let scl = gpiob.pb10.into_alternate().set_open_drain();
            let sda = gpiob.pb3.into_alternate().set_open_drain();

            use stm32f4xx_hal::time::Hertz;
            let i2c_frequency: Hertz = 400_u32.kHz();

            let i2c = I2c::new(
                ctx.device.I2C2,
                (scl, sda),
                Mode::Standard {
                    frequency: i2c_frequency,
                },
                &clocks,
                );
            let interface = ssd1306::I2CDisplayInterface::new(i2c);
            let mut display = Ssd1306::new(
                interface,
                DisplaySize128x32,
                DisplayRotation::Rotate0,
                ).into_buffered_graphics_mode();
            display.init().unwrap();

            let text_style = MonoTextStyleBuilder::new()
                .font(&FONT_10X20)
                .text_color(BinaryColor::On)
                .build();

            Text::with_baseline("yo", Point::new(10, 10), text_style, Baseline::Top)
                .draw(&mut display)
                .unwrap();

            display.flush().unwrap();
        }



        (
            Shared {
                usb_dev,
                usb_class,
            },
            Local {
                matrix: matrix.unwrap(),
                timer,
                debouncer: Debouncer::new([[false; 6]; 4], [[false; 6]; 4], 5),
                layout,
                led
            },
            init::Monotonics(),
        )
    }

    
    #[cfg(feature = "right")]
    fn transform_keypress_coordinates(e: Event) -> Event {
        e.transform(|i, j| (i, 11 - j))
    }

    #[cfg(not(feature = "right"))]
    fn transform_keypress_coordinates(e: Event) -> Event {
        e
    }

    #[task(binds=TIM2, priority=1, local=[debouncer, matrix, timer, layout], shared=[usb_dev, usb_class])]
    fn tick(mut ctx: tick::Context) {
        ctx.local.timer.wait().ok();

        for event in ctx
            .local
            .debouncer
            .events(ctx.local.matrix.get().unwrap())
            .map(transform_keypress_coordinates)
        {
            ctx.local.layout.event(event)
        }

        ctx.local.layout.tick();

        let report: KbHidReport = ctx.local.layout.keycodes().collect();
        if ctx.shared.usb_class.lock(|k| k.device_mut().set_keyboard_report(report.clone())) {
            while let Ok(0) = ctx.shared.usb_class.lock(|k| k.write(report.as_bytes())) {}
        }
    }

    use usb_device::class::UsbClass;

    #[task(binds = OTG_FS, priority = 3, shared = [usb_dev, usb_class])]
    fn usb_tx(cx: usb_tx::Context) {
        (cx.shared.usb_dev, cx.shared.usb_class).lock(|mut usb_dev, mut usb_class| {
            usb_poll(&mut usb_dev, &mut usb_class);
        });
    }

    #[task(binds = OTG_FS_WKUP, priority = 3, shared = [usb_dev, usb_class])]
    fn usb_rx(cx: usb_rx::Context) {
        (cx.shared.usb_dev, cx.shared.usb_class).lock(|mut usb_dev, mut usb_class| {
            usb_poll(&mut usb_dev, &mut usb_class);
        });
    }

    fn usb_poll(usb_dev: &mut UsbDevice<'static, UsbBusType>, keyboard: &mut keyberon::Class<'static, UsbBusType, Leds>) {
        if usb_dev.poll(&mut [keyboard]) {
            keyboard.poll();
        }
    }
}
