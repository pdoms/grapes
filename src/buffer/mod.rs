use std::alloc::Layout;

#[derive(Clone)]
pub struct Buffer<T: Copy + Default> {
    data: *mut T,
    width: i32,
    height: i32,
    cap: usize,
}

impl<T: Copy + Default + Sized> Buffer<T> {
    pub fn init(w: i32, h: i32) -> Self {
        if std::mem::size_of::<T>() == 0 || w * h == 0 {
            panic!("We cannot allocate zero sized buffers");
        }

        let cap = (w * h) as usize;
        let layout = Layout::array::<T>(cap).expect("Layout to be buildable");
        // SECURITY: we checked type size and array len
        let data = unsafe { std::alloc::alloc_zeroed(layout) as *mut T };
        Self {
            data,
            width: w,
            height: h,
            cap,
        }
    }

    pub fn init_with_value(w: i32, h: i32, v: T) -> Self {
        if std::mem::size_of::<T>() == 0 || w * h == 0 {
            panic!("We cannot allocate zero sized buffers");
        }

        let cap = (w * h) as usize;
        let layout = Layout::array::<T>(cap).expect("Layout to be buildable");
        // SECURITY: we checked type size and array len
        let data = unsafe {
            let data = std::alloc::alloc(layout) as *mut T;
            for i in 0..w * h {
                data.add(i as usize).write(v)
            }
            data
        };

        Self {
            data,
            width: w,
            height: h,
            cap,
        }
    }


    pub fn fill(&mut self, v: T) {
        for i in 0..(self.width*self.height) {
            self.set(i, v);
        }
    }

    pub fn as_slice<'b>(&'b self) -> &'b [T] {
        unsafe { std::slice::from_raw_parts(self.data, self.width as usize * self.height as usize) }
    }
    pub fn resize(&mut self, w: usize, h: usize) {
        unimplemented!("Buffer::resize()")
    }

    pub fn len(&self) -> i32 {
        self.width * self.height
    }

    pub fn set(&mut self, index: i32, value: T) {
        if index > self.len() || index < 0 {
            return;
        }
        unsafe {
            self.data.add(index as usize).write(value);
        }
    }

    pub fn get_ptr(&self) -> *const T {
        self.data as *const T
    }


    pub fn set_xy(&mut self, x: i32, y: i32, value: T) {
        if x >= self.width || x < 0 || y < 0 || y >= self.height {
            return;
        }
        let index = y * self.width + x;
        self.set(index, value);
    }

    pub fn get(&mut self, index: i32) -> Option<*mut T> {
        if index > self.len() || index < 0 {
            return None;
        }
        unsafe { Some(self.data.add(index as usize)) }
    }

    pub fn get_xy(&mut self, x: i32, y: i32) -> Option<*mut T> {
        if x >= self.width || x < 0 || y < 0 || y >= self.height {
            return None;
        }
        let index = y * self.width + x;
        self.get(index)
    }

    pub fn offset(&mut self, index: i32) -> *mut T {
        if index > self.len() || index < 0 {
            panic!("Out of Bounds!!");
        }
        unsafe { self.data.add(index as usize) }
    }
    pub fn offset_xy(&mut self, x: i32, y: i32) -> *mut T {
        let index = y * self.width + x;
        self.offset(index)
    }

    pub fn scanline(&mut self, line: i32, x_start: i32, x_end: i32, value: T) {
        if line < 0 || line > self.height() {
            return;
        }
        unsafe {
            for i in x_start..x_end {
                if i > self.width() || i < 0 {
                    return;
                }
                self.data
                    .add((line * self.width() + i) as usize)
                    .write(value);
            }
        }
    }
    pub fn scanlinef(&mut self, line: f32, x_start: f32, x_end: f32, value: T) {
        if line < 0.0 || line >= self.height() as f32 {
            return;
        }
        let x0 = x_start.ceil().max(0.0) as usize;
        let x1 = x_end.floor().min(self.width() as f32 - 1.0) as usize;

        unsafe {
            for i in x0..x1 {
                if i > self.width() as usize {
                    return;
                }
                self.data
                    .add(line as usize * self.width() as usize + i)
                    .write(value);
            }
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn height(&self) -> i32 {
        self.height
    }
}
