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
    Left,
    Right,
    Middle,

    Move(Dir),

    Scroll(Dir),
}

type MouseDev = UsbHidClass<'static, UsbBusType, HList!(WheelMouse<'static, UsbBusType>)>;

pub struct Mouse {
    pub mouse: MouseDev,
    pub report: WheelMouseReport,
}

impl Mouse {
    pub fn new(bus: &'static usb_device::bus::UsbBusAllocator<UsbBusType>) -> Self {
        Self {
            mouse: UsbHidClassBuilder::new().add_device(WheelMouseConfig::default()).build(bus),
            report: WheelMouseReport::default()
        }
    }

    pub fn handle_mouse_btn(&mut self, action: &MAction, is_pressed: bool) {
        if is_pressed {
            match action {
                MAction::Left => self.report.buttons |= 0x1,
                MAction::Right => self.report.buttons |= 0x2,
                MAction::Middle => self.report.buttons |= 0x4,

                MAction::Move(Dir::Up) => self.report.y = self.report.y.saturating_sub(5),
                MAction::Move(Dir::Down) => self.report.y = self.report.y.saturating_add(5),
                MAction::Move(Dir::Left) => self.report.x = self.report.x.saturating_sub(5),
                MAction::Move(Dir::Right) => self.report.x = self.report.x.saturating_add(5),

                MAction::Scroll(Dir::Up) => self.report.vertical_wheel = 1,
                MAction::Scroll(Dir::Down) => self.report.vertical_wheel = -1,
                MAction::Scroll(Dir::Left) => self.report.horizontal_wheel = self.report.horizontal_wheel.saturating_sub(1),
                MAction::Scroll(Dir::Right) => self.report.horizontal_wheel = self.report.horizontal_wheel.saturating_add(1),
            }
        } else {
            match action {
                MAction::Left => self.report.buttons &= 0xFF - 0x1,
                MAction::Right => self.report.buttons &= 0xFF - 0x2,
                MAction::Middle => self.report.buttons &= 0xFF- 0x4,

                MAction::Move(Dir::Up) => self.report.y = self.report.y.saturating_add(5),
                MAction::Move(Dir::Down) => self.report.y = self.report.y.saturating_sub(5),
                MAction::Move(Dir::Left) => self.report.x = self.report.x.saturating_add(5),
                MAction::Move(Dir::Right) => self.report.x = self.report.x.saturating_sub(5),

                MAction::Scroll(Dir::Up) => self.report.vertical_wheel = 0,
                MAction::Scroll(Dir::Down) => self.report.vertical_wheel = 0,
                MAction::Scroll(Dir::Left) => self.report.horizontal_wheel = self.report.horizontal_wheel.saturating_add(1),
                MAction::Scroll(Dir::Right) => self.report.horizontal_wheel = self.report.horizontal_wheel.saturating_sub(1),
            }
        }
    }
}
