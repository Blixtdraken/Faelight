use std::{cell::RefCell, collections::HashSet};

use sdl3::keyboard::Keycode;

use crate::{math::vector2::Vector2f, thread_cell};

#[derive(Default)]
struct KeyboardState{
    is_held:       HashSet<Keycode>,
    just_pressed:  HashSet<Keycode>,
    just_released: HashSet<Keycode>
}

impl KeyboardState{

    pub fn new()->Self{
        Self { is_held: HashSet::with_capacity(32), just_pressed: HashSet::with_capacity(8), just_released: HashSet::with_capacity(8) }
    }

    pub fn reg_press(&mut self, key: Keycode){
       self.just_pressed.insert(key);
       self.is_held.insert(key);
    }

    pub fn reg_release(&mut self, key: Keycode){
        self.just_released.insert(key);
        self.is_held.remove(&key);
    }

    pub fn frame_clear(&mut self){
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub fn is_held(&self, key: Keycode)->bool{
        self.is_held.contains(&key)
    }

    pub fn is_just_pressed(&self, key: Keycode)->bool{
        self.just_pressed.contains(&key)
    }
    pub fn is_just_released(&self, key: Keycode)->bool{
       self.just_released.contains(&key)
    }

   

}



thread_cell!(KEYBOARD, KeyboardState);

pub struct InputReader{}

impl InputReader{
    pub fn is_held(key: Keycode)->bool{
        KEYBOARD.borrow().is_held(key)
    }
    pub fn is_just_pressed(key: Keycode)->bool{
        KEYBOARD.borrow().is_just_pressed(key)
    }
    pub fn is_just_released(key: Keycode)->bool{
        KEYBOARD.borrow().is_just_released(key)
    }
    //pub fn keys_to_vector(right: Keycode, left: Keycode, up:Keycode, down: Keycode)->Vector2f{
    //   Vector2f::new(x, y)
    //   if(is_held(right))
    //}
}

pub struct InputDispatcher{}

impl InputDispatcher{
    pub fn reg_press(key: Keycode){
        KEYBOARD.borrow_mut().reg_press(key)
    }
    pub fn reg_release(key: Keycode){
        KEYBOARD.borrow_mut().reg_release(key)
    }
    pub fn frame_clear(){
       KEYBOARD.borrow_mut().frame_clear()
    }
}






