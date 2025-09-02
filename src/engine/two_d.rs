use crate::{
    context::time::TimeInfo,
    error::{ErrorX, GrapesError, ResultGrapes},
    events::input::Events,
    internal::window::Window,
    renderer::two_d::Renderer,
    state::two_d::State,
};

use super::camera_2d::Camera2dRef;

pub struct Engine<S: State> {
    w: Window,
    state: S,
    renderer: Renderer,
    time: TimeInfo,
    camera: Camera2dRef,
}

impl<S: State> Engine<S> {
    pub fn create_window(
        title: &str,
        width: usize,
        height: usize,
        mut state: S,
        camera: Camera2dRef,
    ) -> ResultGrapes<Self> {
        let w = Window::new(title, width, height)
            .map_err(|err| <ErrorX as Into<GrapesError>>::into(err))?;
        let mut renderer = Renderer::new(width, height);
        state.user_init(&mut renderer, camera.clone());
        let target_fps = w.get_fps();
        Ok(Self {
            w,
            state,
            renderer,
            time: TimeInfo::new(target_fps as u64),
            camera,
        })
    }

    pub fn set_target_fps(&mut self, fps: usize) {
        self.w.set_target_fps(fps);
    }

    pub fn camera(&mut self) -> Camera2dRef {
        self.camera.clone()
    }

    pub fn run(&mut self) -> ResultGrapes<()> {
        self.time.start();
        loop {
            if self.w.should_close() {
                break;
            }
            self.time.update();
            let events = Events::new(&self.w);
            if self
                .state
                .user_update(&mut self.renderer, events, self.camera.clone(), &self.time)
            {
                break;
            }
            let buf = self.renderer.buffer();
            self.w.update_with_buffer_stride(
                buf.as_slice(),
                buf.width(),
                buf.height(),
                buf.width(),
            )?;
        }
        Ok(())
    }
}
