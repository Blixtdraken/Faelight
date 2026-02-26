use std::collections::HashSet;

use sdl3::keyboard::Keycode;

pub struct KeyboardState{
    is_held:       HashSet<Keycode>,
    just_pressed:  HashSet<Keycode>,
    just_released: HashSet<Keycode>
}

impl KeyboardState{

    pub fn new()->Self{
        Self { is_held: HashSet::with_capacity(32), just_pressed: HashSet::with_capacity(8), just_released: HashSet::with_capacity(8) }
    }

    pub fn regPress(&mut self, key: Keycode){
       self.just_pressed.insert(key);
       self.is_held.insert(key);
    }

    pub fn regRelease(&mut self, key: Keycode){
        self.just_released.insert(key);
        self.is_held.remove(&key);
    }

    pub fn frameClear(&mut self){
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub fn isHeld(&self, key: Keycode)->bool{
        self.is_held.contains(&key)
    }

    pub fn isJustPressed(&self, key: Keycode)->bool{
        self.just_pressed.contains(&key)
    }
    pub fn isJustReleased(&self, key: Keycode)->bool{
       self.just_released.contains(&key)
    }

}