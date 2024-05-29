use {
    crate::hal::otg_fs::UsbBusType,

    usbd_human_interface_device::device::mouse::{WheelMouse, WheelMouseReport, WheelMouseConfig},
    usbd_human_interface_device::prelude::*,
    frunk::HList,
};

pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub enum MAction {
    ToggleActive,
    Left,
    Right,
    Middle,

    Move(Dir),
    ToggleSpeedup,

    Scroll(Dir),
}

type MouseDev = UsbHidClass<'static, UsbBusType, HList!(WheelMouse<'static, UsbBusType>)>;

pub struct Mouse {
    pub mouse: MouseDev,
    pub report: WheelMouseReport,
    pub active: bool,
    mouse_speedup: bool,
    move_btn_press_vals: [i8; 4],
}

impl Mouse {
    pub fn new(bus: &'static usb_device::bus::UsbBusAllocator<UsbBusType>) -> Self {
        Self {
            mouse: UsbHidClassBuilder::new().add_device(WheelMouseConfig::default()).build(bus),
            report: WheelMouseReport::default(),
            active: true,
            mouse_speedup: false,
            move_btn_press_vals: [0; 4],
        }
    }

    pub fn handle_mouse_btn(&mut self, action: &MAction, is_pressed: bool) {
        if is_pressed {
            match action {
                MAction::Left => self.report.buttons |= 0x1,
                MAction::Right => self.report.buttons |= 0x2,
                MAction::Middle => self.report.buttons |= 0x4,

                MAction::Move(Dir::Up) => {
                    self.move_btn_press_vals[0] = 4 + self.mouse_speedup as i8 * 6;
                    self.report.y = self.report.y.saturating_sub(self.move_btn_press_vals[0])
                },
                MAction::Move(Dir::Down) => {
                    self.move_btn_press_vals[1] = 4 + self.mouse_speedup as i8 * 6;
                    self.report.y = self.report.y.saturating_add(self.move_btn_press_vals[1])
                },
                MAction::Move(Dir::Left) => {
                    self.move_btn_press_vals[2] = 4 + self.mouse_speedup as i8 * 6;
                    self.report.x = self.report.x.saturating_sub(self.move_btn_press_vals[2])
                },
                MAction::Move(Dir::Right) => {
                    self.move_btn_press_vals[3] = 4 + self.mouse_speedup as i8 * 6;
                    self.report.x = self.report.x.saturating_add(self.move_btn_press_vals[3])
                },

                MAction::Scroll(Dir::Up) => self.report.vertical_wheel = 1,
                MAction::Scroll(Dir::Down) => self.report.vertical_wheel = -1,
                MAction::Scroll(Dir::Left) => self.report.horizontal_wheel = self.report.horizontal_wheel.saturating_sub(1),
                MAction::Scroll(Dir::Right) => self.report.horizontal_wheel = self.report.horizontal_wheel.saturating_add(1),

                MAction::ToggleActive => self.active = !self.active,
                MAction::ToggleSpeedup => self.mouse_speedup = !self.mouse_speedup,
            }
        } else {
            match action {
                MAction::Left => self.report.buttons &= 0xFF - 0x1,
                MAction::Right => self.report.buttons &= 0xFF - 0x2,
                MAction::Middle => self.report.buttons &= 0xFF- 0x4,

                MAction::Move(Dir::Up) => self.report.y = self.report.y.saturating_add(self.move_btn_press_vals[0]),
                MAction::Move(Dir::Down) => self.report.y = self.report.y.saturating_sub(self.move_btn_press_vals[1]),
                MAction::Move(Dir::Left) => self.report.x = self.report.x.saturating_add(self.move_btn_press_vals[2]),
                MAction::Move(Dir::Right) => self.report.x = self.report.x.saturating_sub(self.move_btn_press_vals[3]),

                MAction::Scroll(Dir::Up) => self.report.vertical_wheel = 0,
                MAction::Scroll(Dir::Down) => self.report.vertical_wheel = 0,
                MAction::Scroll(Dir::Left) => self.report.horizontal_wheel = self.report.horizontal_wheel.saturating_add(1),
                MAction::Scroll(Dir::Right) => self.report.horizontal_wheel = self.report.horizontal_wheel.saturating_sub(1),

                _ => (),
            }
        }
    }
}
