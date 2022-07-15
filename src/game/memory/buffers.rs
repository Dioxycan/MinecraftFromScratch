fn create_vertex_buffer() {
    let vertices = vec![
        
        Vertex::new(glm::Vec3::new(0.25, -0.25,0.5), glm::Vec3::new(1.0, 0.0, 0.0)),
        Vertex::new(glm::Vec3::new(0.75, 0.75,0.5), glm::Vec3::new(1.0, 0.0, 0.0)),
        Vertex::new(glm::Vec3::new(-0.25, 0.75,0.5), glm::Vec3::new(1.0, 0.0, 0.0)),""

    ];

    let vertex_count = mem::size_of::<Vertex>() as u64;
    let vertex_buffer_info = vk::BufferCreateInfo::builder()
        .size(vertex_count * vertices.len() as u64)
        .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .build();
    let vertex_buffer = unsafe {
        self.core
            .logical_device
            .create_buffer(&vertex_buffer_info, None)
            .expect("Failed to create vertex buffer")
    };
    let vertex_buffer_memory_requirements = unsafe {
        self.core
            .logical_device
            .get_buffer_memory_requirements(self.vertex_buffer)
    };
    let vertex_buffer_memory_allocate_info = vk::MemoryAllocateInfo::builder()
        .allocation_size(vertex_buffer_memory_requirements.size)
        .memory_type_index(
            self.core
                .find_memory_type(
                    vertex_buffer_memory_requirements.memory_type_bits,
                    vk::MemoryPropertyFlags::HOST_VISIBLE
                        | vk::MemoryPropertyFlags::HOST_COHERENT,
                )
                .unwrap(),
        )
        .build();
    self.vertex_buffer_memory = unsafe {
        self.core
            .logical_device
            .allocate_memory(&vertex_buffer_memory_allocate_info, None)
            .expect("Failed to allocate vertex buffer memory")
    };
    unsafe {
        self.core
            .logical_device
            .bind_buffer_memory(self.vertex_buffer, self.vertex_buffer_memory, 0)
            .expect("Failed to bind vertex buffer memory")
    };
    let mapped_memory = unsafe {
        self.core
            .logical_device
            .map_memory(
                self.vertex_buffer_memory,
                0,
                vertex_buffer_memory_requirements.size,
                vk::MemoryMapFlags::empty(),
            )
            .expect("Failed to map vertex buffer memory")
    };
    unsafe {
        std::ptr::copy_nonoverlapping(
            vertices.as_ptr() as *const u8,
            mapped_memory as *mut u8,
            vertices.len() * mem::size_of::<Vertex>(),
        );
        self.core
            .logical_device
            .unmap_memory(self.vertex_buffer_memory);
    }
}
pub fn draw(&mut self, command_buffer: vk::CommandBuffer) {
    unsafe {
        self.core
            .logical_device
            .cmd_draw(command_buffer, self.vertex_count as u32, 1, 0, 0);
    }
}