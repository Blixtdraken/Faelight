use std::{cell::UnsafeCell, thread::LocalKey};


#[macro_export]
macro_rules!  thread_cell {
    ($vis:vis $name: ident, $t: ty) => {
        thread_local!{
            static _INNER: std::cell::UnsafeCell<$t> = std::cell::UnsafeCell::new(<$t>::default());
        }
        $vis static $name: crate::core::thread_cell::ThreadCell<$t> = 
        crate::core::thread_cell::ThreadCell::<$t>::new(&_INNER);
    };
}

pub struct ThreadCell<T: Default + 'static>{
    pub (in crate::core::thread_cell)
    source: &'static LocalKey<UnsafeCell<T>>
}

impl<T: Default + 'static> ThreadCell<T> {

    //You should use thread_cell!() macro instead of this
    pub const fn new(static_cell: &'static LocalKey<UnsafeCell<T>>)->Self{
        Self{source: static_cell}
    }

    pub fn borrow(&'static self)->&'static T{
        self.source.with(|cell| 
                unsafe { & *cell.get()}
            )
    }

    pub fn borrow_mut(&'static self)->&'static mut T{
            self.source.with(|cell| 
                unsafe { &mut *cell.get()}
            )
        
    }

}






//impl<T: Default + 'static>  Deref for ThreadCell<T>{
//    type Target = T;
//
//    fn deref(&self) -> &Self::Target {
//        self.source.with(|cell| 
//                unsafe { & *cell.get()}
//            )
//    }
//}
//impl<T: Default + 'static>  DerefMut for ThreadCell<T>{
//    fn deref_mut(&mut self) -> &mut T {
//         self.source.with(|cell| 
//                unsafe { &mut *cell.get()}
//            )
//    }
//}