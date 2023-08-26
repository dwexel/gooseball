
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





ok the idea is that balls drop at a spedicif time for both players
	so if that continues I should make the timer a resource


 */



use bevy::{prelude::*};
use super::components::*;
use super::bundles::BallBundle;
use super::shapes::PieMaterial;


use super::{
	DROP_HEIGHT,
};


pub fn drop_ball(
	time: Res<Time>,
	mut commands: Commands, 
	timer_q: Query<&DropOnMeRate>,
	mut player_q: Query<(Entity, &Transform), With<InputHolder>>,
	asset_server: Res<AssetServer>
) {
	/* only use one timer right now */
	let timer = timer_q.single();

	for (e, transform) in player_q.iter_mut() {
		if timer.0.just_finished() {
			let t = transform.translation;

			commands.spawn(BallBundle {
				transform: Transform::from_xyz(t.x, t.y + DROP_HEIGHT, 0.0),
				sprite: Sprite {
					color: Color::rgba(0.0, 0.0, 0.5, 1.0),
					custom_size: Some(Vec2::splat(50.)),
					..default()
				},
				texture: asset_server.get_handle("icon.png"),
				..default()
			})
			.insert(BallLast::None)
			.insert(TimeAdded(time.elapsed_seconds()));
		}
	}
}


pub fn update_pie(
	time: Res<Time>,
	mut q_time: Query<&mut DropOnMeRate>,
	q: Query<&Handle<PieMaterial>>,

	// temporary
	mut meshes: ResMut<Assets<PieMaterial>>
) {
	let mut timer = q_time.single_mut();
	timer.0.tick(time.delta());

	for mat_handle in q.iter() {
		if let Some(mat) = meshes.get_mut(mat_handle) {
			mat.percentage = timer.0.percent();			
		}
	}
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

/*


pub fn ball_thrower(
	q_player: Query<&Transform, With<Player1Marker>>,
	options: Res<PlayerInfo>,
	mut timer: ResMut<ThrowTimer>,
	time: Res<Time>,
	mut commands: Commands
) {
	if options.balls {return}

	if timer.0.tick(time.delta()).just_finished() {
		if let Ok(transform) = q_player.get_single() {
			let player = transform.translation.xy();
			let origin = Vec2::new(0., 0.);
			let o2p = player - origin;

			commands.spawn(BallBundle {
				velocity: Velocity {linvel: THROW_VEL * o2p.normalize(), ..default()},
				transform: Transform::from_xyz(
					origin.x, origin.y, 0.
				),
				restitution: Restitution::coefficient(0.7),
				..default()
			})
			.insert(FromPlayer(Entity::PLACEHOLDER))
			.insert(TimeAdded(time.elapsed_seconds()));
		}
	}
}
 */