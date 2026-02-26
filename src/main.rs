use std::{f32::consts::PI, time::Instant};

use gl::DrawElements;
use sdl3::{event::{Event, WindowEvent}, keyboard::Keycode, sys::keycode};

use crate::{input::keyboard::KeyboardState, math::vector2::{Vector2f, Vector2u}, render::render_objects::{Ibo, Vao, Vbo, create_program}};

mod math;
mod render;
mod input;


fn main() {
    println!("Hello, can I have your name?");

    let sdl = sdl3::init().unwrap();

    let video_system = sdl.video().unwrap();
    
    let window_size = Vector2u::new(1280, 720);

    let window = video_system
        .window("Faelight", window_size.x, window_size.y)
        .resizable()
        .opengl()
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

    //let mut verts: Vec<f32> = vec![
    //    -0.5, -0.5,
    //    0.5, -0.5,
    //    0.5, 0.5,
    //    -0.5, 0.5
    //];

    let mut verts: Vec<Vector2f> = vec![
       Vector2f::new( -0.5, -0.5),
       Vector2f::new( 0.5,  -0.5),
       Vector2f::new( 0.5,   0.5),
       Vector2f::new( -0.5,  0.5)
    ];

    let indices: Vec<u32> = vec![
        0, 1, 2,
        2, 3, 0
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

    let mut delta_timer = Instant::now();

    let mut start_time = Instant::now();

    let mut input = KeyboardState::new();


    'main: loop{
        let delta_time = delta_timer.elapsed().as_secs_f32();
        println!("Deltatime: {delta_time}");
        delta_timer = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                //WindowEvent::CursorPos(x, y) => {println!("x: {x} y: {y}")}
                Event::KeyDown { keycode, .. } =>{ keycode.map(|key| input.regPress(key)); },
                Event::KeyUp { keycode, .. } =>{ keycode.map(|key| input.regRelease(key)); },
                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(width, height) = win_event {
                        unsafe { gl::Viewport(0, 0, width, height) }
                    }
                },
                Event::Quit { .. } => break 'main,
                _ =>()
            }
        }

        //OpenGL

        vbo.set(&verts);

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

