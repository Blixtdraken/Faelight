use std::{cell::Cell, mem::{self}, ops::{Deref, DerefMut}, thread::LocalKey};


#[macro_export]
macro_rules!  thread_cell {
    ($name: ident, $t: ty) => {
        thread_local!{
            static _INNER: std::cell::Cell<Option<$t>> = std::cell::Cell::new(Some(<$t>::default()));
        }
        pub static $name: crate::core::thread_cell::ThreadCell<$t> = crate::core::thread_cell::ThreadCell::<$t>::new(&_INNER);
    };
}


pub struct ThreadCell<T: Default + 'static>{
    pub (in crate::core::thread_cell)
    source: &'static LocalKey<Cell<Option<T>>>
}

pub struct TakeGuard<T: Default + 'static>{
    value: T,
    owner: &'static ThreadCell<T>
}

impl<T: Default> TakeGuard<T>{

    pub fn borrow(&self)->&T{
        &self.value
    }
    pub fn borrow_mut(&mut self)->&mut T{
        &mut self.value
    }

}


impl<T: Default + 'static> ThreadCell<T> {

    //You should use thread_cell!() macro instead of this
    pub const fn new(static_cell: &'static LocalKey<Cell<Option<T>>>)->Self{
        
        Self{source: static_cell}
    }

    //Panics if thread has already been taken
    //Takeguard returns the take when it goes out of scope.
    pub fn take(&'static self)->TakeGuard<T>{
        let val = self.source.with(|cell| cell.take()).expect("Thread cell was taken twice.");
        TakeGuard { value: val, owner: self }
    }

    //None if already taken
    //Same as take otherwise
    pub fn try_take(&'static self)->Option<TakeGuard<T>>{
        let maybe_value = self.source.with(|cell| cell.take());
        match maybe_value {
            Some(val) => Some(TakeGuard { value: val, owner: self }),
            None => None,
        }
    }

   //pub fn give(&'static mut self, guard: &mut TakeGuard<T>){
   //    
   //}

}

impl<T: Default> Drop for TakeGuard<T>{
    fn drop(&mut self) {
        self.owner.source.with(|cell| cell.set(Some(mem::take(&mut self.value))));
    }
}

impl<T: Default> Deref for TakeGuard<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
    
}
impl<T: Default> DerefMut for TakeGuard<T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}



