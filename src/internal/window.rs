use std::{
    ffi::{CStr, CString, c_char, c_int, c_uchar, c_void},
    ptr::NonNull,
};

use libc::{c_uint, c_ulong};
use raw_window_handle::{
    DisplayHandle, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
    WindowHandle, XlibDisplayHandle, XlibWindowHandle,
};
use x11_dl::xlib::{
    self, KeyPressMask, KeyReleaseMask, XIC, XIM, XIMPreeditNothing, XIMStatusNothing,
    XNClientWindow_0, XNFocusWindow_0, XNInputStyle_0, XrmDatabase,
};

use crate::{
    error::{ErrorX, ResultX},
    events::keyboard::K,
    internal::{dsp::Dsp, event::EventProcessResult, keys::Keys, rate::Fps},
};

#[allow(unused)]
const BUTTON_6: c_uint = xlib::Button5 + 1;
#[allow(unused)]
const BUTTON_7: c_uint = xlib::Button5 + 2;

#[allow(unused)]
pub struct Window {
    dsp: Dsp,
    handle: xlib::Window,
    xim: XIM,
    xic: XIC,
    ximage: *mut xlib::XImage,
    buffer: Vec<u32>,
    width: u32,
    height: u32,
    scale: i32,
    background: u32,
    mouse_x: f32,
    mouse_y: f32,
    scroll_x: f32,
    scroll_y: f32,
    buttons: [u8; 3],
    active: bool,
    fps: Fps,
    should_close: bool,
    key_state: Keys,
}

impl HasDisplayHandle for Window {
    fn display_handle(
        &self,
    ) -> std::result::Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError>
    {
        let raw_display = self.dsp.display as *mut c_void;
        let display = NonNull::new(raw_display);
        let handle = XlibDisplayHandle::new(display, self.dsp.screen);
        let raw_handle = RawDisplayHandle::Xlib(handle);
        unsafe { Ok(DisplayHandle::borrow_raw(raw_handle)) }
    }
}

impl HasWindowHandle for Window {
    fn window_handle(
        &self,
    ) -> std::result::Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError>
    {
        let handle = XlibWindowHandle::new(self.handle);
        let raw_handle = RawWindowHandle::Xlib(handle);
        unsafe { Ok(WindowHandle::borrow_raw(raw_handle)) }
    }
}

impl Window {
    pub fn new(title: &str, width: usize, height: usize) -> ResultX<Window> {
        let title = match CString::new(title) {
            Ok(t) => t,
            Err(_) => {
                return Err(ErrorX::TypeConversion(
                    title.into(),
                    "c_string - window title".into(),
                ));
            }
        };

        let mut dsp = Dsp::new()?;

        unsafe {
            let mut xwa: xlib::XSetWindowAttributes = std::mem::zeroed();

            let root = (dsp.lib.XDefaultRootWindow)(dsp.display);

            xwa.border_pixel = (dsp.lib.XBlackPixel)(dsp.display, dsp.screen);
            xwa.backing_pixel = xwa.border_pixel;
            xwa.backing_store = xlib::NotUseful;

            let x = if dsp.screen_width > width {
                (dsp.screen_width - width) / 2
            } else {
                0
            };

            let y = if dsp.screen_height > height {
                (dsp.screen_height - height) / 2
            } else {
                0
            };

            let handle = (dsp.lib.XCreateWindow)(
                dsp.display,
                root,
                x as i32,
                y as i32,
                width as u32,
                height as u32,
                0,
                dsp.depth,
                xlib::InputOutput as u32,
                dsp.visual,
                xlib::CWBackingStore | xlib::CWBackPixel | xlib::CWBorderPixel,
                &mut xwa,
            );

            let empty_string = b"\0";

            (dsp.lib.XSetLocaleModifiers)(empty_string.as_ptr() as _);

            let xim = (dsp.lib.XOpenIM)(
                dsp.display,
                0 as XrmDatabase,
                std::ptr::null_mut::<c_char>(),
                std::ptr::null_mut::<c_char>(),
            );

            if (xim as usize) == 0 {
                return Err(ErrorX::Generic(
                    "Could not setup X IM calling XOpenIM".into(),
                ));
            }

            let xn_input_style = CStr::from_bytes_with_nul_unchecked(XNInputStyle_0);
            let xn_client_window = CStr::from_bytes_with_nul_unchecked(XNClientWindow_0);
            let xn_focus_window = CStr::from_bytes_with_nul_unchecked(XNFocusWindow_0);

            let xic = (dsp.lib.XCreateIC)(
                xim,
                xn_input_style.as_ptr(),
                XIMPreeditNothing | XIMStatusNothing,
                xn_client_window.as_ptr(),
                handle as c_ulong,
                xn_focus_window.as_ptr(),
                handle as c_ulong,
                std::ptr::null_mut::<c_void>(),
            );

            if (xic as usize) == 0 {
                return Err(ErrorX::Generic(
                    "Failed to setup X IC calling XCreateIc".into(),
                ));
            }

            (dsp.lib.XSetICFocus)(xic);
            (dsp.lib.XSelectInput)(dsp.display, handle, KeyPressMask | KeyReleaseMask);
            dsp.gc = (dsp.lib.XCreateGC)(dsp.display, handle, 0, std::ptr::null_mut());
            if handle == 0 {
                return Err(ErrorX::Window(
                    "Unable to open window".into(),
                    "bummer".into(),
                ));
            }

            Self::set_title_raw(&mut dsp, handle, &title)?;

            (dsp.lib.XSelectInput)(
                dsp.display,
                handle,
                xlib::StructureNotifyMask
                    | xlib::KeyPressMask
                    | xlib::KeyReleaseMask
                    | xlib::ButtonPressMask
                    | xlib::ButtonReleaseMask
                    | xlib::FocusChangeMask,
            );

            (dsp.lib.XClearWindow)(dsp.display, handle);
            (dsp.lib.XMapRaised)(dsp.display, handle);
            (dsp.lib.XSetWMProtocols)(dsp.display, handle, &mut dsp.wm_delete_window, 1);
            (dsp.lib.XFlush)(dsp.display);

            let mut draw_buffer: Vec<u32> = Vec::new();

            let ximage = match Self::alloc_image(&dsp, width, height, &mut draw_buffer) {
                Some(ximg) => ximg,
                None => {
                    (dsp.lib.XDestroyWindow)(dsp.display, handle);
                    return Err(ErrorX::Window(
                        "Could not create pixel buffer".into(),
                        "Maybe next time...".into(),
                    ));
                }
            };
            Ok(Self {
                dsp,
                handle,
                xim,
                xic,
                ximage,
                buffer: draw_buffer,
                width: width as u32,
                height: height as u32,
                scale: 1,
                background: 0,
                mouse_x: 0.0,
                mouse_y: 0.0,
                scroll_x: 0.0,
                scroll_y: 0.0,
                buttons: [0, 0, 0],
                active: false,
                should_close: false,
                fps: Fps::new(),
                key_state: Keys::new(),
            })
        }
    }

    fn alloc_image(
        dsp: &Dsp,
        width: usize,
        height: usize,
        draw_buffer: &mut Vec<u32>,
    ) -> Option<*mut xlib::XImage> {
        let bytes_per_line = (width as i32) * 4;

        draw_buffer.resize(width * height, 0);
        let image = unsafe {
            (dsp.lib.XCreateImage)(
                dsp.display,
                dsp.visual,
                dsp.depth as u32,
                xlib::ZPixmap,
                0,
                draw_buffer[..].as_mut_ptr() as *mut c_char,
                width as u32,
                height as u32,
                32,
                bytes_per_line,
            )
        };
        if image.is_null() { None } else { Some(image) }
    }

    fn free_image(&mut self) {
        unsafe {
            (*self.ximage).data = std::ptr::null_mut();
            (self.dsp.lib.XDestroyImage)(self.ximage);
            self.ximage = std::ptr::null_mut();
        }
    }

    fn set_title_raw(dsp: &mut Dsp, handle: xlib::Window, name: &CStr) -> ResultX<()> {
        unsafe { (dsp.lib.XStoreName)(dsp.display, handle, name.as_ptr()) };
        if let Ok(name_len) = c_int::try_from(name.to_bytes().len()) {
            let net_wm_name = dsp.intern_atom(b"_NET_WM_NAME\0", false);
            let utf8_string = dsp.intern_atom(b"UTF8_STRING\0", false);
            unsafe {
                (dsp.lib.XChangeProperty)(
                    dsp.display,
                    handle,
                    net_wm_name,
                    utf8_string,
                    8,
                    xlib::PropModeReplace,
                    name.as_ptr() as *const c_uchar,
                    name_len,
                );
            }
            Ok(())
        } else {
            Err(ErrorX::TypeConversion(
                "Window title too long".into(),
                "too bad".into(),
            ))
        }
    }

    pub fn set_title(&mut self, title: &str) {
        match CString::new(title) {
            Err(_) => {
                println!("Unable to convert {} to c_string", title);
            }
            Ok(t) => {
                if let Err(err) = Self::set_title_raw(&mut self.dsp, self.handle, &t) {
                    println!("{err}");
                }
            }
        }
    }

    pub fn update_with_buffer_stride(
        &mut self,
        buffer: &[u32],
        buf_width: usize,
        buf_height: usize,
        buf_stride: usize,
    ) -> ResultX<()> {
        self.raw_blit_buffer(buffer, buf_width, buf_height, buf_stride);
        self.update();
        Ok(())
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width as usize, self.height as usize)
    }

    fn raw_blit_buffer(
        &mut self,
        buffer: &[u32],
        _buf_width: usize,
        _buf_height: usize,
        _buf_stride: usize,
    ) {
        for (i, p) in self.buffer.iter_mut().enumerate() {
            *p = buffer[i];
        }

        unsafe {
            (self.dsp.lib.XPutImage)(
                self.dsp.display,
                self.handle,
                self.dsp.gc,
                self.ximage,
                0,
                0,
                0,
                0,
                self.width,
                self.height,
            );
            (self.dsp.lib.XFlush)(self.dsp.display);
        }
    }

    pub fn set_target_fps(&mut self, fps: usize) {
        self.fps.set_target_fps(fps);
    }

    pub fn get_fps(&self) -> usize {
        self.fps.get_target_fps()
    }

    fn raw_process_events(&mut self) {
        let count = unsafe { (self.dsp.lib.XPending)(self.dsp.display) };
        unsafe {
            for _ in 0..count {
                let mut event: xlib::XEvent = std::mem::zeroed();
                if (self.dsp.lib.XNextEvent)(self.dsp.display, &mut event) != 0 {
                    continue;
                }
                if self.raw_process_one_event(event) == EventProcessResult::Terminate {
                    return;
                };
            }
        }
    }

    fn raw_process_one_event(&mut self, event: xlib::XEvent) -> EventProcessResult {
        unsafe {
            if event.any.window != self.handle {
                return EventProcessResult::Ok;
            }

            match event.type_ {
                xlib::ClientMessage => {
                    if event.client_message.format == 32
                        && event.client_message.data.get_long(0) as xlib::Atom
                            == self.dsp.wm_delete_window
                    {
                        self.should_close = true;
                        return EventProcessResult::Terminate;
                    }
                }
                xlib::KeyPress => {
                    self.process_key(event, true);
                }
                xlib::KeyRelease => {
                    self.process_key(event, false);
                }
                xlib::ButtonPress => {}
                xlib::ButtonRelease => {}
                xlib::ConfigureNotify => {}
                xlib::FocusIn => {
                    self.active = true;
                }
                xlib::FocusOut => {
                    self.active = false;
                }
                _ => {}
            }
            EventProcessResult::Ok
        }
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn update(&mut self) {
        self.key_state.update();
        self.fps.update();
        self.raw_get_mouse_pos();
        self.raw_process_events();
    }

    pub fn process_key(&mut self, mut ev: xlib::XEvent, is_pressed: bool) {
        let sym: xlib::KeySym = unsafe { (self.dsp.lib.XLookupKeysym)(&mut ev.key, 0) };

        if sym == xlib::NoSymbol as xlib::KeySym {
            return;
        }
        self.update_key_state(sym, is_pressed);
    }

    fn update_key_state(&mut self, sym: xlib::KeySym, is_pressed: bool) {
        if sym > u32::MAX as xlib::KeySym {
            return;
        }
        if is_pressed {
            self.key_state.set(sym.into());
        } else {
            self.key_state.clear(sym.into());
        }
    }

    pub fn key_pressed(&self, k: K) -> bool {
        self.key_state.is_pressed(k)
    }
    pub fn key_down(&self, k: K) -> bool {
        self.key_state.is_down(k)
    }
    pub fn key_released(&self, k: K) -> bool {
        self.key_state.is_released(k)
    }

    fn raw_get_mouse_pos(&mut self) {
        let mut root: xlib::Window = 0;
        let mut root_x: i32 = 0;
        let mut root_y: i32 = 0;

        let mut child: xlib::Window = 0;
        let mut child_x: i32 = 0;
        let mut child_y: i32 = 0;

        let mut mask: u32 = 0;

        unsafe {
            if (self.dsp.lib.XQueryPointer)(
                self.dsp.display,
                self.handle,
                &mut root,
                &mut child,
                &mut root_x,
                &mut root_y,
                &mut child_x,
                &mut child_y,
                &mut mask,
            ) != xlib::False
            {
                self.mouse_x = child_x as f32;
                self.mouse_y = child_y as f32;
            }
        }
    }

    #[allow(unused)]
    fn process_button(&mut self, event: xlib::XEvent, is_down: bool) {
        unsafe {
            match event.button.button {
                xlib::Button1 => {
                    self.buttons[0] = if is_down { 1 } else { 0 };
                    return;
                }
                xlib::Button2 => {
                    self.buttons[1] = if is_down { 1 } else { 0 };
                    return;
                }
                xlib::Button3 => {
                    self.buttons[2] = if is_down { 1 } else { 0 };
                    return;
                }
                _ => {}
            }
            let scroll: (i32, i32) = match event.button.button {
                xlib::Button4 => (0, 10),
                xlib::Button5 => (0, -10),
                BUTTON_6 => (10, 0),
                BUTTON_7 => (-10, 0),
                _ => {
                    return;
                }
            };
            self.scroll_x += scroll.0 as f32 * 0.1;
            self.scroll_y += scroll.1 as f32 * 0.1;
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn get_mouse_position(&self) -> Option<(f32, f32)> {
        Some((self.mouse_x, self.mouse_y))
    }

    pub fn get_screen_size(&self) -> (u32, u32) {
        self.dsp.get_size()
    }

    pub fn button_is_down(&self, b: usize) -> bool {
        self.buttons[b] > 0
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            self.free_image();
            (self.dsp.lib.XDestroyIC)(self.xic);
            (self.dsp.lib.XCloseIM)(self.xim);
            (self.dsp.lib.XDestroyWindow)(self.dsp.display, self.handle);
        }
    }
}
