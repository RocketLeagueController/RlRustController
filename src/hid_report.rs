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

#[rustfmt::skip]
pub const XBOX_JOYSTICK_DESCRIPTOR: &[u8] = &[
    0x05, 0x01, //        (GLOBAL) USAGE_PAGE         0x0001 Generic Desktop Page 
    0x09, 0x05, //        (LOCAL)  USAGE              0x00010005 Game Pad (Application Collection)  
    0xA1, 0x01, //        (MAIN)   COLLECTION         0x01 Application (Usage=0x00010005: Page=Generic Desktop Page, Usage=Game Pad, Type=Application Collection)
    0xA1, 0x00, //          (MAIN)   COLLECTION         0x00 Physical (Usage=0x0: Page=, Usage=, Type=) <-- Error: COLLECTION must be preceded by a USAGE <-- Warning: USAGE type should be CP (Physical Collection)
    0x09, 0x30, //            (LOCAL)  USAGE              0x00010030 X (Dynamic Value)  
    0x09, 0x31, //            (LOCAL)  USAGE              0x00010031 Y (Dynamic Value)  
    0x15, 0x00, //            (GLOBAL) LOGICAL_MINIMUM    0x00 (0)  <-- Info: Consider replacing 15 00 with 14
    0x27, 0xFF, 0xFF, 0x00, 0x00, //      (GLOBAL) LOGICAL_MAXIMUM    0x0000FFFF (65535)  
    0x95, 0x02, //            (GLOBAL) REPORT_COUNT       0x02 (2) Number of fields  
    0x75, 0x10, //            (GLOBAL) REPORT_SIZE        0x10 (16) Number of bits per field  
    0x81, 0x02, //            (MAIN)   INPUT              0x00000002 (2 fields x 16 bits) 0=Data 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0xC0, //          (MAIN, //)   END_COLLECTION     Physical 
    0xA1, 0x00, //          (MAIN)   COLLECTION         0x00 Physical (Usage=0x0: Page=, Usage=, Type=) <-- Error: COLLECTION must be preceded by a USAGE <-- Warning: USAGE type should be CP (Physical Collection)
    0x09, 0x33, //            (LOCAL)  USAGE              0x00010033 Rx (Dynamic Value)  
    0x09, 0x34, //            (LOCAL)  USAGE              0x00010034 Ry (Dynamic Value)  
    0x15, 0x00, //            (GLOBAL) LOGICAL_MINIMUM    0x00 (0) <-- Redundant: LOGICAL_MINIMUM is already 0 <-- Info: Consider replacing 15 00 with 14
    0x27, 0xFF, 0xFF, 0x00, 0x00, //      (GLOBAL) LOGICAL_MAXIMUM    0x0000FFFF (65535) <-- Redundant: LOGICAL_MAXIMUM is already 65535 
    0x95, 0x02, //            (GLOBAL) REPORT_COUNT       0x02 (2) Number of fields <-- Redundant: REPORT_COUNT is already 2 
    0x75, 0x10, //            (GLOBAL) REPORT_SIZE        0x10 (16) Number of bits per field <-- Redundant: REPORT_SIZE is already 16 
    0x81, 0x02, //            (MAIN)   INPUT              0x00000002 (2 fields x 16 bits) 0=Data 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0xC0, //          (MAIN, //)   END_COLLECTION     Physical 
    0x05, 0x01, //          (GLOBAL) USAGE_PAGE         0x0001 Generic Desktop Page <-- Redundant: USAGE_PAGE is already 0x0001
    0x09, 0x32, //          (LOCAL)  USAGE              0x00010032 Z (Dynamic Value)  
    0x15, 0x00, //          (GLOBAL) LOGICAL_MINIMUM    0x00 (0) <-- Redundant: LOGICAL_MINIMUM is already 0 <-- Info: Consider replacing 15 00 with 14
    0x26, 0xFF, 0x03, //        (GLOBAL) LOGICAL_MAXIMUM    0x03FF (1023)  
    0x95, 0x01, //          (GLOBAL) REPORT_COUNT       0x01 (1) Number of fields  
    0x75, 0x0A, //          (GLOBAL) REPORT_SIZE        0x0A (10) Number of bits per field  
    0x81, 0x02, //          (MAIN)   INPUT              0x00000002 (1 field x 10 bits) 0=Data 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0x15, 0x00, //          (GLOBAL) LOGICAL_MINIMUM    0x00 (0) <-- Redundant: LOGICAL_MINIMUM is already 0 <-- Info: Consider replacing 15 00 with 14
    0x25, 0x00, //          (GLOBAL) LOGICAL_MAXIMUM    0x00 (0)  <-- Info: Consider replacing 25 00 with 24
    0x75, 0x06, //          (GLOBAL) REPORT_SIZE        0x06 (6) Number of bits per field  
    0x95, 0x01, //          (GLOBAL) REPORT_COUNT       0x01 (1) Number of fields <-- Redundant: REPORT_COUNT is already 1 
    0x81, 0x03, //          (MAIN)   INPUT              0x00000003 (1 field x 6 bits) 1=Constant 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0x05, 0x01, //          (GLOBAL) USAGE_PAGE         0x0001 Generic Desktop Page <-- Redundant: USAGE_PAGE is already 0x0001
    0x09, 0x35, //          (LOCAL)  USAGE              0x00010035 Rz (Dynamic Value)  
    0x15, 0x00, //          (GLOBAL) LOGICAL_MINIMUM    0x00 (0) <-- Redundant: LOGICAL_MINIMUM is already 0 <-- Info: Consider replacing 15 00 with 14
    0x26, 0xFF, 0x03, //        (GLOBAL) LOGICAL_MAXIMUM    0x03FF (1023)  
    0x95, 0x01, //          (GLOBAL) REPORT_COUNT       0x01 (1) Number of fields <-- Redundant: REPORT_COUNT is already 1 
    0x75, 0x0A, //          (GLOBAL) REPORT_SIZE        0x0A (10) Number of bits per field  
    0x81, 0x02, //          (MAIN)   INPUT              0x00000002 (1 field x 10 bits) 0=Data 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0x15, 0x00, //          (GLOBAL) LOGICAL_MINIMUM    0x00 (0) <-- Redundant: LOGICAL_MINIMUM is already 0 <-- Info: Consider replacing 15 00 with 14
    0x25, 0x00, //          (GLOBAL) LOGICAL_MAXIMUM    0x00 (0)  <-- Info: Consider replacing 25 00 with 24
    0x75, 0x06, //          (GLOBAL) REPORT_SIZE        0x06 (6) Number of bits per field  
    0x95, 0x01, //          (GLOBAL) REPORT_COUNT       0x01 (1) Number of fields <-- Redundant: REPORT_COUNT is already 1 
    0x81, 0x03, //          (MAIN)   INPUT              0x00000003 (1 field x 6 bits) 1=Constant 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0x05, 0x09, //          (GLOBAL) USAGE_PAGE         0x0009 Button Page 
    0x19, 0x01, //          (LOCAL)  USAGE_MINIMUM      0x00090001 Button 1 Primary/trigger (Selector, On/Off Control, Momentary Control, or One Shot Control)  
    0x29, 0x0A, //          (LOCAL)  USAGE_MAXIMUM      0x0009000A Button 10 (Selector, On/Off Control, Momentary Control, or One Shot Control)  
    0x95, 0x0A, //          (GLOBAL) REPORT_COUNT       0x0A (10) Number of fields  
    0x75, 0x01, //          (GLOBAL) REPORT_SIZE        0x01 (1) Number of bits per field  
    0x81, 0x02, //          (MAIN)   INPUT              0x00000002 (10 fields x 1 bit) 0=Data 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0x15, 0x00, //          (GLOBAL) LOGICAL_MINIMUM    0x00 (0) <-- Redundant: LOGICAL_MINIMUM is already 0 <-- Info: Consider replacing 15 00 with 14
    0x25, 0x00, //          (GLOBAL) LOGICAL_MAXIMUM    0x00 (0) <-- Redundant: LOGICAL_MAXIMUM is already 0 <-- Info: Consider replacing 25 00 with 24
    0x75, 0x06, //          (GLOBAL) REPORT_SIZE        0x06 (6) Number of bits per field  
    0x95, 0x01, //          (GLOBAL) REPORT_COUNT       0x01 (1) Number of fields  
    0x81, 0x03, //          (MAIN)   INPUT              0x00000003 (1 field x 6 bits) 1=Constant 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0x05, 0x01, //          (GLOBAL) USAGE_PAGE         0x0001 Generic Desktop Page 
    0x09, 0x39, //          (LOCAL)  USAGE              0x00010039 Hat switch (Dynamic Value)  
    0x15, 0x01, //          (GLOBAL) LOGICAL_MINIMUM    0x01 (1)  
    0x25, 0x08, //          (GLOBAL) LOGICAL_MAXIMUM    0x08 (8)  
    0x35, 0x00, //          (GLOBAL) PHYSICAL_MINIMUM   0x00 (0)  <-- Info: Consider replacing 35 00 with 34
    0x46, 0x3B, 0x01, //        (GLOBAL) PHYSICAL_MAXIMUM   0x013B (315)  
    0x66, 0x14, 0x00, //        (GLOBAL) UNIT               0x0014 Rotation in degrees [1° units] (4=System=English Rotation, 1=Rotation=Degrees)  <-- Info: Consider replacing 66 1400 with 65 14
    0x75, 0x04, //          (GLOBAL) REPORT_SIZE        0x04 (4) Number of bits per field  
    0x95, 0x01, //          (GLOBAL) REPORT_COUNT       0x01 (1) Number of fields <-- Redundant: REPORT_COUNT is already 1 
    0x81, 0x42, //          (MAIN)   INPUT              0x00000042 (1 field x 4 bits) 0=Data 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 1=Null 0=NonVolatile 0=Bitmap 
    0x75, 0x04, //          (GLOBAL) REPORT_SIZE        0x04 (4) Number of bits per field <-- Redundant: REPORT_SIZE is already 4 
    0x95, 0x01, //          (GLOBAL) REPORT_COUNT       0x01 (1) Number of fields <-- Redundant: REPORT_COUNT is already 1 
    0x15, 0x00, //          (GLOBAL) LOGICAL_MINIMUM    0x00 (0)  <-- Info: Consider replacing 15 00 with 14
    0x25, 0x00, //          (GLOBAL) LOGICAL_MAXIMUM    0x00 (0)  <-- Info: Consider replacing 25 00 with 24
    0x35, 0x00, //          (GLOBAL) PHYSICAL_MINIMUM   0x00 (0) <-- Redundant: PHYSICAL_MINIMUM is already 0 <-- Info: Consider replacing 35 00 with 34
    0x45, 0x00, //          (GLOBAL) PHYSICAL_MAXIMUM   0x00 (0)  <-- Info: Consider replacing 45 00 with 44
    0x65, 0x00, //          (GLOBAL) UNIT               0x00 No unit (0=None)  <-- Info: Consider replacing 65 00 with 64
    0x81, 0x03, //          (MAIN)   INPUT              0x00000003 (1 field x 4 bits) 1=Constant 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0xA1, 0x02, //          (MAIN)   COLLECTION         0x02 Logical (Usage=0x0: Page=, Usage=, Type=) <-- Error: COLLECTION must be preceded by a USAGE <-- Warning: USAGE type should be CL (Logical Collection)
    0x05, 0x0F, //            (GLOBAL) USAGE_PAGE         0x000F Physical Interface Device Page 
    0x09, 0x97, //            (LOCAL)  USAGE              0x000F0097 DC Enable Actuators (Selector)  
    0x15, 0x00, //            (GLOBAL) LOGICAL_MINIMUM    0x00 (0) <-- Redundant: LOGICAL_MINIMUM is already 0 <-- Info: Consider replacing 15 00 with 14
    0x25, 0x01, //            (GLOBAL) LOGICAL_MAXIMUM    0x01 (1)  
    0x75, 0x04, //            (GLOBAL) REPORT_SIZE        0x04 (4) Number of bits per field <-- Redundant: REPORT_SIZE is already 4 
    0x95, 0x01, //            (GLOBAL) REPORT_COUNT       0x01 (1) Number of fields <-- Redundant: REPORT_COUNT is already 1 
    0x91, 0x02, //            (MAIN)   OUTPUT             0x00000002 (1 field x 4 bits) 0=Data 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0x15, 0x00, //            (GLOBAL) LOGICAL_MINIMUM    0x00 (0) <-- Redundant: LOGICAL_MINIMUM is already 0 <-- Info: Consider replacing 15 00 with 14
    0x25, 0x00, //            (GLOBAL) LOGICAL_MAXIMUM    0x00 (0)  <-- Info: Consider replacing 25 00 with 24
    0x91, 0x03, //            (MAIN)   OUTPUT             0x00000003 (1 field x 4 bits) 1=Constant 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0x09, 0x70, //            (LOCAL)  USAGE              0x000F0070 Magnitude (Dynamic Value)  
    0x15, 0x00, //            (GLOBAL) LOGICAL_MINIMUM    0x00 (0) <-- Redundant: LOGICAL_MINIMUM is already 0 <-- Info: Consider replacing 15 00 with 14
    0x25, 0x64, //            (GLOBAL) LOGICAL_MAXIMUM    0x64 (100)  
    0x75, 0x08, //            (GLOBAL) REPORT_SIZE        0x08 (8) Number of bits per field  
    0x95, 0x04, //            (GLOBAL) REPORT_COUNT       0x04 (4) Number of fields  
    0x91, 0x02, //            (MAIN)   OUTPUT             0x00000002 (4 fields x 8 bits) 0=Data 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0x09, 0x50, //            (LOCAL)  USAGE              0x000F0050 Duration (Dynamic Value)  
    0x66, 0x01, 0x10, //          (GLOBAL) UNIT               0x1001 Time in seconds [1 s units] (1=System=SI Linear, 1=Time=Seconds)  
    0x55, 0x0E, //            (GLOBAL) UNIT_EXPONENT      0x0E (Unit Value x 10⁻²)  
    0x26, 0xFF, 0x00, //          (GLOBAL) LOGICAL_MAXIMUM    0x00FF (255)  
    0x95, 0x01, //            (GLOBAL) REPORT_COUNT       0x01 (1) Number of fields  
    0x91, 0x02, //            (MAIN)   OUTPUT             0x00000002 (1 field x 8 bits) 0=Data 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0x09, 0xA7, //            (LOCAL)  USAGE              0x000F00A7 Start Delay (Dynamic Value)  
    0x91, 0x02, //            (MAIN)   OUTPUT             0x00000002 (1 field x 8 bits) 0=Data 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0x65, 0x00, //            (GLOBAL) UNIT               0x00 No unit (0=None)  <-- Info: Consider replacing 65 00 with 64
    0x55, 0x00, //            (GLOBAL) UNIT_EXPONENT      0x00 (Unit Value x 10⁰)  <-- Info: Consider replacing 55 00 with 54
    0x09, 0x7C, //            (LOCAL)  USAGE              0x000F007C Loop Count (Dynamic Value)  
    0x91, 0x02, //            (MAIN)   OUTPUT             0x00000002 (1 field x 8 bits) 0=Data 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0xC0, //           (MAIN, //)   END_COLLECTION     Logical 
    0x05, 0x01, //          (GLOBAL) USAGE_PAGE         0x0001 Generic Desktop Page 
    0x09, 0x80, //          (LOCAL)  USAGE              0x00010080 System Control (Application Collection)  
    0xA1, 0x00, //          (MAIN)   COLLECTION         0x00 Physical (Usage=0x00010080: Page=Generic Desktop Page, Usage=System Control, Type=Application Collection) <-- Warning: USAGE type should be CP (Physical Collection)
    0x09, 0x85, //            (LOCAL)  USAGE              0x00010085 System Main Menu (One Shot Control)  
    0x15, 0x00, //            (GLOBAL) LOGICAL_MINIMUM    0x00 (0) <-- Redundant: LOGICAL_MINIMUM is already 0 <-- Info: Consider replacing 15 00 with 14
    0x25, 0x01, //            (GLOBAL) LOGICAL_MAXIMUM    0x01 (1)  
    0x95, 0x01, //            (GLOBAL) REPORT_COUNT       0x01 (1) Number of fields <-- Redundant: REPORT_COUNT is already 1 
    0x75, 0x01, //            (GLOBAL) REPORT_SIZE        0x01 (1) Number of bits per field  
    0x81, 0x02, //            (MAIN)   INPUT              0x00000002 (1 field x 1 bit) 0=Data 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0x15, 0x00, //            (GLOBAL) LOGICAL_MINIMUM    0x00 (0) <-- Redundant: LOGICAL_MINIMUM is already 0 <-- Info: Consider replacing 15 00 with 14
    0x25, 0x00, //            (GLOBAL) LOGICAL_MAXIMUM    0x00 (0)  <-- Info: Consider replacing 25 00 with 24
    0x75, 0x07, //            (GLOBAL) REPORT_SIZE        0x07 (7) Number of bits per field  
    0x95, 0x01, //            (GLOBAL) REPORT_COUNT       0x01 (1) Number of fields <-- Redundant: REPORT_COUNT is already 1 
    0x81, 0x03, //            (MAIN)   INPUT              0x00000003 (1 field x 7 bits) 1=Constant 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0xC0, //          (MAIN, //)   END_COLLECTION     Physical 
    0x05, 0x06, //          (GLOBAL) USAGE_PAGE         0x0006 Generic Device Controls Page 
    0x09, 0x20, //          (LOCAL)  USAGE              0x00060020 Battery Strength (Dynamic Value)  
    0x15, 0x00, //          (GLOBAL) LOGICAL_MINIMUM    0x00 (0) <-- Redundant: LOGICAL_MINIMUM is already 0 <-- Info: Consider replacing 15 00 with 14
    0x26, 0xFF, 0x00, //        (GLOBAL) LOGICAL_MAXIMUM    0x00FF (255)  
    0x75, 0x08, //          (GLOBAL) REPORT_SIZE        0x08 (8) Number of bits per field  
    0x95, 0x01, //          (GLOBAL) REPORT_COUNT       0x01 (1) Number of fields <-- Redundant: REPORT_COUNT is already 1 
    0x81, 0x02, //          (MAIN)   INPUT              0x00000002 (1 field x 8 bits) 0=Data 1=Variable 0=Absolute 0=NoWrap 0=Linear 0=PrefState 0=NoNull 0=NonVolatile 0=Bitmap 
    0xC0, //        (MAIN, //)   END_COLLECTION     Application 
];

#[derive(Clone, Copy, Debug, Default, PackedStruct)]
#[packed_struct(endian = "lsb", bit_numbering="lsb0", size_bytes="17")]
pub struct XboxJoystickReport {
    #[packed_field(bits="0..=15")]
    pub GD_GamePadX : u16, // : 16,                          // Usage 0x00010030: X, Value = 0 to 65535
    #[packed_field(bits="16..=31")]
    pub GD_GamePadY : u16, // : 16,                          // Usage 0x00010031: Y, Value = 0 to 65535
    #[packed_field(bits="32..=47")]
    pub GD_GamePadRx : u16, // : 16,                         // Usage 0x00010033: Rx, Value = 0 to 65535
    #[packed_field(bits="48..=63")]
    pub GD_GamePadRy : u16, // : 16,                         // Usage 0x00010034: Ry, Value = 0 to 65535

    #[packed_field(bits="64..=79")]
    pub GD_GamePadZ : u16, // : 10,                          // Usage 0x00010032: Z, Value = 0 to 1023
    //pub unknown0 : i8, //: 6,                                // Pad

    #[packed_field(bits="80..=95")]
    pub GD_GamePadRz : u16, // : 10,                         // Usage 0x00010035: Rz, Value = 0 to 1023
    //pub unknown1 : i8, //: 6,                                 // Pad

    #[packed_field(bits="96")]
    pub BTN_GamePadButton1 : Integer<u8, packed_bits::Bits1>, // : 1,                     // Usage 0x00090001: Button 1 Primary/trigger, Value = 0 to 0
    #[packed_field(bits="97")]
    pub BTN_GamePadButton2 : Integer<u8, packed_bits::Bits1>, // : 1,                     // Usage 0x00090002: Button 2 Secondary, Value = 0 to 0
    #[packed_field(bits="98")]
    pub BTN_GamePadButton3 : Integer<u8, packed_bits::Bits1>, // : 1,                     // Usage 0x00090003: Button 3 Tertiary, Value = 0 to 0
    #[packed_field(bits="99")]
    pub BTN_GamePadButton4 : Integer<u8, packed_bits::Bits1>, // : 1,                     // Usage 0x00090004: Button 4, Value = 0 to 0
    #[packed_field(bits="100")]
    pub BTN_GamePadButton5 : Integer<u8, packed_bits::Bits1>, // : 1,                     // Usage 0x00090005: Button 5, Value = 0 to 0
    #[packed_field(bits="101")]
    pub BTN_GamePadButton6 : Integer<u8, packed_bits::Bits1>, // : 1,                     // Usage 0x00090006: Button 6, Value = 0 to 0
    #[packed_field(bits="102")]
    pub BTN_GamePadButton7 : Integer<u8, packed_bits::Bits1>, // : 1,                     // Usage 0x00090007: Button 7, Value = 0 to 0
    #[packed_field(bits="103")]
    pub BTN_GamePadButton8 : Integer<u8, packed_bits::Bits1>, // : 1,                     // Usage 0x00090008: Button 8, Value = 0 to 0
    #[packed_field(bits="104")]
    pub BTN_GamePadButton9 : Integer<u8, packed_bits::Bits1>, // : 1,                     // Usage 0x00090009: Button 9, Value = 0 to 0
    #[packed_field(bits="105")]
    pub BTN_GamePadButton10 : Integer<u8, packed_bits::Bits1>, // : 1,                    // Usage 0x0009000A: Button 10, Value = 0 to 0

    #[packed_field(bits="106..=111")]
    pub pad0 : Integer<u8, packed_bits::Bits6>,
    //pub unknown2 : i8, //: 6,                                // Pad

    #[packed_field(bits="112..=119")]
    pub GD_GamePadHatSwitch : u8, // : 4,                    // Usage 0x00010039: Hat switch, Value = 1 to 8, Physical = (Value - 1) x 45 in degrees
    //pub unknown3 : i8, //: 4,                                // Pad

    #[packed_field(bits="120..=127")]
    pub GD_GamePadSystemControlSystemMainMenu : u8, // : 1,  // Usage 0x00010085: System Main Menu, Value = 0 to 1
    //pub unknown4 : i8, //: 7,                                // Pad

    #[packed_field(bits="128..=135")]
    pub GEN_GamePadBatteryStrength : u8, // : 8,             // Usage 0x00060020: Battery Strength, Value = 0 to 255
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
