use super::AllocationType;
use crate::core::Core;
use crate::utils::list::{Link, List, Node};
use ash::vk;
use std::cell::{RefCell, RefMut};
use std::ffi::c_void;
use std::rc::Rc;

use super::buffers::Buffer;

fn has_granularity_conflict(mut type1: AllocationType, mut type2: AllocationType) -> bool {
    if type1 > type2 {
        std::mem::swap(&mut type1, &mut type2);
    }
    match type1 {
        AllocationType::Free => false,
        AllocationType::Buffer => {
            return type2 == AllocationType::Image || type2 == AllocationType::ImageOptimal;
        }
        AllocationType::Image => {
            return type2 == AllocationType::Image
                || type2 == AllocationType::ImageLinear
                || type2 == AllocationType::ImageOptimal;
        }
        AllocationType::ImageLinear => {
            return type2 == AllocationType::ImageOptimal;
        }
        AllocationType::ImageOptimal => false,
    }
}

fn is_on_same_page<T>(r_a_offset: T, r_a_size: T, r_b_offset: T, page_size: T) -> bool
where
    T: std::convert::Into<i128>,
{
    let r_a_offset: i128 = r_a_offset.into();
    let r_a_size: i128 = r_a_size.into();
    let r_b_offset: i128 = r_b_offset.into();
    let page_size: i128 = page_size.into();

    let r_a_end = r_a_offset + r_a_size - 1;
    let r_a_end_page = r_a_end & !(page_size - 1);
    let r_b_start = r_b_offset;
    let r_b_start_page = r_b_start & !(page_size - 1);
    r_a_end_page == r_b_start_page
}
pub fn aling(number: vk::DeviceSize, alignment: vk::DeviceSize) -> vk::DeviceSize {
    (number) + ((alignment) - 1) & !((alignment) - 1)
}
#[derive(Debug)]
pub struct Block {
    pub offset: vk::DeviceSize,
    pub size: vk::DeviceSize,
    pub buffer_size: vk::DeviceSize,
    pub left_padding: vk::DeviceSize,
    pub right_padding: vk::DeviceSize,
    pub allocation_type: AllocationType,
    pub id: usize,
}
impl Block {
    pub fn new(
        id: usize,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
        buffer_size: vk::DeviceSize,
        allocation_type: AllocationType,
        left_padding: vk::DeviceSize,
        right_padding: vk::DeviceSize,
    ) -> Self {
        Self {
            offset,
            size,
            buffer_size,
            allocation_type,
            right_padding,
            left_padding,
            id,
        }
    }
    pub fn is_free(&self) -> bool {
        self.allocation_type == AllocationType::Free
    }
}
impl Drop for Block {
    fn drop(&mut self) {
        println!("Block dropped");
    }
}
pub struct Allocator {
    core: Rc<Core>,
    pub handle: vk::DeviceMemory,
    size: vk::DeviceSize,
    pub free_memory: vk::DeviceSize,
    pub memory_type_index: u32,
    pub memory_flags: vk::MemoryPropertyFlags,
    pub list: List<Block>,
    pub last_block_id: usize,
    pub data: Option<*mut c_void>,
}

impl Allocator {
    pub fn new(
        core: Rc<Core>,
        allocation_size: vk::DeviceSize,
        memory_flags: vk::MemoryPropertyFlags,
        memory_type_index: u32,
    ) -> Self {
        let create_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(allocation_size)
            .memory_type_index(memory_type_index)
            .build();
        let handle = unsafe {
            core.logical_device
                .allocate_memory(&create_info, None)
                .expect("Failed to allocate memory")
        };
        Self {
            core,
            handle,
            size: allocation_size,
            memory_type_index,
            memory_flags: memory_flags,
            free_memory: allocation_size,
            last_block_id: 1,
            list: List::new(Block::new(
                0,
                0,
                allocation_size,
                0,
                AllocationType::Free,
                0,
                0,
            )),
            data: None,
        }
    }
    pub fn allocate_memory(
        &mut self,
        memory_requirements: vk::MemoryRequirements,
        allocation_type: AllocationType,
        buffer_index: usize,
        granularity: vk::DeviceSize,
    )-> (usize,u64){
        let mut previous: Link<Block> = None;
        let mut aligned_offset = 0;
        let mut left_padding: vk::DeviceSize = 0;
        let mut right_padding: vk::DeviceSize = 0;
        let mut best_fit: Link<Block> = None;
        for node in self.list.iter() {
            let block = &node.borrow().data;
            let size = memory_requirements.size;
            aligned_offset = aling(block.offset, memory_requirements.alignment);
            left_padding = aligned_offset - block.offset;
            if block.is_free() && block.size >= size && block.size >= left_padding + size {
                match previous {
                    Some(previous) => {
                        let previous = &previous.borrow().data;
                        if is_on_same_page(
                            previous.offset + previous.left_padding,
                            previous.size - previous.right_padding,
                            aligned_offset,
                            granularity,
                        ) && has_granularity_conflict(previous.allocation_type, allocation_type)
                        {
                            aligned_offset = aling(aligned_offset, granularity);
                            left_padding = aligned_offset - block.offset;
                        }
                    }
                    None => {}
                }
                match node.borrow().next.clone() {
                    Some(next) => {
                        if is_on_same_page(
                            aligned_offset,
                            size,
                            next.borrow().data.offset,
                            granularity,
                        ) && has_granularity_conflict(
                            allocation_type,
                            next.borrow().data.allocation_type,
                        ) {
                            right_padding = granularity;
                        }
                    }
                    None => {}
                }
                if block.size >= left_padding + size + right_padding {
                    best_fit = Some(node.clone());
                    break;
                }
            }
            previous = Some(node.clone());

        }
        let size = left_padding + memory_requirements.size + right_padding;
        if let Some(ref node) = best_fit {
            let mut mut_node = node.borrow_mut();
            let block = &mut_node.data;
            println!("Allocating memory for buffer {}", aligned_offset);
            if block.size > size {
                let new_block = Block::new(
                    self.last_block_id,
                    block.offset + size,
                    block.size - size,
                    0,
                    AllocationType::Free,
                    0,
                    0,
                );
                self.last_block_id += 1;
                let new_node = Node {
                    data: new_block,
                    next: mut_node.next.clone(),
                };
                mut_node.next = Some(Rc::new(RefCell::new(new_node)));
            } else {
            }
            let block = &mut mut_node.data;
            block.size = size;
            block.allocation_type = allocation_type;
            block.left_padding = left_padding;
            block.right_padding = right_padding;
            block.buffer_size = memory_requirements.size;
        } else {
            panic!("Failed to allocate memory");
        }
        self.free_memory -= size;
        (best_fit.unwrap().borrow().data.id,aligned_offset)
    }
    pub fn free_memory(&mut self, block_id: usize) {
        let mut pre: Option<Rc<RefCell<Node<Block>>>> = None;
        self.list.iter().for_each(|node| {
            let mut mut_node = node.borrow_mut();
            let block = &mut mut_node.data;
            if !block.is_free(){
                if block_id == block.id {
                    block.allocation_type = AllocationType::Free;
                    block.buffer_size = 0;
                    block.right_padding = 0;
                    block.left_padding = 0;
                    self.free_memory += block.size;
                } else {
                    pre = None;
                    return ;
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

    pub fn map_memory(&mut self) {
        if self
            .memory_flags
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            self.data = unsafe {
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
