pub struct Image{
    
}

// let depth_format = self.core.find_supported_format(
//     vec![
//         vk::Format::D32_SFLOAT,
//         vk::Format::D32_SFLOAT_S8_UINT,
//         vk::Format::D24_UNORM_S8_UINT,
//     ],
//     vk::ImageTiling::OPTIMAL,
//     vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
// );
// let extent = &self.swap_chain_extent;
// for _i in 0..self.images.len() {
//     let image_create_info = vk::ImageCreateInfo::builder()
//         .image_type(vk::ImageType::TYPE_2D)
//         .extent(vk::Extent3D {
//             width: extent.width,
//             height: extent.height,
//             depth: 1,
//         })
//         .mip_levels(1)
//         .array_layers(1)
//         .format(depth_format)
//         .tiling(vk::ImageTiling::OPTIMAL)
//         .initial_layout(vk::ImageLayout::UNDEFINED)
//         .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
//         .sharing_mode(vk::SharingMode::EXCLUSIVE)
//         .samples(vk::SampleCountFlags::TYPE_1)
//         .build();
//     let image = unsafe {
//         self.core
//             .logical_device
//             .create_image(&image_create_info, None)
//             .expect("Failed to create image")
//     };
//     let mem_req: vk::MemoryRequirements = unsafe {
//         self.core
//             .logical_device
//             .get_image_memory_requirements(image)
//     };
//     let mem_index = self.core.find_memory_type(
//         mem_req.memory_type_bits,
//         vk::MemoryPropertyFlags::DEVICE_LOCAL,
//     );
//     let alloc_info = vk::MemoryAllocateInfo::builder()
//         .allocation_size(mem_req.size)
//         .memory_type_index(mem_index.unwrap())
//         .build();
//     let image_memory = unsafe {
//         self.core
//             .logical_device
//             .allocate_memory(&alloc_info, None)
//             .expect("Failed to allocate image memory")
//     };
//     unsafe {
//         self.core
//             .logical_device
//             .bind_image_memory(image, image_memory, 0)
//             .expect("Failed to bind")
//     };
//     self.depth_images.push(image);
//     self.depth_image_memories.push(image_memory);
//     let view_info = vk::ImageViewCreateInfo::builder()
//         .image(image)
//         .view_type(vk::ImageViewType::TYPE_2D)
//         .format(depth_format)
//         .subresource_range(vk::ImageSubresourceRange {
//             aspect_mask: vk::ImageAspectFlags::DEPTH,
//             base_mip_level: 0,
//             level_count: 1,
//             base_array_layer: 0,
//             layer_count: 1,
//         })
//         .build();
//     let depth_image_view = unsafe {
//         self.core
//             .logical_device
//             .create_image_view(&view_info, None)
//             .expect("Failed to create image view")
//     };
//     self.depth_image_views.push(depth_image_view);
// }