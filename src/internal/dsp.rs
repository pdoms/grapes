use std::ffi::CStr;

use x11_dl::{xcursor, xlib};

use crate::error::{ErrorX, ResultX};

pub struct Dsp {
    pub(crate) lib: x11_dl::xlib::Xlib,
    pub(crate) display: *mut xlib::Display,
    pub(crate) screen: i32,
    pub(crate) visual: *mut xlib::Visual,
    pub(crate) gc: xlib::GC,
    pub(crate) _context: xlib::XContext,
    pub(crate) cursor_lib: x11_dl::xcursor::Xcursor,
    pub(crate) cursors: [xlib::Cursor; 8],
    pub(crate) screen_width: usize,
    pub(crate) screen_height: usize,
    pub(crate) depth: i32,
    pub(crate) keyboard_ext: bool,
    pub(crate) wm_delete_window: xlib::Atom,
}

impl Dsp {
    pub(crate) fn new() -> ResultX<Self> {
        let mut dsp = unsafe {
            libc::setlocale(libc::LC_ALL, std::ptr::null());
            let lib = xlib::Xlib::open()
                .map_err(|err| ErrorX::CouldNotLoadX11("xlib".into(), format!("{err:?}")))?;

            let cursor_lib = xcursor::Xcursor::open()
                .map_err(|err| ErrorX::CouldNotLoadX11("xcursor".into(), format!("{err:?}")))?;

            let display = (lib.XOpenDisplay)(std::ptr::null());

            if display.is_null() {
                return Err(ErrorX::Display("open".into(), String::new()));
            }
            let mut supported = 0;
            (lib.XkbSetDetectableAutoRepeat)(display, 1, &mut supported);

            let screen = (lib.XDefaultScreen)(display);
            let visual = (lib.XDefaultVisual)(display, screen);
            let depth = (lib.XDefaultDepth)(display, screen);

            let gc = (lib.XDefaultGC)(display, screen);

            let screen_width = usize::try_from((lib.XDisplayWidth)(display, screen))
                .map_err(|err| ErrorX::Display("screen_width".into(), format!("{err:?}")))?;
            let screen_height = usize::try_from((lib.XDisplayHeight)(display, screen))
                .map_err(|err| ErrorX::Display("screen_height".into(), format!("{err:?}")))?;

            let context = (lib.XrmUniqueQuark)();

            Self {
                lib,
                display,
                screen,
                visual,
                gc,
                cursor_lib,
                cursors: [0; 8],
                screen_width,
                screen_height,
                depth,
                keyboard_ext: false,
                wm_delete_window: 0,
                _context: context,
            }
        };

        dsp.formats()?;
        dsp.extensions();
        dsp.init_cursors();
        dsp.init_atoms();

        Ok(dsp)
    }

    fn formats(&mut self) -> ResultX<()> {
        let mut conv_depth = -1;
        unsafe {
            let mut count: i32 = -1;
            let formats = (self.lib.XListPixmapFormats)(self.display, &mut count);

            for i in 0..count {
                let pix_fmt = *formats.offset(i as isize);

                if pix_fmt.depth == self.depth {
                    conv_depth = pix_fmt.bits_per_pixel;
                }
            }

            if conv_depth != 32 {
                Err(ErrorX::Window("format".into(), "32-bit unavailable".into()))
            } else {
                Ok(())
            }
        }
    }

    fn extensions(&mut self) {
        let mut major: i32 = 1;
        let mut minor: i32 = 0;

        let mut opcode: i32 = 0;
        let mut event: i32 = 0;
        let mut error: i32 = 0;
        unsafe {
            if (self.lib.XkbQueryExtension)(
                self.display,
                &mut opcode,
                &mut event,
                &mut error,
                &mut major,
                &mut minor,
            ) != xlib::False
            {
                self.keyboard_ext = true
            }
        }
    }

    fn init_cursors(&mut self) {
        self.cursors[0] = self.load_cursors(b"arrow\0");
        self.cursors[1] = self.load_cursors(b"xterm\0");
        self.cursors[2] = self.load_cursors(b"crosshair\0");
        self.cursors[3] = self.load_cursors(b"hand2\0");
        self.cursors[4] = self.load_cursors(b"hand2\0");
        self.cursors[5] = self.load_cursors(b"sb_h_double_arrow\0");
        self.cursors[6] = self.load_cursors(b"sb_v_double_arrow\0");
        self.cursors[7] = self.load_cursors(b"diamond_cross\0");
    }

    fn load_cursors(&mut self, name: &'static [u8]) -> xlib::Cursor {
        unsafe {
            let name = CStr::from_bytes_with_nul_unchecked(name);
            (self.cursor_lib.XcursorLibraryLoadCursor)(self.display, name.as_ptr())
        }
    }

    pub fn intern_atom(&mut self, name: &'static [u8], only_if_exists: bool) -> xlib::Atom {
        unsafe {
            let name = CStr::from_bytes_with_nul_unchecked(name);
            (self.lib.XInternAtom)(
                self.display,
                name.as_ptr(),
                if only_if_exists {
                    xlib::True
                } else {
                    xlib::False
                },
            )
        }
    }

    fn init_atoms(&mut self) {
        self.wm_delete_window = self.intern_atom(b"WM_DELETE_WINDOW\0", false);
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.screen_width as u32, self.screen_height as u32)
    }
}

impl Drop for Dsp {
    fn drop(&mut self) {
        unsafe {
            (self.lib.XCloseDisplay)(self.display);
        }
    }
}
