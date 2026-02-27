use std::ops::{Div, Mul};

use num_traits::{Float, Num, PrimInt, Unsigned};

#[derive(Copy, Clone, Default)]
pub struct Vector2<T>{
    pub x: T,
    pub y: T
}


pub type Vector2f   = Vector2<f32>;
pub type Vector2i   = Vector2<i32>;
pub type Vector2u   = Vector2<u32>;
pub type Vector2f64 = Vector2<f64>;
pub type Vector2i64 = Vector2<i64>;
pub type Vector2u64 = Vector2<u64>;


impl<T: Num + Copy> Vector2<T>{
    pub fn new(x: T, y: T)->Self{
        Self { x, y }
    }
    pub fn zero()->Self{
        Self { x: T::zero(), y: T::zero() }
    }
}

impl<T: Float + Copy> Vector2<T> {

    pub fn normalize(&mut self){
        let length = self.len();
        let fraction = T::one()/length;
        self.x = self.x * fraction;
        self.y = self.y * fraction;
    }
    pub fn normalized(&self) -> Self{
        let mut res = *self;
        res.normalize();
        res 
    }

    pub fn len(&self)-> T{
        (self.x*self.x + self.y*self.y).sqrt()  
    }

}


impl<T: PrimInt + Copy> Vector2<T> {

    //Do integer stuff :P

}

impl<T: Unsigned + Copy> Vector2<T> {

    pub fn width(&self)->T {
        self.x
    }

    pub fn height(&self)->T {
        self.y
    }

}


impl<T: Div<Output = T> + Copy> Div<T> for Vector2<T>{

    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        Self {x, y}
        
    }
    
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector2<T>{

    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        Self {x, y}
        
    }
    
}




