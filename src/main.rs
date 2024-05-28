#![no_std]
#![no_main]

use panic_halt as _;
use stm32f4xx_hal as hal;
mod layout;
mod oled;
mod mouse;

#[rtic::app(device = hal::pac, dispatchers = [TIM1_CC])]
mod app {
    use {
        super::*,

        hal::{
            gpio::{EPin, Input, PC13, Output, PushPull},
            otg_fs::{UsbBus, UsbBusType, USB},
            pac::{USART1, TIM2},
            timer::counter::CounterHz,
            prelude::*,
            serial
        },
        cortex_m::interrupt,

        nb::block,

        usb_device::prelude::*,

        keyberon::{
            debounce::Debouncer,
            key_code::KbHidReport,
            layout::{Event, Layout, CustomEvent},
            matrix::DirectPinMatrix,
            keyboard
        },

        layout::{LAYERS, CustomAction},
        oled::OLED,
        mouse::Mouse,
    };

    pub struct Leds { caps_lock:  PC13<Output<PushPull>> }

    impl keyboard::Leds for Leds {
        fn caps_lock(&mut self, status: bool) {
            match status {
                true => self.caps_lock.set_low(),
                false => self.caps_lock.set_high(),
            }
        }
    }

    #[shared]
    struct Shared {
        usb_dev: UsbDevice<'static, UsbBusType>,
        usb_class: keyberon::Class<'static, UsbBusType, Leds>,
        layout: Layout<12, 4, 5, CustomAction>,
        mouse: Mouse
    }

    #[local]
    struct Local {
        matrix: DirectPinMatrix<EPin<Input>, 6, 4>,
        debouncer: Debouncer<[[bool; 6]; 4]>,
        timer: CounterHz<TIM2>,
        tx: serial::Tx<USART1>,
        rx: serial::Rx<USART1>,
        enter_dfu: bool,
        use_right_usb: bool,
        #[cfg(feature = "right")]
        oled: OLED,
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

        let pa0 = gpioa.pa0.into_pull_up_input();
        let pb12 = gpiob.pb12.into_pull_up_input();
        let enter_dfu = pa0.is_low() && pb12.is_low();

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

        *ctx.local.bus = Some(UsbBus::new(usb, ctx.local.ep_memory));
        let usb_bus = ctx.local.bus.as_ref().unwrap();

        let usb_class = keyberon::new_class(usb_bus, Leds {caps_lock});

        let mouse = Mouse::new(usb_bus);

        let usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x16c0, 0x27db))
        .strings(&[StringDescriptors::default()
            .manufacturer("crolbar")
            .product("YUKI")
            .serial_number("1337")])
        .unwrap()
        .build();


        let matrix = interrupt::free(move |_| {
            DirectPinMatrix::new([
                [
                    Some(gpioa.pa9.into_pull_up_input().erase()),
                    Some(gpioa.pa8.into_pull_up_input().erase()),
                    Some(gpiob.pb15.into_pull_up_input().erase()),
                    Some(gpiob.pb14.into_pull_up_input().erase()),
                    Some(gpiob.pb13.into_pull_up_input().erase()),
                    Some(pb12.into_pull_up_input().erase()),
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
                    Some(pa0.into_pull_up_input().erase()),
                ],
            ]).unwrap()
        });

        let mut serial = serial::Serial::new(
            ctx.device.USART1,
            interrupt::free(move |_| 
                (gpiob.pb6.into_alternate::<7>(), gpiob.pb7.into_alternate::<7>())
            ),
            38_400.bps(),
            &mut clocks
        ).unwrap().with_u8_data();

        serial.listen(serial::Event::RxNotEmpty);
        let (tx, rx) = serial.split();

        (
            Shared {
                usb_dev,
                usb_class,
                layout: Layout::new(&LAYERS),
                mouse,
            },
            Local {
                matrix,
                timer,
                debouncer: Debouncer::new([[false; 6]; 4], [[false; 6]; 4], 5),
                tx, rx,
                enter_dfu,
                use_right_usb: true,
                #[cfg(feature = "right")]
                oled: OLED::new(
                    gpiob.pb10.into_alternate().set_open_drain(),
                    gpiob.pb3.into_alternate().set_open_drain(),
                    ctx.device.I2C2, &clocks
                )
            },
            init::Monotonics(),
       )
    }

    #[task(priority = 3, capacity = 8, shared = [layout])]
    fn handle_event(mut ctx: handle_event::Context, event: Event) {
        ctx.shared.layout.lock(|l| l.event(event))
    }

    #[task(binds = USART1, priority = 2, local = [rx])]
    fn rx(ctx: rx::Context) {
        if let Ok(b) = ctx.local.rx.read() {
            if let Ok(event) = de(b) {
                #[cfg(not(feature = "right"))]
                let event = event.transform(|i, j| (i, 11 - j));

                handle_event::spawn(event).unwrap();
            }
        }
    }

    #[task(
        binds=TIM2,
        priority=1,
        local=[debouncer, matrix, timer, tx, oled, use_right_usb],
        shared=[usb_dev, usb_class, layout, mouse]
    )]
    fn tick(mut ctx: tick::Context) {
        ctx.local.timer.wait().ok();

        ctx.local
            .debouncer
            .events(ctx.local.matrix.get().unwrap())
            .for_each(|e| {
                block!(ctx.local.tx.write(ser(e))).unwrap();

                #[cfg(feature = "right")]
                let e = e.transform(|i, j| (i, 11 - j));

                handle_event::spawn(e).unwrap();
            });

        match ctx.shared.layout.lock(|l| l.tick()) {
            CustomEvent::NoEvent => (),
            CustomEvent::Press(CustomAction::USB) => *ctx.local.use_right_usb = !*ctx.local.use_right_usb,
            CustomEvent::Press(CustomAction::M(maction)) => ctx.shared.mouse.lock(|m| m.handle_mouse_btn(maction, true)),
            CustomEvent::Release(CustomAction::M(maction)) => ctx.shared.mouse.lock(|m| m.handle_mouse_btn(maction, false)),
            _ => ()
        }

        let use_right_usb = ctx.local.use_right_usb;

        if ctx.shared.usb_dev.lock(|d| d.state()) == UsbDeviceState::Configured {
            #[cfg(feature = "right")] let cond = *use_right_usb;
            #[cfg(not(feature = "right"))] let cond = !*use_right_usb;

            if cond {
                let report: KbHidReport = ctx.shared.layout.lock(|l| l.keycodes().collect());
                if ctx.shared.usb_class.lock(|k| k.device_mut().set_keyboard_report(report.clone())) {
                    while let Ok(0) = ctx.shared.usb_class.lock(|k| k.write(report.as_bytes())) {}
                }

                while let Ok(()) = ctx.shared.mouse.lock(|m| m.mouse.device().write_report(&m.report)) {}
            }
        }

        #[cfg(feature = "right")] 
        ctx.local.oled.draw(
            ctx.shared.layout.lock(|l| l.current_layer()),
            *use_right_usb
        )
    }

    #[idle(local = [enter_dfu])]
    fn idle(ctx: idle::Context) -> ! {
        // if pa0 && pb12 are shorted and the board resets this will load dfu
        if *ctx.local.enter_dfu {
            unsafe { cortex_m::asm::bootload(0x1FFF0000 as _) }
        }

        loop { rtic::export::wfi() }
    }

    // im using one byte for the coords and the is_press status
    // the last three bits are the y
    // the second to last three are the x
    // the first bit is the is_press status
    //                                     press     5       2
    // so for example Event::Press(2, 5) == (1) 0 (1 0 1) (0 1 0)
    //
    // this only works with 8 or less columns or rows
    fn ser(e: Event) -> u8 {
        let (y, x) = e.coord();
        y | x << 3 | (e.is_press() as u8) << 7
    }
    fn de(n: u8) -> Result<Event, ()> {
        match n & 128 {
            128 => Ok(Event::Press(n & 7, (n & 56) >> 3)),
            0 => Ok(Event::Release(n & 7, (n & 56) >> 3)),
            _ => Err(())
        } 
    }

    use usb_device::class::UsbClass;
    #[task(binds = OTG_FS, priority = 3, shared = [usb_dev, usb_class, mouse])]
    fn usb(ctx: usb::Context) {
        (ctx.shared.usb_dev, ctx.shared.usb_class, ctx.shared.mouse)
        .lock(|usb_dev, kb, m| {
            if m.active {
                if usb_dev.poll(&mut [&mut m.mouse, kb]) { kb.poll() }
            } else {
                if usb_dev.poll(&mut [kb]) { kb.poll() }
            }
        });
    }
}
