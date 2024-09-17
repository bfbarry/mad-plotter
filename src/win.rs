use crate::state::State;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder
};

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(&window).await;
    let mut surface_configured = false;
    // `let` to hide unused Result warning
    let _ = event_loop.run(move |event, control_flow| match event {
        // if things look confusing we are using same variable names for different types (e.g., event)
        Event::WindowEvent {  //NOTE:extracting the fields of the variant to pass on to next match...
            ref event,
            window_id
        } if window_id == state.window().id() => if !state.input(event) {
            match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event: KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                        },
                    ..
                } => control_flow.exit(),
                WindowEvent::Resized(physical_size) => {
                    surface_configured = true;
                    state.resize(*physical_size);
                },
                WindowEvent::RedrawRequested => {
                    // tell winit we want another frame
                    state.window().request_redraw();

                    if !surface_configured {
                        return;
                    }

                    state.update();

                    match state.render() {
                        Ok(_) => {}
                        //reconfigure if surface is lost or outdated
                        Err(
                            wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated
                        ) => state.resize(state.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            control_flow.exit()
                        }
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                            }
                        },
                _ => {}
            }
        }
        _ => {}
    });

}


