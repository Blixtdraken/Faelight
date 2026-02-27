use std::cell::Cell;



thread_local! {
    pub static RENDER_CALLBACK: Cell<RenderCallback> = Cell::<RenderCallback>::new(RenderCallback::new());
}
//Taking is safe because 

#[derive(Default)]
struct RenderCallback{

}


impl RenderCallback {

        pub fn new()->Self{

            let borrow = RENDER_CALLBACK.take(); 
            RENDER_CALLBACK.take();

            Self {  }
        }




}
