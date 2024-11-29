use anyhow::Result;
use ash::{self, vk};
use gpu_allocator::{
    vulkan::{AllocationCreateDesc, AllocationScheme, Allocator, AllocatorCreateDesc},
    MemoryLocation,
};
use std::time;

fn main() -> Result<()> {
    // Config
    let allocation_callbacks = None;
    let width = 4096;
    let height = 4096;
    let value_count = width * height;
    let alpha = 255;
    let red = 70;
    let green = 63;
    let blue = 158;
    let value = red | green << 8 | blue << 16 | alpha << 24;

    let entry = unsafe { ash::Entry::load() }?;
    let instance = {
        let application_info = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_3);
        let create_info = vk::InstanceCreateInfo::default().application_info(&application_info);
        unsafe { entry.create_instance(&create_info, allocation_callbacks) }?
    };

    let physical_device = unsafe { instance.enumerate_physical_devices() }?
        .into_iter()
        .next()
        .unwrap();
    let device = {
        let queue_priorities = [1.];
        let queue_create_info = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(0)
            .queue_priorities(&queue_priorities)];
        let create_info = vk::DeviceCreateInfo::default().queue_create_infos(&queue_create_info);
        unsafe { instance.create_device(physical_device, &create_info, allocation_callbacks) }
    }?;

    let queue = unsafe { device.get_device_queue(0, 0) };

    // Create buffer
    let buffer = {
        let create_info: vk::BufferCreateInfo<'_> = vk::BufferCreateInfo::default()
            .size(value_count * std::mem::size_of::<u32>() as vk::DeviceSize)
            .usage(vk::BufferUsageFlags::TRANSFER_DST);
        unsafe { device.create_buffer(&create_info, allocation_callbacks) }
    }?;

    // Create allocator
    let mut allocator = {
        let allocator_create_description = AllocatorCreateDesc {
            instance: instance.clone(),
            device: device.clone(),
            physical_device,
            debug_settings: Default::default(),
            buffer_device_address: false,
            allocation_sizes: Default::default(),
        };
        Allocator::new(&allocator_create_description)
    }?;
    let allocation = {
        let memory_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
        let allocation_create_description = AllocationCreateDesc {
            name: "Buffer allocation",
            requirements: memory_requirements,
            location: MemoryLocation::CpuToGpu,
            linear: true,
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        };
        let allocation = allocator.allocate(&allocation_create_description)?;
        unsafe {
            device
                .bind_buffer_memory(buffer, allocation.memory(), allocation.offset())
                .unwrap()
        };
        allocation
    };
    let command_pool = {
        let create_info = vk::CommandPoolCreateInfo::default().queue_family_index(0);
        unsafe { device.create_command_pool(&create_info, allocation_callbacks) }
    }?;
    let command_buffer = {
        let create_info = vk::CommandBufferAllocateInfo::default()
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_pool(command_pool)
            .command_buffer_count(1);
        unsafe { device.allocate_command_buffers(&create_info) }?
            .into_iter()
            .next()
            .unwrap()
    };

    // Recording command buffer
    {
        let begin_info = vk::CommandBufferBeginInfo::default();
        unsafe { device.begin_command_buffer(command_buffer, &begin_info) }?;
    };
    unsafe {
        device.cmd_fill_buffer(
            command_buffer,
            buffer,
            allocation.offset(),
            allocation.size(),
            value,
        )
    }
    {
        unsafe { device.end_command_buffer(command_buffer) }?;
    };

    // Execute command buffer by uploading it to the GPU through queue
    let fence = {
        let create_info = vk::FenceCreateInfo::default();
        unsafe { device.create_fence(&create_info, allocation_callbacks) }
    }?;
    {
        let binding = [command_buffer];
        let submits = [vk::SubmitInfo::default().command_buffers(&binding)];
        unsafe { device.queue_submit(queue, &submits, fence) }?;
    };

    // Wait for execution to complete
    let start = time::Instant::now();
    unsafe { device.wait_for_fences(&[fence], true, u64::MAX) }?;
    println!("GPU took {:?}", time::Instant::now().duration_since(start));

    // Read data
    let data = allocation.mapped_slice().unwrap();
    let start = time::Instant::now();
    image::save_buffer(
        "image.png",
        data,
        width as u32,
        height as u32,
        image::ColorType::Rgba8,
    )?;
    println!(
        "Saving image took {:?}",
        time::Instant::now().duration_since(start)
    );

    // Clean up
    allocator.free(allocation).unwrap();
    drop(allocator);
    unsafe { device.destroy_fence(fence, allocation_callbacks) };
    unsafe { device.destroy_command_pool(command_pool, allocation_callbacks) };
    unsafe { device.destroy_buffer(buffer, allocation_callbacks) };
    unsafe { device.destroy_device(allocation_callbacks) };
    unsafe { instance.destroy_instance(allocation_callbacks) };

    Ok(())
}
