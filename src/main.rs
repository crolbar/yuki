#![no_std]
#![no_main]

use panic_halt as _;
use stm32f4xx_hal as hal;
mod layout;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [TIM1_CC])]
mod app {
    use super::*;

    #[cfg(feature = "right")]
    use {
        embedded_graphics::{
            mono_font::{iso_8859_16::FONT_10X20 as FONT, MonoTextStyleBuilder},
            pixelcolor::BinaryColor,
            prelude::*,
            text::{Baseline, Text},
        },

        hal::i2c::Mode,

        ufmt::uwrite,
    };

    use ssd1306::{mode::BufferedGraphicsMode, prelude::*, Ssd1306};

    use {
        hal::{
            gpio::{self, EPin, Input},
            otg_fs::{UsbBus, UsbBusType, USB},
            prelude::*,
            i2c::I2c,
            pac::I2C2,
            serial
        },

        nb::block,

        usb_device::prelude::*,

        keyberon::{
            debounce::Debouncer,
            key_code::KbHidReport,
            layout::{Event, Layout},
            matrix::DirectPinMatrix,
        },

        crate::layout::LAYERS,
    };

    pub struct Leds { caps_lock:  gpio::gpioc::PC13<gpio::Output<gpio::PushPull>> }

    impl keyberon::keyboard::Leds for Leds {
        fn caps_lock(&mut self, status: bool) {
            match status {
                true => self.caps_lock.set_low(),
                false => self.caps_lock.set_high(),
            }
        }
    }

    type Display = Ssd1306<I2CInterface<I2c<I2C2>>, DisplaySize128x32, BufferedGraphicsMode<DisplaySize128x32>>;

    #[shared]
    struct Shared {
        usb_dev: UsbDevice<'static, UsbBusType>,
        usb_class: keyberon::Class<'static, UsbBusType, Leds>,
        #[lock_free]
        layout: Layout<12, 4, 4, core::convert::Infallible>,
        use_right_usb: bool,
        #[cfg(feature = "right")]
        #[lock_free]
        oled_refresh: bool,
    }

    #[local]
    struct Local {
        matrix: DirectPinMatrix<EPin<Input>, 6, 4>,
        debouncer: Debouncer<[[bool; 6]; 4]>,
        timer: hal::timer::counter::CounterHz<hal::pac::TIM2>,
        transform: fn(Event) -> Event,
        tx: serial::Tx<hal::pac::USART1>,
        rx: serial::Rx<hal::pac::USART1>,
        buf: [u8; 4],
        #[cfg(feature = "right")]
        display: Display,
        #[cfg(feature = "right")]
        prev_layer: usize,
    }


    #[init(
        local = [
            bus: Option<usb_device::bus::UsbBusAllocator<UsbBusType>> = None,
            ep_memory: [u32; 1024] = [0; 1024]
        ]
    )]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
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


        let usb = USB::new(
            (
                ctx.device.OTG_FS_GLOBAL,
                ctx.device.OTG_FS_DEVICE,
                ctx.device.OTG_FS_PWRCLK,
            ), 
            (gpioa.pa11, gpioa.pa12), &clocks
        );


        let mut caps_lock = gpioc.pc13.into_push_pull_output();
        caps_lock.set_high();

        *ctx.local.bus = Some(
            UsbBus::new(usb, ctx.local.ep_memory)
        );
        let usb_bus = ctx.local.bus.as_ref().unwrap();

        let usb_class = keyberon::new_class(usb_bus, Leds {caps_lock});

        let usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x16c0, 0x27db))
        .strings(&[StringDescriptors::default()
            .manufacturer("crolbar")
            .product("YUKI")
            .serial_number("1337")])
        .unwrap()
        .build();


        let matrix = cortex_m::interrupt::free(move |_| {
            DirectPinMatrix::new([
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
            ]).unwrap()
        });

        #[cfg(feature = "right")]
        let display: Display = init_display(
            gpiob.pb10.into_alternate().set_open_drain(),
            gpiob.pb3.into_alternate().set_open_drain(),
            ctx.device.I2C2, &clocks
        );

        let serial_pins = cortex_m::interrupt::free(move |_| {
            (gpiob.pb6.into_alternate::<7>(), gpiob.pb7.into_alternate::<7>())
        });

        let mut serial = serial::Serial::new(ctx.device.USART1, serial_pins, 38_400.bps(), &mut clocks).unwrap().with_u8_data();

        serial.listen(serial::Event::RxNotEmpty);
        let (tx, rx) = serial.split();

        let transform: fn(Event) -> Event = match cfg!(feature = "right") {
            true => |e| e.transform(|i, j| (i, 11 - j)),
            false => |e| e
        };

        (
            Shared {
                usb_dev,
                usb_class,
                layout: Layout::new(&LAYERS),
                use_right_usb: true,
                #[cfg(feature = "right")]
                oled_refresh: false,
            },
            Local {
                matrix,
                timer,
                debouncer: Debouncer::new([[false; 6]; 4], [[false; 6]; 4], 5),
                transform,
                buf: [0; 4],
                tx, rx,

                #[cfg(feature = "right")]
                display,
                #[cfg(feature = "right")]
                prev_layer: 0,
            },
            init::Monotonics(),
       )
    }

    #[task(priority = 1, capacity = 8, shared = [layout])]
    fn handle_event(c: handle_event::Context, event: Event) {
        c.shared.layout.event(event)
    }

    #[task(binds = USART1, priority = 2, local = [rx, buf], shared = [use_right_usb])]
    fn rx(mut ctx: rx::Context) {
        if let Ok(b) = ctx.local.rx.read() {
            ctx.local.buf.rotate_left(1);
            ctx.local.buf[3] = b;

            if ctx.local.buf[3] == b'\n' {
                if let Ok(event) = deserialize(&ctx.local.buf[..]) {
                    if event == Event::Press(3, 9) {
                        ctx.shared.use_right_usb.lock(|right_usb|{
                            *right_usb = !*right_usb;
                        })
                    }

                    handle_event::spawn(event).unwrap();
                }
            }
        }
    }

    #[task(
        binds=TIM2,
        priority=1,
        local=[debouncer, matrix, timer, transform, tx],
        shared=[usb_dev, usb_class, layout, use_right_usb, oled_refresh]
    )]
    fn tick(mut ctx: tick::Context) {
        ctx.local.timer.wait().ok();

        let use_right_usb = ctx.shared.use_right_usb.lock(|u| *u);

        let mtx = ctx.local.matrix.get().unwrap();

        // if the two buttons and the reset button on the board are held
        // and then the reset button is released this will load dfu
        let dbnc = ctx.local.debouncer.get();
        if (mtx[0][5] && !dbnc[0][5]) && (mtx[3][5] && !dbnc[3][5]) {
            unsafe { cortex_m::asm::bootload(0x1FFF0000 as _) }
        }

        for event in ctx
            .local
            .debouncer
            .events(mtx)
            .map(ctx.local.transform)
        {
            #[cfg(feature = "right")]
            {
                if event == Event::Press(3, 9) {
                    ctx.shared.use_right_usb.lock(|uru|{ *uru = !*uru });

                    *ctx.shared.oled_refresh = true;
                }
            }

            handle_event::spawn(event).unwrap();
            for &b in &serialize(event) {
                block!(ctx.local.tx.write(b)).unwrap();
            }
        }
        ctx.shared.layout.tick();

        if ctx.shared.usb_dev.lock(|d| d.state()) == UsbDeviceState::Configured {
            if 
                (cfg!(feature = "right") && use_right_usb) ||
                (cfg!(not(feature = "right")) && !use_right_usb)
            {
                write_kb_rep::spawn().unwrap();
            }
        }

        #[cfg(feature = "right")] { display_shit::spawn().unwrap() }
    }

    #[cfg(feature = "right")]
    #[task(priority = 1, capacity = 8, local = [display, prev_layer], shared = [layout, use_right_usb, oled_refresh])]
    fn display_shit(mut ctx: display_shit::Context) {
        let curr_layer = ctx.shared.layout.current_layer();

        if &curr_layer != ctx.local.prev_layer {
            *ctx.shared.oled_refresh = true;
            *ctx.local.prev_layer = curr_layer;
        }

        if *ctx.shared.oled_refresh {
            let display = ctx.local.display;

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
            ctx.shared.use_right_usb.lock(|uru| {
                let _ = uwrite!(&mut txt, "{}", 
                    if *uru { "-->" } else { "<--" }
                );
            });

            Text::with_baseline(&txt, Point::new(2, 5), text_style, Baseline::Top)
                .draw(display)
                .unwrap();

            display.flush().unwrap();

            *ctx.shared.oled_refresh = false;
        }
    }

    #[task(priority = 1, capacity = 8, shared = [layout, usb_dev, usb_class])]
    fn write_kb_rep(mut ctx: write_kb_rep::Context) {
        let report: KbHidReport = ctx.shared.layout.keycodes().collect();
        if ctx.shared.usb_class.lock(|k| k.device_mut().set_keyboard_report(report.clone())) {
            while let Ok(0) = ctx.shared.usb_class.lock(|k| k.write(report.as_bytes())) {}
        }
    }

    fn deserialize(bytes: &[u8]) -> Result<Event, ()> {
        match *bytes {
            [b'P', i, j, b'\n'] => Ok(Event::Press(i, j)),
            [b'R', i, j, b'\n'] => Ok(Event::Release(i, j)),
            _ => Err(()),
        }
    }

    fn serialize(e: Event) -> [u8; 4] {
        match e {
            Event::Press(i, j) => [b'P', i, j, b'\n'],
            Event::Release(i, j) => [b'R', i, j, b'\n'],
        }
    }

    use usb_device::class::UsbClass;

    #[task(binds = OTG_FS, priority = 3, shared = [usb_dev, usb_class])]
    fn usb_tx(cx: usb_tx::Context) {
        (cx.shared.usb_dev, cx.shared.usb_class)
        .lock(|usb_dev, kb| {
            if usb_dev.poll(&mut [kb]) { kb.poll() }
        });
    }

    #[task(binds = OTG_FS_WKUP, priority = 3, shared = [usb_dev, usb_class])]
    fn usb_rx(cx: usb_rx::Context) {
        (cx.shared.usb_dev, cx.shared.usb_class)
        .lock(|usb_dev, kb| {
            if usb_dev.poll(&mut [kb]) { kb.poll() }
        });
    }

    #[cfg(feature = "right")]
    fn init_display(
        scl: gpio::gpiob::PB10<gpio::Alternate<4, gpio::OpenDrain>>,
        sda: gpio::gpiob::PB3<gpio::Alternate<9, hal::gpio::OpenDrain>>,
        i2c2: I2C2,
        clocks: &hal::rcc::Clocks
    ) -> Display {

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

        display
    }
}
