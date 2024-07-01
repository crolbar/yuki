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
    scroll_dir: i8,
    scroll_ticks: u8,
    scroll_is_vert: bool,
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
            scroll_dir: 0,
            scroll_ticks: 0,
            scroll_is_vert: false,
        }
    }

    pub fn mouse_tick(&mut self) {
        if self.scroll_dir != 0 {

            self.scroll_ticks += 1;

            if self.scroll_ticks == 10 {
                if self.scroll_is_vert {
                    self.report.vertical_wheel = 0;
                } else {
                    self.report.horizontal_wheel = 0;
                }
            }

            if self.scroll_ticks == 100 {
                if self.scroll_is_vert {
                    self.report.vertical_wheel = self.scroll_dir;
                } else {
                    self.report.horizontal_wheel = self.scroll_dir;
                }

                self.scroll_ticks = 0;
            }
        } else {
            self.report.vertical_wheel = 0;
            self.report.horizontal_wheel = 0;
            self.scroll_ticks = 0;
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

                MAction::Scroll(Dir::Up) => {
                    self.scroll_dir = 1;
                    self.scroll_is_vert = true;
                    self.report.vertical_wheel = self.scroll_dir;
                },
                MAction::Scroll(Dir::Down) => {
                    self.scroll_dir = -1;
                    self.scroll_is_vert = true;
                    self.report.vertical_wheel = self.scroll_dir;
                },
                MAction::Scroll(Dir::Left) => {
                    self.scroll_dir = -1;
                    self.scroll_is_vert = false;
                    self.report.horizontal_wheel = self.scroll_dir;
                },
                MAction::Scroll(Dir::Right) => {
                    self.scroll_dir = 1;
                    self.scroll_is_vert = false;
                    self.report.horizontal_wheel = self.scroll_dir;
                },

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

                MAction::Scroll(Dir::Up) |
                MAction::Scroll(Dir::Down) | 
                MAction::Scroll(Dir::Left) |
                MAction::Scroll(Dir::Right) => {
                    self.scroll_dir = 0;
                    self.scroll_ticks = 0;
                    self.report.vertical_wheel = 0;
                    self.report.horizontal_wheel = 0;
                },

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
