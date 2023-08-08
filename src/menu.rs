/*
convertt to vec...



 */


use bevy::prelude::*;
use crate::AppState;

use super::components::*;



pub fn setup_menu_system(mut commands: Commands, asset_server: Res<AssetServer>) {
	// grab resources
	// hmm
	// todo
	// let _fonts: Vec<HandleUntyped> = asset_server.load_folder("fonts").unwrap();

	
	commands.spawn(
		NodeBundle { 
			style: Style { flex_direction: FlexDirection::Column, flex_basis: Val::Percent(30.), align_items: AlignItems::Center, justify_content: JustifyContent::Center, ..default() },
			visibility: Visibility::Hidden,
			background_color: BackgroundColor(Color::MIDNIGHT_BLUE),
			..default()
		}
	)
	.with_children(|parent| {
		parent.spawn((
			TextBundle { text: Text::from_section("set 1 player", TextStyle {..default()}), ..default()},
		));
		parent.spawn((
			TextBundle { text: Text::from_section("reset position", TextStyle {..default()}), ..default()},
		));
		parent.spawn((
			TextBundle { text: Text::from_section("balls", TextStyle {..default()}), ..default()},
		));
	});
}


const MENU_SIZE: usize = 3;


pub fn run_menu_system(
	keys: Res<Input<KeyCode>>,
	mut menu_query: Query<(&mut Text), With<Parent>>,
	mut menu_pointer: Local<usize>,
	mut next: ResMut<NextState<AppState>>,
	mut player_options: ResMut<PlayerInfo>
) {

	if keys.any_just_pressed([KeyCode::W, KeyCode::Up]) {
		if *menu_pointer == 0 { *menu_pointer = MENU_SIZE }
		*menu_pointer -= 1;
	}

	if keys.any_just_pressed([KeyCode::S, KeyCode::Down]) {
		*menu_pointer += 1;
		if *menu_pointer == MENU_SIZE { *menu_pointer = 0 }
	}


	for mut t in menu_query.iter_mut() {
		while t.sections.len() > 1 {
			t.sections.pop();
		}
	}

	let mut i: usize = 0;
	for mut t in menu_query.iter_mut() {
		if i == *menu_pointer {
			t.sections.push(TextSection {
				value: "*".into(), ..default()
			})
		}
		i += 1;
	}

	// enter
	if keys.any_just_pressed([KeyCode::F, KeyCode::M]) {
		match *menu_pointer {
			0 => {
				player_options.players = 1;
				next.set(AppState::Reset);
			},
			1 => {
				println!("reset!");
				next.set(AppState::Reset);
			},
			2 => {},
			_ => {}
		}
	}

}



// changes all hidden nodes to visibile
pub fn show_menu(mut q: Query<&mut Visibility>) {
	for mut visibility in q.iter_mut() {
		if *visibility == Visibility::Hidden {
			*visibility = Visibility::Visible;
		}
	}
}

// changes all visible nodes to hidden
pub fn hide_menu(mut q: Query<&mut Visibility>) {
	for mut visibility in q.iter_mut() {
		if *visibility == Visibility::Visible {
			*visibility = Visibility::Hidden;
		}
	}
}