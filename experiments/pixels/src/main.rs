#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use rand::prelude::*;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

struct Line {
    p1: (i32, i32),
    p2: (i32, i32),
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame_mut());
            if let Err(err) = pixels.render() {
                error!("pixels.render() failed: {err}");
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    error!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        let mut rng = rand::thread_rng();

        // clear buffer
        for (pixel) in frame.chunks_exact_mut(4) {
            let rgba = [0x35, 0x35, 0x35, 0xff];
            pixel.copy_from_slice(&rgba);
        }

        //  let x = (i % WIDTH as usize) as i16;
        //  let y = (i / WIDTH as usize) as i16;

        let x1: f32 = rng.gen::<f32>() * (WIDTH as f32);
        let y1: f32 = rng.gen::<f32>() * HEIGHT as f32;
        let x2: f32 = rng.gen::<f32>() * WIDTH as f32;
        let y2: f32 = rng.gen::<f32>() * HEIGHT as f32;

        // draw a line
        let line = Line {
            p1: (x1 as i32, y1 as i32),
            p2: (x2 as i32, y2 as i32),
        };

        self.draw_line(frame, line);
    }

    fn draw_line(&self, frame: &mut [u8], line: Line) {
        for (x, y) in line_drawing::Bresenham::new(line.p1, line.p2) {
            let pixelnum = (y * WIDTH as i32 + x) * 4;
            frame[pixelnum as usize] = 0x00;
            frame[(pixelnum + 1) as usize] = 0xff;
            frame[(pixelnum + 2) as usize] = 0x00;
            frame[(pixelnum + 3) as usize] = 0xff;
        }
    }
}
