use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

struct VulkanApp {
    event_loop: EventLoop<()>,
    window: winit::window::Window,
}

impl VulkanApp {
    fn new() -> VulkanApp {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Triangle Foobar")
            .build(&event_loop)
            .unwrap();
        VulkanApp { event_loop, window }
    }

    fn run(self) {
        let window = self.window;
        let event_loop = self.event_loop;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => {
                        println!("input.virtual_keycode={:?}", input.virtual_keycode);
                        if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                            if input.state == ElementState::Pressed {
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
