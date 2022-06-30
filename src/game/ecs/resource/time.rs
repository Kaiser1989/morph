//////////////////////////////////////////////////
// Using

//////////////////////////////////////////////////
// Definition

#[derive(Debug, Default)]
pub struct GameTime {
    pub frame_time: f32,
    pub all_time: f32,
}

//////////////////////////////////////////////////
// Implementation

impl GameTime {
    pub fn new(frame_time: f32, all_time: f32) -> GameTime {
        GameTime { frame_time, all_time }
    }

    pub fn update(&mut self, frame_time: f32) {
        self.frame_time = frame_time;
        self.all_time += frame_time;
    }
}
