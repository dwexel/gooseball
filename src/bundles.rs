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
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
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
    pub drop_rate: DropOnMeRate,
}

#[allow(unused_parens)]
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            sprite: default(),
            transform: default(),
            global_transform: default(),
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: default(),
            computed_visibility: default(),
            camera_target: default(),
            input_holder: default(),
            velocity: default(),
            rigid_body: RigidBody::KinematicVelocityBased,
            character_controller: KinematicCharacterController {
                offset: CharacterLength::Absolute(0.1),
                ..default()
            },


            collider: Collider::capsule_y(30., 20.),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collision_groups: CollisionGroups::new(Group::GROUP_1, Group::ALL),

            ball_sensor: default(),

            source: default(),
            settings: PlaybackSettings { 
                mode: PlaybackMode::Loop, volume: Volume::new_relative(0.5), speed: 1.0, paused: false 
            },
            drop_rate: DropOnMeRate(Timer::from_seconds(3., TimerMode::Repeating))
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

}


impl Default for BallBundle {
	fn default() -> Self {
		Self {
		    rigid_body: RigidBody::Dynamic,
			collider: Collider::ball(20.),
            collision_groups: CollisionGroups::new(Group::GROUP_2, Group::ALL ^ Group::GROUP_3),
			gravity_scale: GravityScale(GRAVITY_SCALE),
			mass_props: default(),
            restitution: Restitution::coefficient(0.7),
			ext_impulse: default(),
			active: ActiveEvents::COLLISION_EVENTS,
			velocity: default(),
			transform: default(),
			global_transform: default()
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
            collision_groups: CollisionGroups::new(Group::GROUP_3, Group::ALL ^ Group::GROUP_2),
            player_sensor: PlayerSensor { despawn_on_enter: false },
            sensor: default(),
            collider: Collider::ball(50.),
            transform: default(),
            global_transform: default()
        }
    }
}


/*
    commands.spawn((
        /* in group 2, collide with all */
        CollisionGroups::new(Group::GROUP_2, Group::ALL),
        PlayerSensor {despawn_on_enter: true},
        Sensor,
        Collider::ball(50.),
        TransformBundle::from(Transform::from_xyz(-250., 20., 0.)),
    ));

 */


