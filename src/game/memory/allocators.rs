// pub fn create_memory_allocator(
//     core: &Core,
//     memory_type_bits: u32,
//     memory_property_flags: vk::MemoryPropertyFlags,
// ) -> vk::DeviceMemory {
//     let memory_requirements = unsafe {
//         core.logical_device.get_buffer_memory_requirements(vk::Buffer::null())
//     };
//     let memory_allocate_info = vk::MemoryAllocateInfo::builder()
//         .allocation_size(memory_requirements.size)
//         .memory_type_index(
//             core
//                 .find_memory_type(
//                     memory_requirements.memory_type_bits,
//                     memory_property_flags,
//                 )
//                 .unwrap(),
//         )
//         .build();
//     unsafe {
//         core.logical_device.allocate_memory(&memory_allocate_info, None)
//     }
// }