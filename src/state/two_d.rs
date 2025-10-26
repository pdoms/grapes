use crate::{
    context::time::TimeInfo, engine::camera_2d::Camera2dRef, events::input::Events,
    renderer::two_d::Renderer,
};

pub trait State {
    fn user_init(renderer: &mut Renderer, camera: Camera2dRef) -> Self;
    fn user_update(
        &mut self,
        renderer: &mut Renderer,
        events: Events,
        camera: Camera2dRef,
        time_info: &TimeInfo,
    ) -> bool;
}
