/*
convertt to vec...


could have font reources in here

could put "drop ball" options in the existing options struct





is there a way to spawn a bunch of similar entities without using a bundle?



 */


use bevy::prelude::*;
use crate::AppState;

use super::components::*;



pub fn setup_menu_system(mut commands: Commands, asset_server: Res<AssetServer>) {
	// grab resources
	// hmm
	// todo

	
	let _fonts: Vec<HandleUntyped> = asset_server.load_folder("fonts/").unwrap();




	// why is it on the right?

	// also , get_handle don't panic but it doens't warn either
	let text_style = TextStyle {font_size: 20., font: asset_server.get_handle("fonts/FiraMono-Regular.ttf"), color: Color::WHITE};
	
	
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
			TextBundle { text: Text::from_section("set 1 player", text_style.clone()), ..default()},
		));
		parent.spawn((
			TextBundle { text: Text::from_section("reset position", text_style.clone()), ..default()},
		));
		parent.spawn((
			TextBundle { text: Text::from_section("balls (off)", text_style.clone()), ..default()},
		));
	});
}


const MENU_SIZE: usize = 3;

// mod arith

pub fn run_menu_system(
	keys: Res<Input<KeyCode>>,
	mut menu_query: Query<(Entity, &mut Text), With<Parent>>,
	mut menu_pointer: Local<usize>,

	//
	mut next: ResMut<NextState<AppState>>,
	mut player_options: ResMut<PlayerInfo>
) {

	// // store refs?
	// maybe use dereference
	// whatev

	let ents: Vec<Entity> = menu_query.iter().map(|(entity, _)| entity).collect();


	if keys.any_just_pressed([KeyCode::W, KeyCode::Up]) {
		if *menu_pointer == 0 { *menu_pointer = ents.len() }
		*menu_pointer -= 1;
	}

	if keys.any_just_pressed([KeyCode::S, KeyCode::Down]) {
		*menu_pointer += 1;
		if *menu_pointer == ents.len() { *menu_pointer = 0 }
	}



	// unordered iteration
	for (_, mut t) in menu_query.iter_mut() {
		while t.sections.len() > 1 {
			t.sections.pop();
		}
	}


	// get returns a borrow
	// safe 
	if let Some(e) = ents.get(*menu_pointer) {

		// get mutable ref to text
		// bevy approved way
		let (_, mut t) = menu_query.get_mut(*e).unwrap();

		// modify text
		t.sections.push(TextSection { 
			value: "*".into(), ..default() 
		});

		// functionalize this yayayayay
		if keys.any_just_pressed([KeyCode::F, KeyCode::M]) {
			match *menu_pointer {
				0 => {
					player_options.players = 1;
					next.set(AppState::Reset);
				},
				1 => {
					next.set(AppState::Reset);
				},
				2 => {
					// t.sections[0].value = ""
					player_options.balls = !player_options.balls;
					
					t.sections[0].value = match player_options.balls {
						true => "balls (on)".to_string(),
						false => "balls (off)".to_string()
					}

				},
				_ => {}
			}
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