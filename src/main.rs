use std::{f32::consts::PI, time::Instant};

use gl::DrawElements;
use sdl3::{event::Event, keyboard::Keycode, sys::keycode};

use crate::{math::vector2::Vector2, render::render_objects::{Ibo, Vao, Vbo, create_program}};

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

    let program = create_program().unwrap();
    program.set();

    let mut verts: Vec<f32> = vec![
        -0.5, -0.5,
        0.0, -0.5,
        0.5, 0.5
    ];

    let indices: Vec<u32> = vec![
        0, 1, 2
    ];

    let vbo = Vbo::generate();
    vbo.set(&verts);

    let vao = Vao::generate();
    vao.set();

    let ibo = Ibo::generate();
    ibo.set(&indices);

    unsafe{ 
        gl::ClearColor(0.15, 0.15, 0.16, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();

    let start_time = Instant::now();

    'main: loop{
        
        for event in event_pump.poll_iter() {
            match event {
                //WindowEvent::CursorPos(x, y) => {println!("x: {x} y: {y}")}
                Event::KeyDown { timestamp, window_id, keycode, scancode, keymod, repeat, which, raw } =>{
                    let key= 
                    match keycode {
                        Some(key) => key,
                        None => continue    
                    };
                    println!("Key pressed: {key}");
                    match key {
                        Keycode::Left => verts[4]  -= 0.01,
                        Keycode::Right => verts[4] += 0.01,
                        Keycode::Down => verts[5]  -= 0.01,
                        Keycode::Up => verts[5]    += 0.01,
                        _ => continue
                    }
                    vbo.set(&verts);


                }
               Event::Quit { .. } => break 'main,
                _ =>()
            }
        }

        //OpenGL
        let elapsed = start_time.elapsed().as_secs_f32() * 0.5;
        unsafe {
            gl::ClearColor(
                (((elapsed - 0.000) * PI).sin()+1.0)*0.5, 
                (((elapsed - 0.666) * PI).sin()+1.0)*0.5, 
                (((elapsed + 0.666) * PI).sin()+1.0)*0.5,
                1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const _
            )
        }

        window.gl_swap_window();
    }


}

