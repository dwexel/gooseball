/*
can you set global gravity with rapier?


note:
some player components still have to be inserted manually ...

does player need "external impulse" compnent?


 */


use bevy::{
    prelude::*, 
    render::texture::DEFAULT_IMAGE_HANDLE, 
    audio::{
        PlaybackMode, 
        Volume
    }
};

use bevy_rapier2d::prelude::*;

use super::components::*;
use crate::GRAVITY_SCALE;



#[derive(Bundle)]
pub struct PlayerBundle {
    // sprite bundle
    // pub sprite: Sprite,
    // pub texture: Handle<Image>,

    pub sprite: TextureAtlasSprite,
    pub texture: Handle<TextureAtlas>,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,

    // mine
    pub camera_target: CameraTarget,
    pub input_holder: InputHolder,

    // physics 
    pub velocity: Velocity,
    pub rigid_body: RigidBody,
    pub character_controller: KinematicCharacterController,
    pub collider: Collider,
    pub locked_axes: LockedAxes,
    pub collision_groups: CollisionGroups,

    // mine
    pub ball_sensor: BallSensor,

    // audiobundle
    pub source: Handle<AudioSource>,
    pub settings: PlaybackSettings,

    // mine
    pub char_vel: CharacterVelocity
}

#[allow(unused_parens)]
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            sprite: default(),
            // texture: DEFAULT_IMAGE_HANDLE.typed(),
            texture: default(),
            transform: default(),
            global_transform: default(),
            visibility: default(),
            computed_visibility: default(),
            camera_target: default(),
            input_holder: default(),
            velocity: default(),
            rigid_body: RigidBody::KinematicVelocityBased,
            character_controller: KinematicCharacterController {
                // not working
                // apply_impulse_to_dynamic_bodies: true,

                // exlude sensors?
                // filter_flags: 
                ..default()
            },
            collider: Collider::capsule_y(30., 20.),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collision_groups: CollisionGroups::new(Group::GROUP_1, Group::ALL),
            ball_sensor: default(),
            source: default(),
            settings: PlaybackSettings { 
                mode: PlaybackMode::Loop, 
                volume: Volume::new_relative(0.5), 
                speed: 1.0, 
                paused: false 
            },
            char_vel: CharacterVelocity(Vec2::ZERO)
        }
    }
}



#[derive(Bundle)]
pub struct BallBundle {
	/* physics */
	pub rigid_body: RigidBody,
	pub collider: Collider,
    pub collision_groups: CollisionGroups,

    // mine
	pub gravity_scale: GravityScale,
	pub mass_props: ColliderMassProperties,
	pub restitution: Restitution,
	pub ext_impulse: ExternalImpulse,
	pub active: ActiveEvents,
	pub velocity: Velocity,

	/* transform bundle  */
	pub transform: Transform,
	pub global_transform: GlobalTransform,

    /* sprite bundle */
    pub sprite: Sprite,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility
}


impl Default for BallBundle {
	fn default() -> Self {
		Self {
		    rigid_body: RigidBody::Dynamic,
			collider: Collider::ball(20.),
            collision_groups: CollisionGroups::new(Group::GROUP_1, Group::ALL),
			gravity_scale: GravityScale(GRAVITY_SCALE),
			mass_props: default(),
            restitution: Restitution::coefficient(0.7),
			ext_impulse: default(),
			active: ActiveEvents::COLLISION_EVENTS,
			velocity: default(),
			transform: default(),
			global_transform: default(),

            // sprite: Sprite {
            //     color: Color::rgba(0.5, 0.5, 0.5, 1.0), 
            //     custom_size: Some(Vec2::new(100., 100.)), 
            //     ..default()
            // },

            sprite: default(),
            texture: default(),
            visibility: default(),
            computed_visibility: default()
		}
	}
}

#[derive(Bundle)]
pub struct PlayerSensorBundle {
    pub collision_groups: CollisionGroups,
    pub player_sensor: PlayerSensor,
    pub sensor: Sensor,
    pub collider: Collider,
    //
    pub transform: Transform,
    pub global_transform: GlobalTransform
}

impl Default for PlayerSensorBundle {
    fn default() -> Self {
        Self {
            collision_groups: CollisionGroups::new(Group::GROUP_1, Group::ALL),
            player_sensor: PlayerSensor { despawn_on_enter: false },
            sensor: default(),
            collider: Collider::ball(50.),
            transform: default(),
            global_transform: default()
        }
    }
}


#[derive(Bundle)]
pub struct GroundBundle {
    pub local: Transform,
    pub global: GlobalTransform,
    pub collider: Collider,
    pub body: RigidBody
}

impl Default for GroundBundle {
    fn default() -> Self {
        Self {
            local: default(),
            global: default(),
            collider: default(),
            body: RigidBody::Fixed
        }
    }
}