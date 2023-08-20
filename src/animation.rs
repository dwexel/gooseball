// todo: implement range 

use bevy::prelude::*;
use std::time::Duration;
use std::ops::Range;


fn get_frame_index(start_frame: i8, num_frames: i8, percent: f32) -> usize {
    let n = num_frames as f32;
    let s = start_frame as usize;
    s + (percent * n).floor() as usize
}


pub struct Animation(i8, i8, Timer);

impl Animation {
    // takes first frame and last frame
    // or?
    
    // fn new(start: i8, end: i8, seconds: f32) -> Self {
    //     Self(start, end, Timer::from_seconds(seconds, TimerMode::Repeating))
    // }

    fn new(range: Range<i8>, seconds: f32) -> Self {
        Self(range.start, range.end, Timer::from_seconds(seconds, TimerMode::Repeating))
    }

    fn tick(&mut self, delta: Duration) {
        self.2.tick(delta);
    }

    fn get_frame_index(&self) -> usize {
        get_frame_index(self.0, self.1 - self.0, self.2.percent())
    }
}
