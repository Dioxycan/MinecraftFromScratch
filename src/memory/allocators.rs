use crate::core::Core;
use crate::utils::list::{Link, List, Node};
use ash::vk;
use std::cell::{RefCell, RefMut};
use std::ffi::c_void;
use std::rc::Rc;
use std::result::Result;

use super::buffers::Buffer;
pub struct Allocator {
    core: Rc<Core>,
    pub handle: vk::DeviceMemory,
    size: vk::DeviceSize,
    pub free_memory: vk::DeviceSize,
    pub memory_type_index: u32,
    pub memory_flags: vk::MemoryPropertyFlags,
    pub list: List<Block>,
    pub host_data_ptr: Option<*mut c_void>,
    pub alignment: vk::DeviceSize,
}
#[derive(Debug)]
pub struct Block {
    offset: vk::DeviceSize,
    size: vk::DeviceSize,
    buffer_index: Option<usize>,
}
impl Block {
    pub fn new(offset: vk::DeviceSize, size: vk::DeviceSize, buffer_index: Option<usize>) -> Self {
        Self {
            offset,
            size,
            buffer_index: None,
        }
    }
}
impl Allocator {
    pub fn new(
        core: Rc<Core>,
        allocation_size: vk::DeviceSize,
        memory_flags: vk::MemoryPropertyFlags,
        alignment: vk::DeviceSize,
    ) -> Self {
        let mut size = alignment*(allocation_size/alignment);
        if size < allocation_size {
            size += alignment;
        }
        let memory_properties = unsafe {
            core.instance
                .get_physical_device_memory_properties(core.physical_device)
        };
        let type_index = memory_properties
            .memory_types
            .iter()
            .enumerate()
            .find(|(i, memory_type)| {
                memory_type.property_flags == memory_flags
                    && memory_properties.memory_heaps[memory_type.heap_index as usize].size / 4
                        >= size
            })
            .map(|(i, _)| i as u32)
            .unwrap();
        let create_info = create_allocator_info(type_index, size);
        let handle = unsafe {
            core.logical_device
                .allocate_memory(&create_info, None)
                .expect("Failed to allocate memory")
        };
        Self {
            core,
            handle,
            size: size,
            memory_type_index: type_index,
            memory_flags: memory_flags,
            free_memory: size,
            list: List::new(Block::new(0, size, None)),
            host_data_ptr: None,
            alignment,
        }
    }
    pub fn allocate_memory(&mut self, buffer: &mut Buffer, buffer_index: usize) {
        let node = self.find_block(buffer.memory_requirements);

        if let Some(node) = node {
            let mut mut_node = node.borrow_mut();
            let block = &mut mut_node.data;

            println!("Allocating memory for buffer {}", buffer.size);
            if block.size - buffer.size > 0 {
                let mut size = self.alignment * (buffer.size/ self.alignment);
                if size < buffer.size {
                    size += self.alignment;
                }
                println!("{:?}", size);
                let new_block = Block::new(
                    size,
                    block.size -size,
                    None,
                );
                block.size = size;
                block.buffer_index = Some(buffer_index);
                buffer.offsets.push(block.offset);
                unsafe {
                    self.core
                        .logical_device
                        .bind_buffer_memory(buffer.handle, self.handle, block.offset)
                        .expect("Failed to bind buffer memory")
                };

                let new_node = Node {
                    data: new_block,
                    next: mut_node.next.clone(),
                };
                mut_node.next = Some(Rc::new(RefCell::new(new_node)));
            } else {
                block.buffer_index = Some(buffer_index);
            }
         
        } else {
            panic!("Failed to allocate memory");
        }
        self.free_memory -= buffer.size;
    }
    pub fn free_memory(&mut self, buffer_index: usize) {
        let mut pre: Option<Rc<RefCell<Node<Block>>>> = None;
        self.list.iter().for_each(|node| {
            let mut mut_node = node.borrow_mut();
            let block = &mut mut_node.data;
            if let Some(index) = block.buffer_index {
                if index == buffer_index {
                    block.buffer_index = None;
                    self.free_memory += block.size;
                } else {
                    pre = None;
                }
            }
            if let Some(ref pre) = pre {
                let mut mut_pre = pre.borrow_mut();
                let pre_block = &mut mut_pre.data;
                pre_block.size += block.size;
                mut_pre.next = mut_node.next.clone();
            } else {
                pre = Some(node.clone());
            }
        });
    }
    pub fn find_block(&mut self, memory_requirements: vk::MemoryRequirements) -> Link<Block> {
        self.list.iter().find(|node| {
            let block = &node.borrow().data;
            block.buffer_index.is_none() &&block.size >= memory_requirements.size
        })
    }
    pub fn map_memory(&mut self) {
        if self
            .memory_flags
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            self.host_data_ptr = unsafe {
                Some(
                    self.core
                        .logical_device
                        .map_memory(self.handle, 0, self.size, vk::MemoryMapFlags::empty())
                        .expect("Failed to map memory"),
                )
            }
        }
    }
}

fn create_allocator_info(
    memory_type_index: u32,
    allocation_size: vk::DeviceSize,
) -> vk::MemoryAllocateInfo {
    vk::MemoryAllocateInfo::builder()
        .allocation_size(allocation_size)
        .memory_type_index(memory_type_index)
        .build()
}
