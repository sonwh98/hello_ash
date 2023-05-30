use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use ash::{vk, Entry, Instance};

#[cfg(any(target_os = "macos", target_os = "ios"))]
use ash::vk::{KhrGetPhysicalDeviceProperties2Fn, KhrPortabilityEnumerationFn};

use raw_window_handle::HasRawDisplayHandle;
use std::ffi::CStr;
use std::os::raw::c_char;

#[allow(dead_code)]
struct VulkanApp {
    event_loop: EventLoop<()>,
    window: winit::window::Window,
    entry: Entry,
    instance: Instance,
}

impl VulkanApp {
    fn new() -> VulkanApp {
        unsafe {
            let event_loop = EventLoop::new();
            let window = WindowBuilder::new()
                .with_title("Triangle Foobar")
                .build(&event_loop)
                .unwrap();
            let entry = Entry::linked();
            let app_name = CStr::from_bytes_with_nul_unchecked(b"VulkanTriangle\0");

            let appinfo = vk::ApplicationInfo::builder()
                .application_name(app_name)
                .application_version(0)
                .engine_name(app_name)
                .engine_version(0)
                .api_version(vk::make_api_version(0, 1, 0, 0));

            let layer_names = [CStr::from_bytes_with_nul_unchecked(
                b"VK_LAYER_KHRONOS_validation\0",
            )];
            let layers_names_raw: Vec<*const c_char> = layer_names
                .iter()
                .map(|raw_name| raw_name.as_ptr())
                .collect();
            let mut extension_names =
                ash_window::enumerate_required_extensions(window.raw_display_handle())
                    .unwrap()
                    .to_vec();

            #[cfg(any(target_os = "macos", target_os = "ios"))]
            {
                extension_names.push(KhrPortabilityEnumerationFn::name().as_ptr());
                // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
                extension_names.push(KhrGetPhysicalDeviceProperties2Fn::name().as_ptr());
            }

            let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
                vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
            } else {
                vk::InstanceCreateFlags::default()
            };

            let create_info = vk::InstanceCreateInfo::builder()
                .application_info(&appinfo)
                .enabled_layer_names(&layers_names_raw)
                .enabled_extension_names(&extension_names)
                .flags(create_flags);

            let instance: Instance = entry
                .create_instance(&create_info, None)
                .expect("Instance creation error");
            VulkanApp {
                event_loop,
                window,
                entry,
                instance,
            }
        }
    }

    fn run(self) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => {
                        println!("input.virtual_keycode={:?}", input.virtual_keycode);
                        if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                            if input.state == ElementState::Pressed {
                                *control_flow = ControlFlow::Exit;
                                println!("Escape key pressed!");
                            } else if input.state == ElementState::Released {
                                println!("Escape key released!");
                            }
                        }
                    }
                    WindowEvent::MouseInput { button, state, .. } => {
                        println!("mouseInput button={:?} state={:?}", button, state);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        println!("cursorMoved position={:?}", position);
                    }
                    x => println!("other {:?}", x),
                },
                _ => (),
            }
        });
    }
}

fn main() {
    let app = VulkanApp::new();
    app.run();
}
