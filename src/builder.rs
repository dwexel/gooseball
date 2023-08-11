use super::components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


pub fn added_system(
	query_1: Query<(Entity, &BuilderBlock), Added<BuilderBlock>>,
	query_2: Query<(Entity, &BuilderLine), Added<BuilderLine>>,
	mut commands: Commands
) {
	for (entity, b) in &query_1 {
		commands.entity(entity).insert((
			TransformBundle::from(Transform::from_xyz(b.x, b.y, 0.)),
			Collider::cuboid(b.w, b.h),
		));
	}

	for (entity, l) in &query_2 {

		let vertices = l.v.iter();

		// copies values?
		let v = vertices.map(|(x, y)| Vec2::new(*x, *y));

		commands.entity(entity).insert((
			TransformBundle::from(Transform::from_xyz(0., 0., 0.)),
			Collider::polyline(v.collect(), None)
		));
	}
}

pub fn changed_system(
	// disjoitn
	mut query_1: Query<(&BuilderBlock, &mut Transform, &mut Collider), Changed<BuilderBlock>>,
	mut query_2: Query<(&BuilderLine, &mut Transform, &mut Collider), (Changed<BuilderLine>, Without<BuilderBlock>)>,
) {
	for (b, mut t, mut c) in query_1.iter_mut() {
		println!("updating: {} {}", b.x, b.y);
		t.translation.x = b.x;
		t.translation.y = b.y;
		// todo
		*c = Collider::cuboid(b.w, b.h);
	}

	for (l, mut _t, mut c) in query_2.iter_mut() {
		println!("updating: {:?}", l.v);
		
		// doesn't remove old collider point?
		let vertices = l.v.iter().map(|(x, y)| Vec2::new(*x, *y));
		*c = Collider::polyline(vertices.collect(), None);
	}
}

