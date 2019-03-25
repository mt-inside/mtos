use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        col: 0,
        color: ColorCode::new(Color::LightGray, Color::Blue),
        buf: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(fg: Color, bg: Color) -> Self {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VgaChar {
    ascii: u8,
    color: ColorCode,
}

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<VgaChar>; VGA_WIDTH]; VGA_HEIGHT],
}

pub struct Writer {
    col: usize,
    color: ColorCode,
    buf: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b => {
                if self.col >= VGA_WIDTH {
                    self.new_line();
                }

                let row = VGA_HEIGHT - 1;
                let col = self.col;

                let color = self.color;
                self.buf.chars[row][col].write(VgaChar {
                    ascii: b,
                    color: color,
                });
                self.col += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for b in s.bytes() {
            match b {
                // 3 dots is inclusive range
                0x20...0x7e | b'\n' => self.write_byte(b),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for r in 1..VGA_HEIGHT {
            for c in 0..VGA_WIDTH {
                let char = self.buf.chars[r][c].read();
                self.buf.chars[r - 1][c].write(char);
            }
        }
        self.clear_row(VGA_HEIGHT - 1);
        self.col = 0;
    }

    fn clear_row(&mut self, r: usize) {
        let blank = VgaChar {
            ascii: b' ',
            color: self.color,
        };
        for c in 0..VGA_WIDTH {
            self.buf.chars[r][c].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    x86_64::instructions::interrupts::without_interrupts(|| {
        /* Interrupt handlers may want to print, which would cause them to deadlock with this code. */
        WRITER.lock().write_fmt(args).unwrap();
    });
}
