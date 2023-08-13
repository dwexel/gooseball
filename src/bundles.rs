use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};
use bevy_rapier2d::prelude::*;
use super::components::*;



#[derive(Bundle)]
pub struct PlayerBundle {
    // insert
    // pub single_child: SingleChild,
    
    // sprite bundle
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,

    // cam
    pub camera_target: CameraTarget,

    // can't put in inputmethod righ tnow
    pub input_holder: InputHolder,

    // physics 
    pub velocity: Velocity,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub gravity_scale: GravityScale,
    pub restitution: Restitution,
    pub locked_axes: LockedAxes,
    pub collision_groups: CollisionGroups,
    pub external_impulse: ExternalImpulse,

    //
    pub ball_sensor: BallSensor
}

#[allow(unused_parens)]
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            //
            sprite: default(),
            transform: default(),
            global_transform: default(),
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: default(),
            computed_visibility: default(),
            //
            camera_target: default(),
            input_holder: default(),
            //
            velocity: default(),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(50.),
            gravity_scale: default(),
            restitution: default(),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collision_groups: CollisionGroups::new(Group::GROUP_1, (Group::ALL ^ Group::GROUP_2)),
            external_impulse: default(),
            ball_sensor: default()

        }
    }
}



#[derive(Bundle)]
pub struct PaddleBundle {
    // doesn't need to be pub because it doesn't need to be specified when 
    // creating ever
	pub paddle_marker: PaddleMarker,

    // transform bundle
    // The transform of the entity.
    pub local: Transform,
    // The global transform of the entity.
    pub global: GlobalTransform,
    
    // physics
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub collision_groups: CollisionGroups
}

#[allow(unused_parens)]
impl Default for PaddleBundle {
    fn default() -> Self {
        Self {
            paddle_marker: default(),
            local: default(),
            global: default(),
            rigid_body: RigidBody::KinematicPositionBased,
            collider: Collider::cuboid(40., 10.),
            collision_groups: CollisionGroups::new(Group::GROUP_2, (Group::ALL ^ Group::GROUP_1))   
        }
    }
}



#[derive(Bundle)]
pub struct BallBundle {}

