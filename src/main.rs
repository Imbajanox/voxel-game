use voxel_game::{Camera, Renderer};
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};
use std::sync::Arc;

struct GameState {
    camera: Camera,
    movement_speed: f32,
    rotation_speed: f32,
    last_frame_time: std::time::Instant,
}

impl GameState {
    fn new(aspect_ratio: f32) -> Self {
        Self {
            camera: Camera::new(aspect_ratio),
            movement_speed: 10.0,
            rotation_speed: 0.002,
            last_frame_time: std::time::Instant::now(),
        }
    }

    fn update(&mut self) -> f32 {
        let current_time = std::time::Instant::now();
        let delta_time = current_time.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = current_time;
        delta_time
    }
}

fn main() {
    env_logger::init();
    
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new()
        .with_title("Voxel Game Engine")
        .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
        .build(&event_loop)
        .unwrap());

    let mut renderer = pollster::block_on(Renderer::new(window.clone()));
    let size = renderer.size();
    let aspect_ratio = size.width as f32 / size.height as f32;
    let mut game_state = GameState::new(aspect_ratio);

    let mut keys_pressed = std::collections::HashSet::new();

    event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => control_flow.exit(),
                WindowEvent::Resized(physical_size) => {
                    renderer.resize(*physical_size);
                    game_state.camera.update_aspect(
                        physical_size.width as f32 / physical_size.height as f32
                    );
                }
                WindowEvent::KeyboardInput {
                    event: KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state,
                        ..
                    },
                    ..
                } => {
                    match state {
                        ElementState::Pressed => {
                            keys_pressed.insert(*key);
                            // Handle escape key to close window
                            if *key == KeyCode::Escape {
                                control_flow.exit();
                            }
                        }
                        ElementState::Released => {
                            keys_pressed.remove(key);
                        }
                    }
                }
                WindowEvent::RedrawRequested => {
                    // Update game state
                    let delta_time = game_state.update();
                    
                    // Handle continuous movement
                    let speed = game_state.movement_speed * delta_time;
                    if keys_pressed.contains(&KeyCode::KeyW) {
                        game_state.camera.move_forward(speed);
                    }
                    if keys_pressed.contains(&KeyCode::KeyS) {
                        game_state.camera.move_backward(speed);
                    }
                    if keys_pressed.contains(&KeyCode::KeyA) {
                        game_state.camera.move_left(speed);
                    }
                    if keys_pressed.contains(&KeyCode::KeyD) {
                        game_state.camera.move_right(speed);
                    }
                    if keys_pressed.contains(&KeyCode::Space) {
                        game_state.camera.move_up(speed);
                    }
                    if keys_pressed.contains(&KeyCode::ShiftLeft) {
                        game_state.camera.move_down(speed);
                    }
                    
                    // Handle rotation
                    let rot_speed = game_state.rotation_speed * 50.0 * delta_time;
                    if keys_pressed.contains(&KeyCode::ArrowLeft) {
                        game_state.camera.rotate(-rot_speed, 0.0);
                    }
                    if keys_pressed.contains(&KeyCode::ArrowRight) {
                        game_state.camera.rotate(rot_speed, 0.0);
                    }
                    if keys_pressed.contains(&KeyCode::ArrowUp) {
                        game_state.camera.rotate(0.0, rot_speed);
                    }
                    if keys_pressed.contains(&KeyCode::ArrowDown) {
                        game_state.camera.rotate(0.0, -rot_speed);
                    }

                    // Update camera uniform
                    renderer.update_camera(&game_state.camera);

                    // Render
                    match renderer.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size()),
                        Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),
                        Err(e) => eprintln!("Render error: {:?}", e),
                    }
                }
                _ => {}
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}
