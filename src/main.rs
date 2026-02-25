use std::{f32::consts::PI, time::Instant};

use sdl3::event::{Event,};

use crate::math::vector2::Vector2;

mod math;
mod render;



fn main() {
    println!("Hello, can I have your name?");

    let sdl = sdl3::init().unwrap();

    let video_system = sdl.video().unwrap();
    
    let window_size: Vector2<u32> = Vector2::new(1280, 720);

    let window = video_system.window("Faelight", window_size.x, window_size.y)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| 
        match video_system.gl_get_proc_address(s) {
            Some(f) => f as *const std::os::raw::c_void,
            None => std::ptr::null()
        }
    );

    unsafe{ 
        gl::ClearColor(0.15, 0.15, 0.16, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let start_time = Instant::now();

    'main: loop{
        
        for event in event_pump.poll_iter() {
            match handle_window_event(&event) {
                EventResponse::Quit => break 'main,
                _ => ()
            }
        }

        //OpenGL
        let elapsed = start_time.elapsed().as_secs_f32();
        unsafe {
            //gl::ClearColor(
            //    (PI*0.5+elapsed.sin()+1.0)*0.5, 
            //    (PI+elapsed.sin()+1.0)*0.5, 
            //    (elapsed.sin()+1.0)*0.5
            //, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }


}

enum EventResponse{
    None,
    Quit
}

fn handle_window_event(event: &Event) -> EventResponse{
     match event {
                //WindowEvent::CursorPos(x, y) => {println!("x: {x} y: {y}")}
               Event::Quit { .. } => {EventResponse::Quit}
                _ =>EventResponse::None
            }
}
