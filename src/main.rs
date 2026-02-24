use std::{thread, time::Duration};



fn main() {
    println!("Hello, can I have your name?");
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("Glfw failed to init.");

    let (mut window, window_events) = 
    glfw.create_window(1280, 720, "Faelight", glfw::WindowMode::Windowed)
    .expect("Window creation failed.");
    window.set_key_polling(true);

    while !window.should_close() {
        glfw.poll_events();
        
        for(_, event) in glfw::flush_messages(&window_events){
            
            match event {
                match
            }

        }
    }
}
