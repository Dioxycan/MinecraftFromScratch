use std::{rc::Rc, borrow::BorrowMut, cell::{RefCell,RefMut}};

use super::{command::Command, swap_chain};
use super::swap_chain::SwapChain;
use crate::game::{core::Core, window::Window};
use ash::vk;
pub struct Renderer{
    pub swap_chain: SwapChain,
    is_frame_started: bool,
    core: Rc<Core>,
    pub command: Command,
    pub current_frame_index: u32,
    pub current_image_index: u32,
    pub is_window_resized: bool,
    pub window_extent: vk::Extent2D,
}
impl Renderer {
    pub fn new(core: Rc<Core>, window_extent: vk::Extent2D) -> Self {
        let mut swap_chain = SwapChain::new(core.clone());
        swap_chain.init(&window_extent,None);
        let command = Command::new(&core);
        Renderer {
            core,
            swap_chain: swap_chain,
            is_frame_started: false,
            window_extent,
            command,
            current_frame_index: 0,
            current_image_index: 0,
            is_window_resized: false,

        }
    }
    pub fn recreate_swap_chain(&mut self) {
        unsafe{
            self.core.logical_device.device_wait_idle().unwrap();
        }
        let mut swap_chain = SwapChain::new(self.core.clone());
        println!("{:?}",self.window_extent);
        swap_chain.init(&self.window_extent,Some(self.swap_chain.swap_chain));
        self.swap_chain = swap_chain;
    }
    pub fn get_render_pass(&self) -> &vk::RenderPass {
        self.swap_chain.get_render_pass()
    }
    pub fn get_current_command_buffer(&self) -> vk::CommandBuffer {
        self.command.command_buffers[self.current_frame_index as usize]
    }
    pub fn begin_frame(&mut self) -> vk::CommandBuffer {
        assert!(self.is_frame_started == false, "Frame already started");
        let command_buffer = self.get_current_command_buffer();
        match self.swap_chain.acquire_next_image() {
            Ok((image_index,is_ok)) => {
                self.current_image_index = image_index;
                self.is_frame_started = true;
                let begin_info = vk::CommandBufferBeginInfo::default();
                unsafe {
                    self.core
                        .logical_device
                        .begin_command_buffer(command_buffer, &begin_info)
                        .expect("Failed to begin command buffer");
                }
            }
            Err(ash::vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                println!(
                    "Failed to acquire next image: {:?}",
                    ash::vk::Result::ERROR_OUT_OF_DATE_KHR
                );
                self.recreate_swap_chain();
                return vk::CommandBuffer::null();
            }
            Err(ash::vk::Result::SUBOPTIMAL_KHR) => {
                panic!(
                    "Failed to acquire next image: {:?}",
                    ash::vk::Result::SUBOPTIMAL_KHR
                );
            }
            Err(err) => {
                panic!("Failed to acquire next image: {:?}", err);
            }
        }
        return command_buffer;
    }
    pub fn end_frame(&mut self) {
        assert!(
            self.is_frame_started,
            "Can't call end_frame if frame is not in progress"
        );
        let command_buffer = self.get_current_command_buffer();
        unsafe {
            self.core
                .logical_device
                .end_command_buffer(command_buffer)
                .expect("Failed to end command buffer");
        }
        let result = self.swap_chain.submit_command_buffer(&command_buffer,&self.current_image_index);
        if result == Err(ash::vk::Result::ERROR_OUT_OF_DATE_KHR) || result == Err(ash::vk::Result::SUBOPTIMAL_KHR)|| self.is_window_resized{
            println!("Failed to submit command buffer: {:?}", result);
            self.is_window_resized = false;
            self.recreate_swap_chain();
            self.is_frame_started = false;
            return;
        }
        self.is_frame_started = false;
        self.current_frame_index = (self.current_frame_index +1) % super::swap_chain::MAX_FRAMES_IN_FLIGHT as u32;
    }
    pub fn begin_render_pass(&mut self, command_buffer: vk::CommandBuffer) {
        assert!(
            self.is_frame_started,
            "Can't call begin_render_pass if frame is not in progress"
        );

        assert!(
            command_buffer == self.get_current_command_buffer(),
            "Can't begin render pass on command buffer from a different frame"
        );
        let render_pass_info = vk::RenderPassBeginInfo::builder()
            .render_pass(*self.get_render_pass())
            .framebuffer(self.swap_chain.frame_buffers[self.current_image_index as usize])
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.swap_chain.swap_chain_extent,
            })
            .clear_values(&[
                vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 1.0],
                    },
                },
                vk::ClearValue {
                    depth_stencil: vk::ClearDepthStencilValue {
                        depth: 1.0,
                        stencil: 0,
                    },
                },
            ]);
        unsafe {
            self.core.logical_device.cmd_begin_render_pass(
                command_buffer,
                &render_pass_info,
                vk::SubpassContents::INLINE,
            );
        }
        let viewport = vk::Viewport::builder()
            .x(0.0)
            .y(0.0)
            .width(self.swap_chain.swap_chain_extent.width as f32)
            .height(self.swap_chain.swap_chain_extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0)
            .build();
        let scissor = vk::Rect2D::builder()
            .offset(vk::Offset2D { x: 0, y: 0 })
            .extent(self.swap_chain.swap_chain_extent)
            .build();
        unsafe {
            self.core
                .logical_device
                .cmd_set_viewport(command_buffer, 0, &[viewport]);
            self.core
                .logical_device
                .cmd_set_scissor(command_buffer, 0, &[scissor]);
        }
    }
    pub fn end_render_pass(&mut self, command_buffer: vk::CommandBuffer) {
        assert!(
            self.is_frame_started,
            "Can't call end_render_pass if frame is not in progress"
        );
        assert!(
            command_buffer == self.get_current_command_buffer(),
            "Can't begin render pass on command buffer from a different frame"
        );
        unsafe {
            self.core.logical_device.cmd_end_render_pass(command_buffer);
        }
    }
}
