use super::Block;
use crate::core::Core;
use crate::utils::list::{Link, List, Node};
use ash::vk;
use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::c_void;
pub struct SubMemory {
    pub offset: vk::DeviceSize,
    pub size: vk::DeviceSize,
    pub is_free: bool,
    pub id: usize,
}
pub struct Buffer {
    core: Rc<Core>,
    pub handle: vk::Buffer,
    pub offsets: Vec<vk::DeviceSize>,
    pub allocator_id: Option<usize>,
    pub block_id: Option<usize>,
    pub memory_type: vk::MemoryPropertyFlags,
    pub memory_requirements: vk::MemoryRequirements,
    pub buffer_usage: vk::BufferUsageFlags,
    pub size: vk::DeviceSize,
    pub data: Option<*mut c_void>,
    pub free_memory: vk::DeviceSize,
    pub sub_memory_ids: usize,
    pub sub_memory_list: List<SubMemory>,
}

impl Buffer {
    pub fn new(
        core: Rc<Core>,
        buffer_info: vk::BufferCreateInfo,
        memory_type: vk::MemoryPropertyFlags,
    ) -> Self {
        let handle = unsafe {
            core.logical_device
                .create_buffer(&buffer_info, None)
                .expect("Failed to create buffer")
        };
        Self {
            handle,
            offsets: vec![],
            memory_type,
            memory_requirements: unsafe {
                core.logical_device.get_buffer_memory_requirements(handle)
            },
            size: buffer_info.size,
            data: None,
            buffer_usage: buffer_info.usage,
            free_memory: buffer_info.size,
            core,
            sub_memory_ids: 1,
            sub_memory_list: List::new(SubMemory {
                offset: 0,
                size: buffer_info.size,
                is_free: true,
                id: 0,
            }),
            allocator_id: None,
            block_id: None,
        }
    }
    pub fn allocate_sub_memory(&mut self, size: vk::DeviceSize) -> Link<SubMemory> {
        let mut best_fit: Link<SubMemory> = None;
        for node in self.sub_memory_list.iter() {
            let block = &node.borrow().data;
            if block.is_free && block.size >= size {
                best_fit = Some(node.clone());
                break;
            }
        }
        if let Some(ref node) = best_fit {
            let mut mut_node = node.borrow_mut();
            let block = &mut_node.data;
            if block.size > size {
                let new_block = SubMemory {
                    offset: block.offset+size,
                    size: block.size-size,
                    is_free: true,
                    id: self.sub_memory_ids,
                };
                self.sub_memory_ids += 1;
                let new_node = Node {
                    data: new_block,
                    next: mut_node.next.clone(),
                };
                mut_node.next = Some(Rc::new(RefCell::new(new_node)));
            } else {
            }
            let block = &mut mut_node.data;
            block.is_free = false;
        } else {
        }
        self.free_memory -= size;
        best_fit.clone()
    }
    pub fn free_sub_memory(&mut self, sub_memory_id: usize) {
        let mut pre: Link<SubMemory> = None;
        self.sub_memory_list.iter().for_each(|node| {
            let mut mut_node = node.borrow_mut();
            let sub_memory = &mut mut_node.data;
            if !sub_memory.is_free {
                if sub_memory_id == sub_memory.id {
                    sub_memory.is_free = true;
                    self.free_memory += sub_memory.size;
                } else {
                    pre = None;
                }
            }
            if let Some(ref pre) = pre {
                let mut mut_pre = pre.borrow_mut();
                let pre_sub_memory = &mut mut_pre.data;
                pre_sub_memory.size += sub_memory.size;
                mut_pre.next = mut_node.next.clone();
            } else {
                pre = Some(node.clone());
            }
        });
    }
}
impl Drop for Buffer {
    fn drop(&mut self) {
        println!("drop buffer");
        unsafe {
            self.core.logical_device.destroy_buffer(self.handle, None);
        }
    }
}
