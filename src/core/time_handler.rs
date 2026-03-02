use std::time::Instant;

use crate::thread_cell;



struct TimeHandler{
    start_time: Instant,
    last_frame: Instant,
    last_delta_time: f64
}

impl TimeHandler {

    pub fn new()->Self{
        TimeHandler { start_time: Instant::now(), last_frame: Instant::now(), last_delta_time: 0.0_f64}
    }

    //Return delta time and reset delta timer, (Only run once a frame)
    pub fn reset_get_delta(&mut self)->f64{
        let delta_time = self.last_frame.elapsed().as_secs_f64();
        self.last_frame = Instant::now();
        delta_time  
    }
    //Return delta time and reset delta timer, (Only run once a frame)
    pub fn reset_get_delta_f32(&mut self)->f32{
        let delta_time = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();
        delta_time  
    }

}

impl Default for TimeHandler{
    fn default() -> Self {
        Self::new()
    }
}