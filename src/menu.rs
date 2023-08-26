/*

idea:
a system that takes a component as a generic
and "shows" or "hides" all entities with that component

 */

use bevy::prelude::*;
use crate::AppState;
use super::components::*;


pub fn setup_menu_system(mut commands: Commands, asset_server: Res<AssetServer>) {
	let _fonts: Vec<HandleUntyped> = asset_server.load_folder("fonts/").unwrap();

	// why is it on the right?
	// also , get_handle don't panic but it doens't warn either
	let text_style = TextStyle {font_size: 20., font: asset_server.get_handle("fonts/FiraMono-Regular.ttf"), color: Color::WHITE};
	
	commands.spawn((
		MenuMarker,
		NodeBundle { 
			style: Style { flex_direction: FlexDirection::Column, flex_basis: Val::Percent(30.), align_items: AlignItems::Center, justify_content: JustifyContent::Center, ..default() },
			visibility: Visibility::Hidden,
			background_color: BackgroundColor(Color::MIDNIGHT_BLUE),
			..default()
		}
	))
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
		parent.spawn((
			TextBundle { text: Text::from_section("log (showing)", text_style.clone()), ..default()},
		));
		parent.spawn((
			TextBundle { text: Text::from_section("cam", text_style.clone()), ..default()},
		));
	});
}


pub fn run_menu_system(
	keys: Res<Input<KeyCode>>,
	mut menu_query: Query<(Entity, &mut Text), With<Parent>>,
	mut menu_pointer: Local<usize>,

	//
	mut next: ResMut<NextState<AppState>>,
	// mut player_options: ResMut<PlayerInfo>,
	mut settings_balls: ResMut<Settings_balls>,
	mut settings_players: ResMut<Settings_players>,
	mut settings_log: ResMut<Settings_log>,
	mut settings_camera_system: ResMut<Settings_camera_system>,

	//
	mut display: Query<&mut Visibility, With<LogTextDisplayer>>

) {
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

	if let Some(e) = ents.get(*menu_pointer) {
		let (_, mut t) = menu_query.get_mut(*e).unwrap();

		t.sections.push(TextSection { 
			value: "*".into(), 
			style: TextStyle {font_size: 14., ..default()}
		});

		if keys.any_just_pressed([KeyCode::F, KeyCode::M]) {
			match *menu_pointer {
				0 => {
					settings_players.0 = 1;
					next.set(AppState::Reset);
				},
				1 => {
					next.set(AppState::Reset);
				},
				2 => {
					settings_balls.0 = !settings_balls.0;
					t.sections[0].value = match settings_balls.0 {
						true => "balls (on)".to_string(),
						false => "balls (off)".to_string()
					}
				},
				3 => {
					settings_log.0 = !settings_log.0;
					t.sections[0].value = match settings_log.0 {
						true => {
							for mut v in display.iter_mut() {
								*v = Visibility::Visible
							}
							"log (showing)".to_string()
						},
						false => {
							for mut v in display.iter_mut() {
								*v = Visibility::Hidden
							}
							"log (hiding)".to_string()
						}
					}
				},
				4 => {
					settings_camera_system.0 = !settings_camera_system.0;
					next.set(AppState::InGame);
				}

				_ => {}
			}
		}
	}
}

// changes all hidden nodes to visibile
// menu parent is set to be hidden so...
pub fn show_menu(mut q: Query<&mut Visibility, With<MenuMarker>>) {
	for mut visibility in q.iter_mut() {
		if *visibility == Visibility::Hidden {
			*visibility = Visibility::Visible;
		}
	}
}

// changes all visible nodes to hidden
pub fn hide_menu(mut q: Query<&mut Visibility, With<MenuMarker>>) {
	for mut visibility in q.iter_mut() {
		if *visibility == Visibility::Visible {
			*visibility = Visibility::Hidden;
		}
	}
}