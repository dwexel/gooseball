
/*
balls track:
who they were spawned by
who batted them 


the problem 
i want them to behave less ....
yeah


maybe make the paddle a SENSOR and apply forces man ually?


control schemes:
depending on which direction you're pushing, the swing will happen in a different direction?
and send the ball ina different way.

or, if you have a controller,
one stick is the movement of the person, one stick is the movement of the bat.
that would work pretty good for a very physics-based game. And might be easier to start off with



options


 */


use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;

use super::components::*;

use super::{
	DROP_HEIGHT,
	GRAVITY_SCALE
};







pub fn drop_ball(
	mut commands: Commands, 
	mut timer: ResMut<BallTimer>,
	player_query: Query<(Entity, &Transform), With<InputHolder>>,
	time: Res<Time>,
	options: Res<PlayerInfo>
) {
	if !options.balls {
		// yeah,,,,, but can you move this outside of the system?
		return;
	}

	// update ball timer
	if timer.0.tick(time.delta()).just_finished() {
		for (entity, transform) in player_query.iter() {
			let t = transform.translation;

			commands.spawn((
				// this is so... players can track colissions???
				FromPlayer(entity),
				// todo: use durations


				TimeAdded(time.elapsed_seconds()),
				Velocity {..default()},
				RigidBody::Dynamic,
				Collider::ball(20.0),
				GravityScale(GRAVITY_SCALE),

				Damping {linear_damping: 2., ..default()},

				// high masss?
				ColliderMassProperties::Density(5.0),
				Restitution::coefficient(0.7),
				// ActiveEvents::COLLISION_EVENTS,
				TransformBundle::from(Transform::from_xyz(
					t.x, t.y + DROP_HEIGHT, 0.0
				))
			));    
		}
	}
}




const TOTAL_BALLS: usize = 5;

pub fn manage_balls(
	mut commands: Commands, 
	ball_query: Query<(Entity, &TimeAdded)>
) {
	let mut ball_times: Vec<(Entity, &TimeAdded)> = ball_query.iter().collect();
	if ball_times.len() > TOTAL_BALLS {
		// todo: use some kind of oop to make this cleaner
		// or sort by key?
		ball_times
			.sort_by(
				|a, b| a.1.0.partial_cmp(&b.1.0).unwrap()
		);
		// despawn oldest ball
		commands.entity(ball_times[0].0).despawn();
	}
}


/* A system that displays the events. */
pub fn display_events(
   mut collision_events: EventReader<CollisionEvent>,
   // mut contact_force_events: EventReader<ContactForceEvent>,
	mut sensors: Query<(&mut BallSensor, &Transform)>
) {
   for collision_event in collision_events.iter() {
      // println!("Received collision event: {:?}", collision_event);

		let (s, e1, e2, flags) = match *collision_event {
			CollisionEvent::Started(e1, e2, flags) => {
				("started", e1, e2, flags)
			},
			CollisionEvent::Stopped(e1, e2, flags) => {
				("stopped", e1, e2, flags)
			}
		};

		if flags == CollisionEventFlags::SENSOR {
			println!("sensor {s}");
		}

		// update ball sensors

		if let CollisionEvent::Started(e1, e2, flags) = *collision_event {
			// use bitwise?
			if flags == CollisionEventFlags::SENSOR {
				if let Ok((mut b, _)) = sensors.get_mut(e1) {
					b.hit_on_last_update = true;
				}
				else if let Ok((mut b, _)) = sensors.get_mut(e2) {
					b.hit_on_last_update = true;
				}
			}
		}
	}
}