use bevy::prelude::*;
use bevy_rapier2d::prelude::{RapierContext, QueryFilter, ExternalImpulse};
use super::components::*;

const JUMP_CHECK_HEIGHT: f32 = 80.0;

/*
jump:
check if close enough to floor
if close enough and pressing buttin, 
appy jump and set a timer on the player that means they can't jumpp again until it's over

or, could check height, 
if they go high up enough they can jump again
just don't want double jumps

or if they touch back down.


 */


pub fn jump_query(
	mut controlled: Query<
		(Entity, &Transform, &InputHolder, &mut ExternalImpulse), 
		(With<InputHolder>, Without<JumpTimer>)
		>,
	rapier_context: Res<RapierContext>,
	mut commands: Commands
) {

	for (e, transform, input, mut ext_impulse) in controlled.iter_mut() {
		let ray_origin = Vec2::new(transform.translation.x, transform.translation.y);
		let ray_dir = Vec2::new(0., -1.);
		let max_toi = 100.;
		let solid = false;
		let filter = QueryFilter::only_fixed();

		if let Some((_entity, toi)) = rapier_context.cast_ray(ray_origin, ray_dir, max_toi, solid, filter) {
			
			let distance_to_floor = (ray_dir * toi).y.abs();

			// check if in jump heigt
			if distance_to_floor < JUMP_CHECK_HEIGHT {


				// apply jump
				if input.jump {
					// only insert new timer of old one is gone
					commands.entity(e).insert(JumpTimer::default());

					ext_impulse.impulse = Vec2::new(0., 100.);

				}



			}
		}
	}
}