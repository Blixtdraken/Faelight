#![allow(dead_code)]
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use num_traits::{ConstOne, ConstZero, Float, Num, PrimInt, Unsigned};

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


impl<T: Num + Copy + ConstOne + ConstZero> Vector2<T>{

    pub fn new(x: T, y: T)->Self{
        Self { x, y }
    }
    
    pub const ZERO: Self = Self{x: T::ZERO, y: T::ZERO};
    pub const ONE:  Self = Self{x: T::ONE,  y: T::ONE};
    
    pub const UP:    Self = Self{x: T::ZERO, y: T::ONE};
    pub const RIGHT: Self = Self{x: T::ONE,  y: T::ZERO};

}

impl<T: Float + Copy> Vector2<T> {
    
    pub fn normalize(&mut self){
        let length = self.len();
        let fraction = T::one()/length;
        self.x = self.x * fraction;
        self.y = self.y * fraction;
    }

    pub fn normalized(mut self) -> Self{
       self.normalize();
       self
    }

    pub fn len(&self)-> T{
        (self.x*self.x + self.y*self.y).sqrt()  
    }
    
    pub fn dot(self, other: Self) -> T {
    self.x * other.x + self.y * other.y
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


impl<T: Copy + Num> Div<T> for Vector2<T>{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        Self {x, y}
    }
}

impl<T: Copy + Num> Mul<T> for Vector2<T>{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        Self {x, y}
        
    }
}

impl<T: Copy + Num> Add<Vector2<T>> for Vector2<T>{
    type Output = Self;
    fn add(self, rhs: Vector2<T>) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Self {x, y}
        
    }
}

impl<T: Copy + Num> Sub<Vector2<T>> for Vector2<T>{
    type Output = Self;
    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Self {x, y}
        
    }
}

impl<T: Copy + Num> AddAssign<Vector2<T>> for Vector2<T>{
    fn add_assign(&mut self, rhs: Vector2<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;   
    }
}

impl<T: Copy + Num> SubAssign<Vector2<T>> for Vector2<T>{
    fn sub_assign(&mut self, rhs: Vector2<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}







