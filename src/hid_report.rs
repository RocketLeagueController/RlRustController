//!HID joystick
use crate::usb_class::prelude::*;
use core::default::Default;
use usb_device::bus::UsbBus;
use usb_device::class_prelude::UsbBusAllocator;
use fugit::ExtU32;
use packed_struct::prelude::*;

// from https://github.com/nefarius/ViGEmBus/issues/40
// see https://github.com/dlkj/usbd-human-interface-device/blob/main/src/device/joystick.rs
// see https://github.com/TimDeve/sunrs/blob/0d335dffa3ebe8909a0308639af4bb66dccee902/src/main.rs
// see https://usb.org/sites/default/files/hut1_2.pdf
// see http://who-t.blogspot.com/2018/12/understanding-hid-report-descriptors.html

#[rustfmt::skip]
pub const XBOX_JOYSTICK_DESCRIPTOR: &[u8] = &[
    0x05, 0x01, // Usage Page (Generic Desktop)         5,   1
    0x09, 0x04, // Usage (Joystick)                     9,   4
    0xa1, 0x01, // Collection (Application)             161, 1
    0x09, 0x01, //   Usage Page (Pointer)               9,   1
    0xa1, 0x00, //   Collection (Physical)              161, 0
    0x09, 0x30, //     Usage (X)                        9,   48
    0x09, 0x31, //     Usage (Y)                        9,   49
    0x09, 0x32, //     Usage (Z)                        9,   50
    0x09, 0x33, //     Usage (Rx)                       9,   51
    0x09, 0x34, //     Usage (Ry)                       9,   52
    0x09, 0x35, //     Usage (Rz)                       9,   53
    0x15, 0x81, //     Logical Minimum (-127)           21,  129
    0x25, 0x7f, //     Logical Maximum (127)            37,  127
    0x75, 0x08, //     Report Size (8)                  117, 8
    0x95, 0x06, //     Report count (6)                 149, 6,
    0x81, 0x02, //     Input (Data, Variable, Absolute) 129, 2,
    0xc0,       //   End Collection                     192,
    0x05, 0x09, //   Usage Page (Button)                5,   9,
    0x19, 0x01, //   Usage Minimum (0)                  25,  1,
    0x29, 0x10, //   Usage Maximum (8)                  41,  8,
    0x15, 0x00, //   Logical Minimum (0)                21,  0
    0x25, 0x01, //   Logical Maximum (1)                37,  1,
    0x75, 0x01, //   Report Size (1)                    117, 1,
    0x95, 0x10, //   Report Count (16)                  149, 8
    0x81, 0x02, //   Input (Data, Variable, Absolute)   129, 2,
    0xc0,       // End Collection                       192
];

#[derive(Clone, Copy, Debug, Default, PackedStruct)]
#[packed_struct(endian = "lsb", size_bytes="8")]
pub struct XboxJoystickReport {
    #[packed_field]
    pub x: i8,
    #[packed_field]
    pub y: i8,
    #[packed_field]
    pub z: i8,
    #[packed_field]
    pub rx: i8,
    #[packed_field]
    pub ry: i8,
    #[packed_field]
    pub rz: i8,
    #[packed_field]
    pub buttons: u16,
}

pub struct XboxJoystick<'a, B: UsbBus> {
    interface: Interface<'a, B, InBytes8, OutNone, ReportSingle>,
}

impl<'a, B: UsbBus> XboxJoystick<'a, B> {
    pub fn write_report(&mut self, report: &XboxJoystickReport) -> Result<(), UsbHidError> {
        let data = report.pack();
        self.interface
            .write_report(&data)
            .map(|_| ())
            .map_err(UsbHidError::from)
    }
}

impl<'a, B: UsbBus> DeviceClass<'a> for XboxJoystick<'a, B> {
    type I = Interface<'a, B, InBytes8, OutNone, ReportSingle>;

    fn interface(&mut self) -> &mut Self::I {
        &mut self.interface
    }

    fn reset(&mut self) {}

    fn tick(&mut self) -> Result<(), UsbHidError> {
        Ok(())
    }
}

pub struct XboxJoystickConfig<'a> {
    interface: InterfaceConfig<'a, InBytes8, OutNone, ReportSingle>,
}

impl<'a> Default for XboxJoystickConfig<'a> {
    #[must_use]
    fn default() -> Self {
        Self::new(
            ((InterfaceBuilder::new(XBOX_JOYSTICK_DESCRIPTOR)).unwrap()
                .boot_device(InterfaceProtocol::None)
                .description("Joystick")
                .in_endpoint(10.millis())).unwrap()
            .without_out_endpoint()
            .build(),
        )
    }
}

impl<'a> XboxJoystickConfig<'a> {
    #[must_use]
    pub fn new(interface: InterfaceConfig<'a, InBytes8, OutNone, ReportSingle>) -> Self {
        Self { interface }
    }
}

impl<'a, B: UsbBus + 'a> UsbAllocatable<'a, B> for XboxJoystickConfig<'a> {
    type Allocated = XboxJoystick<'a, B>;

    fn allocate(self, usb_alloc: &'a UsbBusAllocator<B>) -> Self::Allocated {
        Self::Allocated {
            interface: Interface::new(usb_alloc, self.interface),
        }
    }
}
