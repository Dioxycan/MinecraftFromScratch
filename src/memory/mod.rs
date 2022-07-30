pub mod allocators;
mod buffers;
mod image;
use self::allocators::Block;
use crate::{core::Core, utils::list::Link};
use allocators::Allocator;
use ash::vk;
use buffers::Buffer;
use image::Image;
use std::ffi::c_void;
use std::panic;
use std::{rc::Rc, sync::Arc};
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum AllocationType {
    Free,
    Buffer,
    Image,
    ImageLinear,
    ImageOptimal,
}
#[derive(Debug)]
struct MemoryHeap {
    size: vk::DeviceSize,
    free_memory: vk::DeviceSize,
    property_flags: vk::MemoryPropertyFlags,
    memory_types: Vec<MemoryType>,
}
#[derive(Debug)]
struct MemoryType {
    property_flags: vk::MemoryPropertyFlags,
    type_index: usize,
}
fn get_memory_heaps(core: &Core) -> Vec<MemoryHeap> {
    let memory_properties = unsafe {
        core.instance
            .get_physical_device_memory_properties(core.physical_device)
    };
    let mut memory_heaps: Vec<MemoryHeap> = memory_properties.memory_heaps
        [0..memory_properties.memory_heap_count as usize]
        .iter()
        .map(|heap| MemoryHeap {
            size: heap.size,
            free_memory: heap.size,
            property_flags: vk::MemoryPropertyFlags::empty(),
            memory_types: vec![],
        })
        .collect();
    memory_properties.memory_types[0..memory_properties.memory_type_count as usize]
        .iter()
        .enumerate()
        .for_each(|(i, memory_type)| {
            let memory_heap = &mut memory_heaps[memory_type.heap_index as usize];
            memory_heap.memory_types.push(MemoryType {
                property_flags: memory_type.property_flags,
                type_index: i,
            });
            memory_heap.property_flags |= memory_type.property_flags;
        });
    memory_heaps
}

pub struct Memory {
    core: Rc<Core>,
    pub allocators: Vec<Allocator>,
    pub buffers: Vec<Buffer>,
    pub images: Vec<Image>,
    memory_heaps: Vec<MemoryHeap>,
    granularity: vk::DeviceSize,
}
impl Memory {
    pub fn new(core: Rc<Core>) -> Self {
        Self {
            memory_heaps: get_memory_heaps(&core),
            allocators: Vec::new(),
            buffers: Vec::new(),
            images: Vec::new(),
            granularity: unsafe {
                core.instance
                    .get_physical_device_properties(core.physical_device)
                    .limits
                    .buffer_image_granularity
            },
            core,
        }
    }
    pub fn create_allocator(
        &mut self,
        size: vk::DeviceSize,
        memory_property_flags: vk::MemoryPropertyFlags,
        alignment: vk::DeviceSize,
    ) {
        let type_index = self.find_suitable_heap(size, memory_property_flags);
        match type_index {
            Some(type_index) => {
                let mut allocator = Allocator::new(
                    self.core.clone(),
                    size,
                    memory_property_flags,
                    type_index as u32,
                );
                if memory_property_flags.contains(vk::MemoryPropertyFlags::HOST_VISIBLE) {
                    allocator.map_memory();
                }
                self.allocators.push(allocator);
            }
            None => {
                eprintln!("Failed to find suitable heap");
                return;
            }
        }
    }
    pub fn create_buffer(
        &mut self,
        size: vk::DeviceSize,
        allocation_type: AllocationType,
        usage_flags: vk::BufferUsageFlags,
        memory_flags: vk::MemoryPropertyFlags,
    ) -> usize {
        let mut buffer = Buffer::new(
            self.core.clone(),
            vk::BufferCreateInfo {
                size,
                usage: usage_flags,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                ..Default::default()
            },
            memory_flags,            
        );
        let (data, allocator_index, block_id,offset) = self.allocate_memory(
            buffer.memory_requirements,
            memory_flags,
            allocation_type,
            self.granularity,
        );

        buffer.offsets.push(offset);
        buffer.data = data;
        buffer.allocator_id = Some(allocator_index);
        buffer.block_id = Some(block_id);
        unsafe {
            self.core
                .logical_device
                .bind_buffer_memory(
                    buffer.handle,
                    self.allocators[allocator_index].handle,
                    offset,
                )
                .unwrap();
        }
      
        self.buffers.push(buffer);
        self.buffers.len() - 1
    }

    fn allocate_memory(
        &mut self,
        memory_requirements: vk::MemoryRequirements,
        memory_type: vk::MemoryPropertyFlags,
        allocation_type: AllocationType,
        granuality: vk::DeviceSize,
    ) -> (Option<*mut c_void>, usize, usize,u64) {
        let allocator_index = self
            .find_suitable_allocator(
                memory_type,
                memory_requirements.size,
                memory_requirements.memory_type_bits,
            )
            .unwrap();
        let allocator = &mut self.allocators[allocator_index];
        let (block_id,offset) = allocator.allocate_memory(memory_requirements, allocation_type, 0, granuality);
        let data = match allocator.data {
            Some(ref mut data) => Some(unsafe { data.offset(offset as _) }),
            None => None,
        };
        (data, allocator_index, block_id,offset)
    }
    fn find_suitable_allocator(
        &mut self,
        memory_type: vk::MemoryPropertyFlags,
        size: vk::DeviceSize,
        memory_type_index: u32,
    ) -> Option<usize> {
        self.allocators
            .iter_mut()
            .enumerate()
            .find(|(index, allocator)| {
                allocator.memory_flags.contains(memory_type)
                    && allocator.free_memory >= size
                    && memory_type_index & (1 << index) != 0
            })
            .map(|(index, allocator)| index)
    }
    pub fn copy_memory(
        &mut self,
        command_buffer: Option<vk::CommandBuffer>,
        buffer_index: usize,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
        data: *const u8,
    ) {
        match self.buffers[buffer_index].memory_type {
            vk::MemoryPropertyFlags::HOST_VISIBLE => match self.buffers[buffer_index].data {
                Some(host_data_ptr) => unsafe {
                    std::ptr::copy(
                        data,
                        host_data_ptr.offset(offset as isize) as *mut u8,
                        size as usize,
                    );
                },
                None => {
                    eprintln!("Failed to find host data ptr");
                }
            },
            vk::MemoryPropertyFlags::DEVICE_LOCAL => {
                let staging_buffer_index = self.find_suitable_buffer(
                    size,
                    vk::BufferUsageFlags::TRANSFER_SRC,
                    vk::MemoryPropertyFlags::HOST_VISIBLE,
                );
                match staging_buffer_index {
                    Some(staging_buffer_index) => {
                        let sub_memory =
                            self.buffers[staging_buffer_index].allocate_sub_memory(size);
                        match sub_memory {
                            Some(sub_memory) => {
                                let sub_memory = &sub_memory.borrow().data;
                                self.copy_memory(None, buffer_index, sub_memory.offset, size, data);
                                match command_buffer {
                                    Some(command_buffer) => {
                                        let copy_info = vk::BufferCopy {
                                            src_offset: sub_memory.offset,
                                            dst_offset: offset,
                                            size,
                                        };
                                        let copy_infos = [copy_info];
                                        unsafe {
                                            self.core.logical_device.cmd_copy_buffer(
                                                command_buffer,
                                                self.buffers[buffer_index].handle,
                                                self.buffers[staging_buffer_index].handle,
                                                &copy_infos,
                                            );
                                            self.buffers[staging_buffer_index]
                                                .free_sub_memory(sub_memory.id);
                                        }
                                    }
                                    None => {
                                        eprintln!("Failed to find command buffer");
                                    }
                                }
                            }
                            None => {}
                        }
                        panic!("Failed to find sub memory");
                    }
                    None => {
                        eprintln!("Failed to find suitable staging buffer");
                    }
                };
            }
            _ => {}
        }
    }
    fn find_suitable_buffer(
        &mut self,
        size: vk::DeviceSize,
        usage_flags: vk::BufferUsageFlags,
        memory_flags: vk::MemoryPropertyFlags,
    ) -> Option<usize> {
        self.buffers
            .iter_mut()
            .enumerate()
            .find(|(index, buffer)| {
                buffer.size >= size
                    && buffer.memory_type.contains(memory_flags)
                    && buffer.buffer_usage.contains(usage_flags)
            })
            .map(|(index, buffer)| index)
    }
    fn find_suitable_heap(
        &mut self,
        size: vk::DeviceSize,
        memory_property_flags: vk::MemoryPropertyFlags,
    ) -> Option<usize> {
        for (index, heap) in self.memory_heaps.iter_mut().enumerate() {
            if heap.property_flags.contains(memory_property_flags) && heap.free_memory >= size {
                heap.free_memory -= size;
                return heap
                    .memory_types
                    .iter()
                    .find(|memory_type| memory_type.property_flags.contains(memory_property_flags))
                    .map(|memory_type| memory_type.type_index);
            }
        }
        None
    }
    pub fn free_buffer(&mut self, buffer_index: usize) {
        let buffer = &mut self.buffers[buffer_index];
        let allocator = &mut self.allocators[buffer.allocator_id.unwrap()];
        let offset = buffer.offsets.pop().unwrap();
        allocator.free_memory(buffer.block_id.unwrap());
        self.buffers.remove(buffer_index);
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        for allocator in self.allocators.iter_mut() {
            unsafe {
                self.core.logical_device.free_memory(allocator.handle, None);
            }
        }
    }
}
// impl Memory{
//     pub fn new(core:Rc<Core>)->Self{
//         Self{
//             core,
//             allocator,
//         }
//     }
// }
// // // fn create_vertex_buffer() {
// // //     let vertices = vec![

// // //         Vertex::new(glm::Vec3::new(0.25, -0.25,0.5), glm::Vec3::new(1.0, 0.0, 0.0)),
// // //         Vertex::new(glm::Vec3::new(0.75, 0.75,0.5), glm::Vec3::new(1.0, 0.0, 0.0)),
// // //         Vertex::new(glm::Vec3::new(-0.25, 0.75,0.5), glm::Vec3::new(1.0, 0.0, 0.0)),""

// // //     ];

// // //     let vertex_count = mem::size_of::<Vertex>() as u64;
// // //     let vertex_buffer_info = vk::BufferCreateInfo::builder()
// // //         .size(vertex_count * vertices.len() as u64)
// // //         .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
// // //         .sharing_mode(vk::SharingMode::EXCLUSIVE)
// // //         .build();
// // //     let vertex_buffer = unsafe {
// // //         self.core
// // //             .logical_device
// // //             .create_buffer(&vertex_buffer_info, None)
// // //             .expect("Failed to create vertex buffer")
// // //     };
// // //     let vertex_buffer_memory_requirements = unsafe {
// // //         self.core
// // //             .logical_device
// // //             .get_buffer_memory_requirements(self.vertex_buffer)
// // //     };
// // //     let vertex_buffer_memory_allocate_info = vk::MemoryAllocateInfo::builder()
// // //         .allocation_size(vertex_buffer_memory_requirements.size)
// // //         .memory_type_index(
// // //             self.core
// // //                 .find_memory_type(
// // //                     vertex_buffer_memory_requirements.memory_type_bits,
// // //                     vk::MemoryPropertyFlags::HOST_VISIBLE
// // //                         | vk::MemoryPropertyFlags::HOST_COHERENT,
// // //                 )
// // //                 .unwrap(),
// // //         )
// // //         .build();
// // //     self.vertex_buffer_memory = unsafe {
// // //         self.core
// // //             .logical_device
// // //             .allocate_memory(&vertex_buffer_memory_allocate_info, None)
// // //             .expect("Failed to allocate vertex buffer memory")
// // //     };
// // //     unsafe {
// // //         self.core
// // //             .logical_device
// // //             .bind_buffer_memory(self.vertex_buffer, self.vertex_buffer_memory, 0)
// // //             .expect("Failed to bind vertex buffer memory")
// // //     };
// // //     let mapped_memory = unsafe {
// // //         self.core
// // //             .logical_device
// // //             .map_memory(
// // //                 self.vertex_buffer_memory,
// // //                 0,
// // //                 vertex_buffer_memory_requirements.size,
// // //                 vk::MemoryMapFlags::empty(),
// // //             )
// // //             .expect("Failed to map vertex buffer memory")
// // //     };
// // //     unsafe {
// // //         std::ptr::copy_nonoverlapping(
// // //             vertices.as_ptr() as *const u8,
// // //             mapped_memory as *mut u8,
// // //             vertices.len() * mem::size_of::<Vertex>(),
// // //         );
// // //         self.core
// // //             .logical_device
// // //             .unmap_memory(self.vertex_buffer_memory);
// // //     }
// // // }
// // // pub fn draw(&mut self, command_buffer: vk::CommandBuffer) {
// // //     unsafe {
// // //         self.core
// // //             .logical_device
// // //             .cmd_draw(command_buffer, self.vertex_count as u32, 1, 0, 0);
// // //     }
// // // }
