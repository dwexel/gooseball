#[allow(non_camel_case_types)]

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


// for serializing

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

// i don't need indi
pub struct BuilderLine {
    pub v: Vec<(f32, f32)>
}



//-------------------------------

#[derive(Resource, PartialEq)]
pub struct PlayerInfo {
    pub players: u8,
    pub balls: bool,
    pub camera_system: bool
}


//-------------

#[derive(Bundle)]
pub struct PlayerBundle {
    pub single_child: SingleChild,
    
    // sprite bundle
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,


    // can't put in inputmethod righ tnow
    pub input_holder: InputHolder,

    // physics 
    velocity: Velocity,
    rigid_body: RigidBody,
    collider: Collider,
    gravity_scale: GravityScale,
    restitution: Restitution,
    locked_axes: LockedAxes,
    collision_groups: CollisionGroups,
    external_impulse: ExternalImpulse

}

#[derive(Bundle)]
pub struct PaddleBundle {

    paddle_marker: PaddleMarker,

    // transform bundle
    /// The transform of the entity.
    pub local: Transform,
    /// The global transform of the entity.
    pub global: GlobalTransform,
    
    // physics
    rigid_body: RigidBody,
    collider: Collider,
    collision_groups: CollisionGroups
}

#[derive(Bundle)]
pub struct BallBundle {}


//-----------------------------





// #[derive(Resource)]
// pub struct PlayerInfo {
//     pub players: u8
// }

#[derive(Component)]
pub struct InputMethod_wasd;

#[derive(Component)]
pub struct InputMethod_arrow;



#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct InputHolder {
	pub direction: Vec2,
	pub jump: bool,
	pub swing: bool
}


#[derive(Component)]
pub struct PaddleMarker;

#[derive(Component)]
pub struct SingleChild(pub Entity);


#[derive(Component)]
pub struct Player1Marker;


#[derive(Component)]
pub struct Player2Marker;



#[derive(Resource)]
pub struct Player1(pub Entity);

#[derive(Resource)]
pub struct Player2(pub Entity);


#[derive(Resource)]
pub struct BallTimer(pub Timer);

#[derive(Component)]
pub struct TimeAdded(pub f32);


#[derive(Component)]
pub struct FromPlayer(pub Entity);


#[derive(Component)]
pub struct BallSensor {
    pub hit_on_last_update: bool,
    pub inside: bool
}

impl BallSensor {
    pub fn new() -> Self {
        Self {
            hit_on_last_update: false,
            inside: false
        }
    }
}




#[derive(Component)]
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

// move out traitss?
// pub trait ResetAfterUpdate {
//     fn reset(&mut self);
// }


