
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
use super::components::*;

use super::GRAVITY_SCALE;



pub fn drop_ball(
	mut commands: Commands, 
	mut timer: ResMut<BallTimer>,
	player_query: Query<(Entity, &Transform), With<InputHolder>>,
	time: Res<Time>,
) {
	// update ball timer
	if timer.0.tick(time.delta()).just_finished() {

		for (entity, transform) in player_query.iter() {

			let trans = transform.translation;


			commands.spawn((

				// this is so... players can track colissions
				FromPlayer(entity),

				// todo: use durations
				TimeAdded(time.elapsed_seconds()),



				Velocity {..default()},
				RigidBody::Dynamic,
				Collider::ball(20.0),

				GravityScale(GRAVITY_SCALE),

				// high masss?
				ColliderMassProperties::Density(5.0),
				Restitution::coefficient(0.7),
				
				ActiveEvents::COLLISION_EVENTS,
				
				TransformBundle::from(Transform::from_xyz(
					trans.x, trans.y + 100.0, 0.0
				))
			));    
		}
	}
}


pub fn manage_balls(
	mut commands: Commands, 
	ball_query: Query<(Entity, &TimeAdded)>
) {
	let mut ball_times: Vec<(Entity, &TimeAdded)> = ball_query.iter().collect();
	
	let total_balls = 5;

	if ball_times.len() > total_balls {
		// todo: use some kind of oop to make this cleaner

		ball_times.sort_by(
			|a, b| a.1.0.partial_cmp(&b.1.0).unwrap()
		);

		// ball_times.sort_by_key(|a| {a.1.0});

		// despawn oldest ball
		commands.entity(ball_times[0].0).despawn();
	}
}


/* A system that displays the events. */
pub fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
      //   println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
      //   println!("Received contact force event: {:?}", contact_force_event);
    }
}