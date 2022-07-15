mod main_render_system;
mod pipeline;
mod render_system;
pub use main_render_system::MainRenderSystem;
pub use render_system::RenderSystem;
#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let b: $base = mem::zeroed();
            (&b.$field as *const _ as isize) - (&b as *const _ as isize)
        }
    }};
}

