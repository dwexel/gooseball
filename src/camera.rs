
use bevy::prelude::*;
use crate::components::*;


pub fn camera_system(
	mut q_camera: Query<(&mut Transform, &mut OrthographicProjection)>,
	q_targets: Query<&Transform, (With<CameraTarget>, Without<OrthographicProjection>)>,
	//
	keys: Res<Input<KeyCode>>,
	time: Res<Time>,
) {
	let (mut camera_transform, mut projection) = q_camera.single_mut();


	println!("{}", camera_transform.translation);
	
/* 
	// if one target
	if let Ok(target) = q_targets.get_single() {
		camera.translation = target.translation;
		projection.scale = 1.;
		return;
	}

	// convert to vec2?
	let sum: Vec3 = q_targets.iter().map(|t| &t.translation).sum();  
	let count = q_targets.iter().count() as f32;

	let centroid = Vec3::new(
		sum.x / count,
		sum.y / count,
		0.
	);

	camera.translation = centroid;
 */


	// -------------
/* 
	let cam = Vec2::new(camera.translation.x, camera.translation.y);

	let furthest = match q_targets.iter().count() {
		2 => {
		  let f_tran = q_targets.iter().next().unwrap().translation;
		  Vec2::new(f_tran.x, f_tran.y)  
		},
		_ => {
		  todo!();
		}
	};

	let cam_to_target = furthest - cam;
	let padding = 50.;
	let window_x = 400.;
	let window_y = 300.;

	projection.scale =
		    ((cam_to_target.x.abs() + padding) / window_x)
		.max((cam_to_target.y.abs() + padding) / window_y)
		.max(1.);

 */
}

pub fn camera_system_2(
	mut q_camera: Query<(&mut Transform, &mut OrthographicProjection)>,
	keys: Res<Input<KeyCode>>,
	time: Res<Time>,
) {
	let (_, mut projection) = q_camera.single_mut();

	/* zoom out */
	if keys.pressed(KeyCode::Minus) {
	   projection.scale += 10. * time.delta_seconds();
	}
	/* zoom in */
	if keys.pressed(KeyCode::Equals) {
	   projection.scale -= 10. * time.delta_seconds();
	}

	projection.scale = projection.scale.clamp(0.5, 5.0);
}