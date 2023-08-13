
use bevy::prelude::*;



#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BuilderBlock {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}


#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BuilderLine {
    pub v: Vec<(f32, f32)>
}

//-------------------------------

#[derive(Resource, PartialEq)]
pub struct PlayerInfo {
    pub players: u8,
    pub balls: bool,
    pub camera_system: bool,
    pub show_log: bool
}

impl Default for PlayerInfo {
    fn default() -> Self {
        Self {
            players: 2, 
            balls: false,
            camera_system: false, 
            show_log: true
        }
    }
}



// -----------------



// #[derive(Component)]
// pub enum InputMethod {
//     WASD,
//     ARROW
// }



#[allow(non_camel_case_types)]
#[derive(Component)]
pub struct InputMethod_wasd;

#[allow(non_camel_case_types)]
#[derive(Component)]
pub struct InputMethod_arrow;





#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct InputHolder {
	pub direction: Vec2,
	pub jump: bool,
	pub swing: bool
}



#[derive(Component, Default)]
pub struct PaddleMarker;

#[derive(Component)]
pub struct SingleChild(pub Entity);


#[derive(Component)]
pub struct Player1Marker;


#[derive(Component)]
pub struct Player2Marker;





#[derive(Resource)]
pub struct BallTimer(pub Timer);

#[derive(Resource)]
pub struct ThrowTimer(pub Timer);




#[derive(Component)]
pub struct TimeAdded(pub f32);


#[derive(Component)]
pub struct FromPlayer(pub Entity);


#[derive(Component, Default)]
pub struct BallSensor {
    pub hit_on_last_update: bool,
    pub inside: bool
}




#[derive(Component, Default)]
pub struct CameraTarget;



//-----------------------------------



pub trait RemoveAfter {
    // returns true if ready to remove
    fn tick(&mut self, t: f32) -> bool;
}




#[derive(Component, Debug)]
pub struct OneShot {
   pub position: f32,    
   pub length: f32,
}

impl RemoveAfter for OneShot {
    fn tick(&mut self, t: f32) -> bool {
        self.position += t;
        self.position >= self.length
    }
}

impl Default for OneShot {
	fn default() -> Self {
		Self { position: 0.0, length: 5.0 }
	}
}

impl OneShot {
   pub fn normalized(&self) -> f32 { self.position / self.length }
}

#[derive(Component)]
pub struct JumpTimer {
    pub position: f32,
    pub length: f32
}

impl RemoveAfter for JumpTimer {
    fn tick(&mut self, t: f32) -> bool {
        self.position += t;
        self.position >= self.length
    }
}

impl JumpTimer {
    pub fn new(len: f32) -> Self {
        Self {position: 0., length: len}
    } 
}

//------------

#[derive(Resource)]
pub struct LogText(pub Vec<String>);

#[derive(Component)]
pub struct LogTextDisplayer;



#[derive(Event)]
pub struct LogEvent(pub String);



#[derive(Component)]
pub struct MenuMarker;

