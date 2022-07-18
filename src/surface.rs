use ash::{extensions::khr, vk};

use crate::instance;

pub struct Surface {
    pub surface_loader: khr::Surface,
    pub surface: vk::SurfaceKHR,
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.surface_loader.destroy_surface(self.surface, None);
        }
    }
}

impl Surface {
    pub fn new(
        entry: &ash::Entry,
        instance: &instance::Instance,
        window: &winit::window::Window,
    ) -> Result<Self, vk::Result> {
        let surface_loader = khr::Surface::new(&entry, &instance);
        let surface = unsafe { ash_window::create_surface(&entry, &instance, &window, None)? };

        Ok(Self {
            surface_loader,
            surface,
        })
    }
}
