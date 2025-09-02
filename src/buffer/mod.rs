use std::
    alloc::Layout
;

pub struct Buffer<T: Copy + Default> {
    data: *mut T,
    width: usize,
    height: usize,
    cap: usize,
}

impl<T: Copy + Default + Sized> Buffer<T> {
    pub fn init(w: usize, h: usize) -> Self {
        if std::mem::size_of::<T>() == 0 || w * h == 0 {
            panic!("We cannot allocate zero sized buffers");
        }

        let cap = w * h;

        let layout = Layout::array::<T>(cap).expect("Layout to be buildable");

        // SECURITY: we checked type size and array len
        let data = unsafe { std::alloc::alloc(layout) as *mut T };
        Self {
            data,
            width: w,
            height: h,
            cap,
        }
    }
    pub fn as_slice<'b>(&'b self) -> &'b [T] {
        unsafe { std::slice::from_raw_parts(self.data, self.width * self.height) }
    }
    pub fn resize(&mut self, w: usize, h: usize) {
        unimplemented!("Buffer::resize()")
    }

    pub fn len(&self) -> usize {
        self.width * self.height
    }

    pub fn set(&mut self, index: usize, value: T) {
        if index > self.len() {
            return;
        }
        unsafe {
            self.data.add(index).write(value);
        }
    }
    pub fn set_xy(&mut self, x: usize, y: usize, value: T) {
        if x > self.width {
            return;
        }
        let index = y * self.width + x;
        self.set(index, value);
    }
    pub fn offset(&mut self, index: usize) -> *mut T {
        if index > self.len() {
            panic!("Out of Bounds!!");
        }
        unsafe { self.data.add(index) }
    }
    pub fn offset_xy(&mut self, x: usize, y: usize) -> *mut T {
        let index = y * self.width + x;
        self.offset(index)
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}
