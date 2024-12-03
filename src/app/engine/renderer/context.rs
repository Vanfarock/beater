use std::sync::Arc;

use anyhow::Result;
use ash::vk;
use ash_window;
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

pub struct Context {
    entry: ash::Entry,
    instance: ash::Instance,
    surface_extension: ash::khr::surface::Instance,
    surface: vk::SurfaceKHR,
    physical_devices: Vec<PhysicalDevice>,
}

#[derive(Debug, Clone)]
pub struct QueueFamily {
    index: usize,
    properties: vk::QueueFamilyProperties,
}

#[derive(Debug, Clone)]
pub struct PhysicalDevice {
    handle: vk::PhysicalDevice,
    properties: vk::PhysicalDeviceProperties,
    features: vk::PhysicalDeviceFeatures,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    queue_families: Vec<QueueFamily>,
}

impl Context {
    pub fn new(window: Arc<Window>) -> Result<Self> {
        let allocation_callbacks = None;
        let display_handle = window.display_handle()?.as_raw();
        let window_handle = window.window_handle()?.as_raw();

        let entry = unsafe { ash::Entry::load()? };
        let instance = {
            let application_info = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_3);

            let create_info = vk::InstanceCreateInfo::default()
                .application_info(&application_info)
                .enabled_extension_names(ash_window::enumerate_required_extensions(
                    display_handle,
                )?);
            unsafe { entry.create_instance(&create_info, allocation_callbacks) }
        }?;

        let surface_extension = ash::khr::surface::Instance::new(&entry, &instance);
        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                display_handle,
                window_handle,
                allocation_callbacks,
            )
        }?;

        let physical_devices = unsafe { instance.enumerate_physical_devices() }?;

        let physical_devices = physical_devices
            .into_iter()
            .map(|handle| {
                let properties = unsafe { instance.get_physical_device_properties(handle) };
                let features = unsafe { instance.get_physical_device_features(handle) };
                let memory_properties =
                    unsafe { instance.get_physical_device_memory_properties(handle) };
                let queue_families =
                    unsafe { instance.get_physical_device_queue_family_properties(handle) }
                        .iter()
                        .enumerate()
                        .map(|(index, properties)| QueueFamily {
                            index,
                            properties: *properties,
                        })
                        .collect::<Vec<QueueFamily>>();

                PhysicalDevice {
                    handle,
                    properties,
                    features,
                    memory_properties,
                    queue_families,
                }
            })
            .collect::<Vec<PhysicalDevice>>();

        dbg!(physical_devices.clone());

        Ok(Self {
            entry,
            instance,
            surface_extension,
            surface,
            physical_devices,
        })
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        let allocation_callbacks = None;

        unsafe {
            self.surface_extension
                .destroy_surface(self.surface, allocation_callbacks)
        };
        unsafe {
            self.instance.destroy_instance(allocation_callbacks);
        }
    }
}
