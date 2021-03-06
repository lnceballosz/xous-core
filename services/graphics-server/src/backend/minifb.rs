use minifb::{Key, Window, WindowOptions, KeyRepeat};

const WIDTH: usize = 336;
const HEIGHT: usize = 536;

/// Width of the screen in 32-bit words
const WIDTH_WORDS: usize = 11;
const FB_SIZE: usize = WIDTH_WORDS * HEIGHT; // 44 bytes by 536 lines

const MAX_FPS: u64 = 15;
const DARK_COLOUR: u32 = 0xB5B5AD;
const LIGHT_COLOUR: u32 = 0x1B1B19;

pub struct XousDisplay {
    native_buffer: Vec<u32>, //[u32; WIDTH * HEIGHT],
    emulated_buffer: [u32; FB_SIZE],
    window: Window,
    kbd_conn: xous::CID,
}

impl XousDisplay {
    pub fn new() -> XousDisplay {
        let mut window = Window::new(
            "Betrusted",
            WIDTH,
            HEIGHT,
            WindowOptions {
                scale_mode: minifb::ScaleMode::AspectRatioStretch,
                resize: true,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // Limit the maximum refresh rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(
            1000 * 1000 / MAX_FPS,
        )));

        let native_buffer = vec![DARK_COLOUR; WIDTH * HEIGHT];
        window
            .update_with_buffer(&native_buffer, WIDTH, HEIGHT)
            .unwrap();

        let kbd_conn = xous_names::request_connection_blocking(xous::names::SERVER_NAME_KBD).expect("GFX|hosted: can't connect to KBD for emulation");

        XousDisplay {
            native_buffer,
            window,
            emulated_buffer: [0u32; FB_SIZE],
            kbd_conn,
        }
    }

    pub fn blit_screen(&mut self, bmp: [u32; FB_SIZE]) {
        for (dest, src) in self.emulated_buffer.iter_mut().zip(bmp.iter()) {
            *dest = *src;
        }
    }

    pub fn native_buffer(&mut self) -> &mut [u32; FB_SIZE] {
        &mut self.emulated_buffer
    }

    pub fn redraw(&mut self) {
        self.emulated_to_native();
        self.window
            .update_with_buffer(&self.native_buffer, WIDTH, HEIGHT)
            .unwrap();
    }

    pub fn update(&mut self) {
        self.emulated_to_native();
        self.window.update();
        if !self.window.is_open() || self.window.is_key_down(Key::Escape) {
            std::process::exit(0);
        }

        let pressed = self.window.get_keys_pressed(KeyRepeat::No);
        if let Some(keys) = pressed {
            for k in keys {
                log::info!("GFX|hosted: sending key {:?}", k);
                let c = self.decode_key(k);
                if c != '\u{0000}' {
                    xous::send_message(self.kbd_conn, keyboard::api::Opcode::HostModeInjectKey(c).into()).map(|_| ()).unwrap();
                }
            }
        }
    }

    fn emulated_to_native(&mut self) {
        for (dest_row, src_row) in self
            .native_buffer
            .chunks_mut(WIDTH as _)
            .zip(self.emulated_buffer.chunks(WIDTH_WORDS as _))
        {
            for (dest_cell, src_cell) in dest_row.chunks_mut(32).zip(src_row) {
                for (bit, dest) in dest_cell.iter_mut().enumerate() {
                    *dest = if src_cell & (1 << bit) != 0 {
                        DARK_COLOUR
                    } else {
                        LIGHT_COLOUR
                    };
                }
            }
        }
    }

    fn decode_key(&mut self, k: Key) -> char {
        let mut shift = false;
        if self.window.is_key_pressed(Key::LeftShift, KeyRepeat::Yes) ||
           self.window.is_key_pressed(Key::RightShift, KeyRepeat::Yes) {
            shift = true;
        }
        let base: char =
            if shift == false {
                match k {
                    Key::A => 'a',
                    Key::B => 'b',
                    Key::C => 'c',
                    Key::D => 'd',
                    Key::E => 'e',
                    Key::F => 'f',
                    Key::G => 'g',
                    Key::H => 'h',
                    Key::I => 'i',
                    Key::J => 'j',
                    Key::K => 'k',
                    Key::L => 'l',
                    Key::M => 'm',
                    Key::N => 'n',
                    Key::O => 'o',
                    Key::P => 'p',
                    Key::Q => 'q',
                    Key::R => 'r',
                    Key::S => 's',
                    Key::T => 't',
                    Key::U => 'u',
                    Key::V => 'v',
                    Key::W => 'w',
                    Key::X => 'x',
                    Key::Y => 'y',
                    Key::Z => 'z',
                    Key::Key0 => '0',
                    Key::Key1 => '1',
                    Key::Key2 => '2',
                    Key::Key3 => '3',
                    Key::Key4 => '4',
                    Key::Key5 => '5',
                    Key::Key6 => '6',
                    Key::Key7 => '7',
                    Key::Key8 => '8',
                    Key::Key9 => '9',
                    Key::Left => '←',
                    Key::Right => '→',
                    Key::Up => '↑',
                    Key::Down => '↓',
                    Key::Home => '∴',
                    Key::Backspace => 0x8_u8.into(),
                    Key::Delete => 0x8_u8.into(),
                    Key::Space => ' ',
                    Key::Comma => ',',
                    Key::Period => '.',
                    _ => '\u{0000}',
                }
            } else {
                match k {
                    Key::A => 'A',
                    Key::B => 'B',
                    Key::C => 'C',
                    Key::D => 'D',
                    Key::E => 'E',
                    Key::F => 'F',
                    Key::G => 'G',
                    Key::H => 'H',
                    Key::I => 'I',
                    Key::J => 'J',
                    Key::K => 'K',
                    Key::L => 'L',
                    Key::M => 'M',
                    Key::N => 'N',
                    Key::O => 'O',
                    Key::P => 'P',
                    Key::Q => 'Q',
                    Key::R => 'R',
                    Key::S => 'S',
                    Key::T => 'T',
                    Key::U => 'U',
                    Key::V => 'V',
                    Key::W => 'W',
                    Key::X => 'X',
                    Key::Y => 'Y',
                    Key::Z => 'Z',
                    Key::Key0 => ')',
                    Key::Key1 => '!',
                    Key::Key2 => '@',
                    Key::Key3 => '#',
                    Key::Key4 => '$',
                    Key::Key5 => '%',
                    Key::Key6 => '^',
                    Key::Key7 => '&',
                    Key::Key8 => '*',
                    Key::Key9 => '(',
                    Key::Left => '←',
                    Key::Right => '→',
                    Key::Up => '↑',
                    Key::Down => '↓',
                    Key::Home => '∴',
                    Key::Backspace => 0x8_u8.into(),
                    Key::Delete => 0x8_u8.into(),
                    Key::Space => ' ',
                    Key::Comma => '<',
                    Key::Period => '>',
                    _ => '\u{0000}',
                }
            };
        base
    }
}
