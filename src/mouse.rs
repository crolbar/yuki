use {
    crate::hal::otg_fs::UsbBusType,

    usbd_human_interface_device::device::mouse::{WheelMouse, WheelMouseReport, WheelMouseConfig},
    usbd_human_interface_device::prelude::*,
    frunk::HList,
};

pub enum MAction {
    LeftMB,
    RightMB,
    MiddleMB,

    Up,
    Down,
    Left,
    Right,

    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight
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
                MAction::LeftMB => self.report.buttons |= 0x1,
                MAction::RightMB => self.report.buttons |= 0x2,
                MAction::MiddleMB => self.report.buttons |= 0x4,

                MAction::Up => self.report.y = self.report.y.saturating_sub(5),
                MAction::Down => self.report.y = self.report.y.saturating_add(5),
                MAction::Left => self.report.x = self.report.x.saturating_sub(5),
                MAction::Right => self.report.x = self.report.x.saturating_add(5),

                MAction::ScrollUp => self.report.vertical_wheel = 1,
                MAction::ScrollDown => self.report.vertical_wheel = -1,
                MAction::ScrollLeft => self.report.horizontal_wheel = self.report.horizontal_wheel.saturating_sub(1),
                MAction::ScrollRight => self.report.horizontal_wheel = self.report.horizontal_wheel.saturating_add(1),
            }
        } else {
            match action {
                MAction::LeftMB => self.report.buttons &= 0xFF - 0x1,
                MAction::RightMB => self.report.buttons &= 0xFF - 0x2,
                MAction::MiddleMB => self.report.buttons &= 0xFF- 0x4,

                MAction::Up => self.report.y = self.report.y.saturating_add(5),
                MAction::Down => self.report.y = self.report.y.saturating_sub(5),
                MAction::Left => self.report.x = self.report.x.saturating_add(5),
                MAction::Right => self.report.x = self.report.x.saturating_sub(5),

                MAction::ScrollUp => self.report.vertical_wheel = 0,
                MAction::ScrollDown => self.report.vertical_wheel = 0,
                MAction::ScrollLeft => self.report.horizontal_wheel = self.report.horizontal_wheel.saturating_add(1),
                MAction::ScrollRight => self.report.horizontal_wheel = self.report.horizontal_wheel.saturating_sub(1),
            }
        }
    }
}
