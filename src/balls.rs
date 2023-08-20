
/*
balls track:
who they were spawned by
who batted them 


control schemes:
depending on which direction you're pushing, the swing will happen in a different direction?
and send the ball ina different way.

or, if you have a controller,
one stick is the movement of the person, one stick is the movement of the bat.
that would work pretty good for a very physics-based game. And might be easier to start off with




put collision-event-reader compoonents on balls and on players
becuse they're the onees causing the collisions




 */



use std::time::Duration;

use bevy::math::Vec3Swizzles;
use bevy::{prelude::*, gizmos};
use super::components::*;
use super::bundles::BallBundle;

use super::{
	DROP_HEIGHT
};


const THROW_VEL: f32 = 300.;
const TOTAL_BALLS: usize = 2;



// pub fn ball_thrower(
// 	q_player: Query<&Transform, With<Player1Marker>>,
// 	options: Res<PlayerInfo>,
// 	mut timer: ResMut<ThrowTimer>,
// 	time: Res<Time>,
// 	mut commands: Commands
// ) {
// 	if options.balls {return}

// 	if timer.0.tick(time.delta()).just_finished() {
// 		if let Ok(transform) = q_player.get_single() {
// 			let player = transform.translation.xy();
// 			let origin = Vec2::new(0., 0.);
// 			let o2p = player - origin;

// 			commands.spawn(BallBundle {
// 				velocity: Velocity {linvel: THROW_VEL * o2p.normalize(), ..default()},
// 				transform: Transform::from_xyz(
// 					origin.x, origin.y, 0.
// 				),
// 				restitution: Restitution::coefficient(0.7),
// 				..default()
// 			})
// 			.insert(FromPlayer(Entity::PLACEHOLDER))
// 			.insert(TimeAdded(time.elapsed_seconds()));
// 		}
// 	}
// }


pub fn drop_ball(
	mut commands: Commands, 
	// mut timer: ResMut<BallTimer>,
	// player_query: Query<(Entity, &Transform), With<InputHolder>>,
	mut player_q: Query<(Entity, &Transform, &mut DropOnMeRate)>,
	time: Res<Time>,
	mut gizmos: Gizmos
) {

	for (e, transform, mut timer) in player_q.iter_mut() {
		gizmos.circle_2d(transform.translation.xy(), 20., Color::LIME_GREEN);

		if timer.0.tick(time.delta()).just_finished() {
			let t = transform.translation;

			commands.spawn(BallBundle {
				transform: Transform::from_xyz(t.x, t.y + DROP_HEIGHT, 0.0),
				..default()
			})
			.insert(FromPlayer(e))
			.insert(TimeAdded(time.elapsed_seconds()));
		}
	}

	// // update ball timer
	// if timer.0.tick(time.delta()).just_finished() {
	// 	for (entity, transform) in player_query.iter() {
	// 		let t = transform.translation;

	// 		commands.spawn(BallBundle {
	// 			transform: Transform::from_xyz(t.x, t.y + DROP_HEIGHT, 0.0),
	// 			..default()
	// 		})
	// 		.insert(FromPlayer(entity))
	// 		.insert(TimeAdded(time.elapsed_seconds()));
	// 	}
	// }
}


pub fn manage_timers(
	mut q: Query<&mut DropOnMeRate>, 
	time: Res<Time>,
) {

	// let ts = time.elapsed_seconds().floor() as i32;

	// if ts == 5 {
	// 	for mut drop_rate in q.iter_mut() {
	// 		drop_rate.0.set_duration(Duration::from_secs(2));
	// 	}
	// }
}


pub fn manage_balls(
	mut commands: Commands, 
	ball_query: Query<(Entity, &TimeAdded)>
) {
	let mut ball_times: Vec<(Entity, &TimeAdded)> = ball_query.iter().collect();
	if ball_times.len() > 6 {
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




// ------------------------------------------------------



