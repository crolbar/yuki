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
    Speedup,

    Scroll(Dir),
}

type MouseDev = UsbHidClass<'static, UsbBusType, HList!(WheelMouse<'static, UsbBusType>)>;

pub struct Mouse {
    pub mouse: MouseDev,
    pub report: WheelMouseReport,
    pub active: bool,
    move_btn_press_vals: [i8; 4],
}


const DEFAULT_SPEED: i8 = 4;
const SPEED_ADD: i8 = 6;

impl Mouse {
    pub fn new(bus: &'static usb_device::bus::UsbBusAllocator<UsbBusType>) -> Self {
        Self {
            mouse: UsbHidClassBuilder::new().add_device(WheelMouseConfig::default()).build(bus),
            report: WheelMouseReport::default(),
            active: true,
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
                    self.move_btn_press_vals[0] += DEFAULT_SPEED;
                    self.report.y = self.report.y.saturating_sub(self.move_btn_press_vals[0])
                },
                MAction::Move(Dir::Down) => {
                    self.move_btn_press_vals[1] += DEFAULT_SPEED;
                    self.report.y = self.report.y.saturating_add(self.move_btn_press_vals[1])
                },
                MAction::Move(Dir::Left) => {
                    self.move_btn_press_vals[2] += DEFAULT_SPEED;
                    self.report.x = self.report.x.saturating_sub(self.move_btn_press_vals[2])
                },
                MAction::Move(Dir::Right) => {
                    self.move_btn_press_vals[3] += DEFAULT_SPEED;
                    self.report.x = self.report.x.saturating_add(self.move_btn_press_vals[3])
                },

                MAction::Scroll(Dir::Up) => self.report.vertical_wheel = 1,
                MAction::Scroll(Dir::Down) => self.report.vertical_wheel = -1,
                MAction::Scroll(Dir::Left) => self.report.horizontal_wheel = self.report.horizontal_wheel.saturating_sub(1),
                MAction::Scroll(Dir::Right) => self.report.horizontal_wheel = self.report.horizontal_wheel.saturating_add(1),

                MAction::Speedup => {
                    self.move_btn_press_vals.iter_mut().enumerate().for_each(|(i, v)| {
                        if *v != 0 {
                            match i {
                                0 => self.report.y = self.report.y.saturating_sub(SPEED_ADD),
                                1 => self.report.y = self.report.y.saturating_add(SPEED_ADD),
                                2 => self.report.x = self.report.x.saturating_sub(SPEED_ADD),
                                3 => self.report.x = self.report.x.saturating_add(SPEED_ADD),
                                _ => ()
                            }
                        }

                        *v += SPEED_ADD;
                    });
                },

                MAction::ToggleActive => self.active = !self.active,
            }
        } else {
            match action {
                MAction::Left => self.report.buttons &= 0xFF - 0x1,
                MAction::Right => self.report.buttons &= 0xFF - 0x2,
                MAction::Middle => self.report.buttons &= 0xFF- 0x4,

                MAction::Move(Dir::Up) => {
                    self.report.y = self.report.y.saturating_add(self.move_btn_press_vals[0]);
                    self.move_btn_press_vals[0] -= DEFAULT_SPEED;
                },
                MAction::Move(Dir::Down) => {
                    self.report.y = self.report.y.saturating_sub(self.move_btn_press_vals[1]);
                    self.move_btn_press_vals[1] -= DEFAULT_SPEED;
                },
                MAction::Move(Dir::Left) => {
                    self.report.x = self.report.x.saturating_add(self.move_btn_press_vals[2]);
                    self.move_btn_press_vals[2] -= DEFAULT_SPEED;
                },
                MAction::Move(Dir::Right) => {
                    self.report.x = self.report.x.saturating_sub(self.move_btn_press_vals[3]);
                    self.move_btn_press_vals[3] -= DEFAULT_SPEED;
                },

                MAction::Scroll(Dir::Up) => self.report.vertical_wheel = 0,
                MAction::Scroll(Dir::Down) => self.report.vertical_wheel = 0,
                MAction::Scroll(Dir::Left) => self.report.horizontal_wheel = self.report.horizontal_wheel.saturating_add(1),
                MAction::Scroll(Dir::Right) => self.report.horizontal_wheel = self.report.horizontal_wheel.saturating_sub(1),

                MAction::Speedup => {
                    self.move_btn_press_vals.iter_mut().enumerate().for_each(|(i, v)| {
                        *v -= SPEED_ADD;

                        if *v != 0 {
                            match i {
                                0 => self.report.y = self.report.y.saturating_add(SPEED_ADD),
                                1 => self.report.y = self.report.y.saturating_sub(SPEED_ADD),
                                2 => self.report.x = self.report.x.saturating_add(SPEED_ADD),
                                3 => self.report.x = self.report.x.saturating_sub(SPEED_ADD),
                                _ => ()
                            }
                        }
                    })
                } ,

                _ => (),
            }
        }
    }
}
