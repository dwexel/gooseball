/*


with the paddle swing
-- how long does it take to reach the apex where it hits?
-- how long does it take until you can swing again

player radius: 50
bat length: 40
ball radius: 20





todo:
make a class for "input map" / "keybinds" 
that you can pass to a generic system



sensor classes
-- make sure that the system that updates the sensors gets run before the system that reads them
-- also could you have one that uses genrics?





todo:
use the css stuff to add log elements rather than pushing strings...


audio feebback for the player speed
(like footsteps get faster)

"closeness to ground" compnent



kinematic player vs. dynamc player
-- with dynamic: i should put in linear damping on it if there's no direction being pressed down


so an animation has 2 separate things: one, when it's done, and two, when it's ready to be repeated
/



SO FOR THE HIT SYSTEM:
it's Directional,
up: big hit towards othe player (straight out)
up side: up and toward other
side: goes a good distance

maybe, don't need up side for now ...
but i should try it later


 */


use bevy::math::Vec3Swizzles;
use bevy::transform::{TransformSystem, self};
use bevy::{
    asset::ChangeWatcher, 
    prelude::*, 
    utils::Duration, 
};
use bevy_rapier2d::prelude::*;



mod components;
use bundles::PlayerSensorBundle;
use components::*;

// mod jump;
mod balls;
mod menu;
mod builder;
mod bundles;
mod camera;


#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct CollectInput;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct ApplyInput;


fn main() {
    App::new()

        .add_plugins(DefaultPlugins
            .set(AssetPlugin {
                watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)), ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (800.0, 600.0).into(), ..default()
                }), 
                ..default()
            })
        )
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0).with_default_system_setup(false),
            RapierDebugRenderPlugin::default()
        ))
 
        .add_systems(Startup, (
            setup,
            spawn_players_system,
            menu::setup_menu_system
        ))
        .configure_sets(Update, (CollectInput, ApplyInput).chain())
        .add_systems(Update,
            (
                bevy::window::close_on_esc,
                // detect runtime scene changes
                builder::added_system,
                builder::changed_system,
                
                // update camera
                camera::camera_system,

                
                (
                    get_input::<InputMapArrow>,
                    get_input::<InputMapWASD>
                    // get_input_wasd_system,
                    // get_input_arrow_system
                )
                .in_set(CollectInput),

                (   
                    (apply_input_system, apply_swing).chain()
                )
                .in_set(ApplyInput),
                
                (
                    balls::manage_timers, balls::drop_ball 
                )
                .run_if(resource_exists_and_equals(Settings_balls(true))),



                // for now
                // balls::ball_thrower,
                balls::manage_balls,
                
                // update components that need to detect collisions
                update_sensors,

                pause_menu_button_system,
                update_log_system,
                update_sound_speed

            )
            .run_if(in_state(AppState::InGame))
        )
        .add_systems(PostUpdate,    
            (
                update_remove_timers::<JumpTimer>,
                // update_remove_timers::<OneShot>,
                reset_updated_flags.run_if(in_state(AppState::InGame))
            )
        )

        // add physics setup
        .configure_sets(PostUpdate,
            (
                PhysicsSet::SyncBackend,
                PhysicsSet::SyncBackendFlush,
                PhysicsSet::StepSimulation,
                PhysicsSet::Writeback,
            )
            .chain()
            .before(TransformSystem::TransformPropagate),
        )
        .add_systems(PostUpdate,
            (
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::SyncBackend).in_set(PhysicsSet::SyncBackend),
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::SyncBackendFlush).in_set(PhysicsSet::SyncBackendFlush),
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::StepSimulation).in_set(PhysicsSet::StepSimulation),
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::Writeback).in_set(PhysicsSet::Writeback),
            )
            .run_if(in_state(AppState::InGame))
        )

        // menu
        .add_systems(OnEnter(AppState::PauseMenu), menu::show_menu)
        .add_systems(OnExit(AppState::PauseMenu), menu::hide_menu)
        .add_systems(Update, 
            (
                bevy::window::close_on_esc,
                menu::run_menu_system,
                pause_menu_button_system
            )
            .run_if(in_state(AppState::PauseMenu))
        )

        .add_systems(OnEnter(AppState::Reset),
            (
                reset_system, 
                spawn_players_system
            ).chain()
        )

        // types that are deserialized
        .register_type::<BuilderBlock>()
        .register_type::<BuilderLine>()
        .register_type::<Vec2>()
        .register_type::<Vec<Vec2>>()

        .register_type::<(f32, f32)>()
        .register_type::<Vec<(f32, f32)>>()

        .add_state::<AppState>()
        
        .insert_resource(ThrowTimer(Timer::from_seconds(3., TimerMode::Repeating)))
        .insert_resource(BallTimer(Timer::from_seconds(BALL_TIME, TimerMode::Repeating)))
        // .init_resource::<PlayerInfo>()


        .insert_resource(LogText(
            vec!["try me!".to_string()]
        ))
        
        .insert_resource(Settings_players(2))
        .insert_resource(Settings_balls(true))
        .insert_resource(Settings_log(true))

        .add_event::<LogEvent>()

        .run();
}



#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    InGame,
    PauseMenu,
    Reset
}

const SCENE_FILE_PATH: &str = "main.scn.ron";
const BALL_SIZE: f32 = 20.;

// const SWING_RIGHT: bool = true;
// const ANIM_LENGTH: f32 = 0.5;
// const PADDLE_DISTANCE: f32 = 100.;

const DROP_HEIGHT: f32 = 200.;


const ACC: f32 = 16.;
const BALL_TIME: f32 = 2.;
// wacko
const GRAVITY_SCALE: f32 = 5.;

const JUMP_IMPULSE: f32 = 100.;



// ------------------ setup



fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(DynamicSceneBundle {
        scene: asset_server.load(SCENE_FILE_PATH), ..default()
    });

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        TextBundle {
            style: Style {justify_content: JustifyContent::Start, align_self: AlignSelf::FlexEnd, ..default()},
            ..default()
        },
        LogTextDisplayer
    ));

/*      commands.spawn((
        BallSensor::default(),
        Sensor,
        Collider::ball(100.),
        TransformBundle::from(Transform::from_xyz(100., 100., 0.)),
    ));
 */

    commands.spawn(PlayerSensorBundle {
        player_sensor: PlayerSensor { despawn_on_enter: true },
        transform: Transform::from_xyz(-250., 20., 0.),
        ..default()
    });

    // commands.spawn((
    //     /* in group 2, collide with all */
    //     CollisionGroups::new(Group::GROUP_1, Group::ALL),
    //     PlayerSensor::default(),
    //     Sensor,
    //     Collider::ball(100.),
    //     TransformBundle::from(Transform::from_xyz(100., 100., 0.)),
    // ));

    // /* spawn a pickup */
    // commands.spawn((
    //     /* in group 2, collide with all */
    //     CollisionGroups::new(Group::GROUP_1, Group::ALL),
    //     PlayerSensor {despawn_on_enter: true},
    //     Sensor,
    //     Collider::ball(50.),
    //     TransformBundle::from(Transform::from_xyz(-250., 20., 0.)),
    // ));
}



fn spawn_players_system(
    mut commands: Commands, 
    settings: Res<Settings_players>, 
    asset_server: Res<AssetServer>
) {
    
    let icon_handle = asset_server.load("goose2.png");

    commands.spawn(
        bundles::PlayerBundle {
            source: asset_server.load("Windless Slopes.ogg"),
            texture: icon_handle.clone(),
            transform: Transform::from_xyz(0., -100., 0.),
            ..default()
        }
    )
    .insert(InputMapWASD)
    .insert(Player1Marker);

    if settings.0 != 2 {return}

    commands.spawn(
        bundles::PlayerBundle {
            texture: icon_handle.clone(),
            transform: Transform::from_xyz(100., -100., 0.),
            ..default()
        }
    )
    .insert(InputMapArrow)
    .insert(Player2Marker);
}



/* query all RigidBodies and despawn them, then set gamestate back to in-game */
fn reset_system (
    mut commands: Commands, 
    bodies: Query<Entity, With<RigidBody>>,
    mut next: ResMut<NextState<AppState>>,
    mut log_q: Query<&mut Text, With<LogTextDisplayer>>
) {
    for entity in bodies.iter() {
        commands.entity(entity).despawn();
    }

    next.set(AppState::InGame);

    for mut text in log_q.iter_mut() {
        text.sections.clear();
    }
}



// -------------------- update

fn update_sound_speed(
    // music_controller: Query<&AudioSink, With<MyMusic>>, 
    players_displayers: Query<(&AudioSink, &Sprite, &Velocity)>,
) {
    for (sink, _, vel) in players_displayers.iter() {
        // so what exaxtly is the maximum velocity???????
        // how is such a thing calculated
        // todo: check if on the ground
        sink.set_speed(0.2 + (vel.linvel.x.abs() / 300.0));
        // todo: flip sprite
    }
}

fn pause_menu_button_system(
    keys: Res<Input<KeyCode>>,
    state: Res<State<AppState>>,
    mut next: ResMut<NextState<AppState>>
) {
    if keys.just_pressed(KeyCode::P) {
        match *state.get() {
            AppState::InGame => {
                next.set(AppState::PauseMenu);
            }
            AppState::PauseMenu => {
                next.set(AppState::InGame);
            },
            _ => {panic!();}
        }
    }
}





    // good for analog input
// note: could probably genericize this

#[derive(Component)]
struct InputMapWASD;

#[derive(Component)]
struct InputMapArrow;




trait InputMap {
    const RIGHT: KeyCode;
    const UP: KeyCode;
    const LEFT: KeyCode;
    const DOWN: KeyCode;
    const JUMP: KeyCode;
    const SWING: KeyCode;
}

impl InputMap for InputMapWASD {
    const RIGHT: KeyCode = KeyCode::D;
    const UP: KeyCode = KeyCode::W;
    const LEFT: KeyCode = KeyCode::A;
    const DOWN: KeyCode = KeyCode::S;
    const JUMP: KeyCode = KeyCode::F;
    const SWING: KeyCode = KeyCode::G;
}

impl InputMap for InputMapArrow {
    const RIGHT: KeyCode = KeyCode::Right;
    const UP: KeyCode = KeyCode::Up;
    const LEFT: KeyCode = KeyCode::Left;
    const DOWN: KeyCode = KeyCode::Down;
    const JUMP: KeyCode = KeyCode::BracketLeft;
    const SWING: KeyCode = KeyCode::BracketRight;
}

fn get_input<T: Component + InputMap>(
    keys: Res<Input<KeyCode>>, 
    mut q: Query<&mut InputHolder, With<T>>
) {
    let mut h = InputHolder { direction: Vec2::ZERO, jump: false, swing: false };

    if keys.pressed(T::RIGHT) {h.direction.x += 1.}
    if keys.pressed(T::UP) {h.direction.y += 1.}
    if keys.pressed(T::LEFT) {h.direction.x -= 1.}
    if keys.pressed(T::DOWN) {h.direction.y -= 1.}

    if keys.just_pressed(T::JUMP) {h.jump = true}
    if keys.just_pressed(T::SWING) {h.swing = true}

    h.direction = h.direction.normalize_or_zero();
    
    for mut holder in q.iter_mut() {
        *holder = h.clone();
    }
}

fn get_input_wasd_system(
    mut players_query: Query<&mut InputHolder, With<InputMethod_wasd>>,
    keys: Res<Input<KeyCode>>
) {
    // let k = *keys;

    let mut h = InputHolder { direction: Vec2::ZERO, jump: false, swing: false };

    if keys.pressed(KeyCode::D) {
        h.direction.x += 1.;
    }
    if keys.pressed(KeyCode::W) {
        h.direction.y += 1.;
    }
    if keys.pressed(KeyCode::A) {
        h.direction.x -= 1.;
    }
    if keys.pressed(KeyCode::S) {
        h.direction.y -= 1.;
    }

    if keys.just_pressed(KeyCode::F) {
        h.jump = true;
    }
    if keys.just_pressed(KeyCode::G) {
        h.swing = true;
    }

    h.direction = h.direction.normalize_or_zero();
    
    for mut holder in players_query.iter_mut() {
        *holder = h.clone();
    }
}

fn get_input_arrow_system(
    mut players_query: Query<&mut InputHolder, With<InputMethod_arrow>>,
    keys: Res<Input<KeyCode>>
) {
    let mut h = InputHolder { direction: Vec2::ZERO, jump: false, swing: false };

    if keys.pressed(KeyCode::Right) {
        h.direction.x += 1.;
    }
    if keys.pressed(KeyCode::Up) {
        h.direction.y += 1.;
    }
    if keys.pressed(KeyCode::Left) {
        h.direction.x -= 1.;
    }
    if keys.pressed(KeyCode::Down) {
        h.direction.y -= 1.;
    }

    if keys.just_pressed(KeyCode::Up) {
        h.jump = true;
    }
    if keys.just_pressed(KeyCode::M) {
        h.swing = true;
    }

    h.direction = h.direction.normalize_or_zero();
    
    for mut holder in players_query.iter_mut() {
        *holder = h.clone();
    }
}

#[derive(Clone, Copy, Component, Debug, PartialEq)]
enum Direction {
    Right,
    Up,
    Left,
    Down
}

impl TryFrom<Vec2> for Direction {
    // todo: check 0 with epsilon
    type Error = &'static str;

    fn try_from(value: Vec2) -> Result<Self, Self::Error> {
        if value.x.is_nan() || value.y.is_nan() {
            return Err("NaN in vector");
        }
        if value.x == 0. && value.y == 0. {
            return Err("Vector is 0");
        }
        let abs = value.abs();
        if abs.x >= abs.y {
            if value.x >= 0. {
                Ok(Self::Right)
            } else {
                Ok(Self::Left)
            }
        } else {
            if value.y >= 0. {
                Ok(Self::Up)
            } else {
                Ok(Self::Down)
            }
        }        
    }
}

impl From<&Direction> for Vec2 {
    fn from(value: &Direction) -> Self {
        match *value {
            Direction::Right => Self::X,
            Direction::Up => Self::Y,
            Direction::Left => Self::NEG_X,
            Direction::Down => Self::NEG_Y,
        }
    }
}




/*
can *move*
if you make input systems exclusive to each player


damn,,,,,
normalize_or_zero

 */


fn apply_input_system(
    time: Res<Time>,
    mut commands: Commands,
    mut players_q: Query<(
        Entity, 
        &InputHolder, 
        &mut Velocity,
        Option<&mut OneShot>,
        &mut ExternalImpulse, 
        &Transform
    )>,
    rapier_context: Res<RapierContext>
) {
    for (entity, input, mut velocity, o, mut ext, transform) in players_q.iter_mut() {
        let horz = Vec2::new(input.direction.x, 0.);

        /* apply acceleration or damping */

        if horz.length_squared() >= 0.01 {
            // velocity.linvel += horz * ACC;
            velocity.linvel += horz * 22.;
        }
        else if velocity.linvel.x != 0. {
            let sign = velocity.linvel.x.signum();
            velocity.linvel.x -= 10. * sign;
            if velocity.linvel.x.signum() != sign {
                velocity.linvel.x = 0.;
            }
        }

        /* can't start a new swing until the next frame huh */

        if let Some(mut oneshot) = o {
            if oneshot.0.tick(time.delta()).just_finished() {
                commands.entity(entity).remove::<OneShot>();
            }
        }
        else {
            if let Ok(dir) = Direction::try_from(input.direction) {
                commands.entity(entity).insert(dir);
            }
            if input.swing {
                commands.entity(entity).insert(
                    OneShot::from_seconds(0.5),
                );
            }
        }

        if input.jump {
            let ray_origin = transform.translation.xy();
            let ray_dir = Vec2::new(0., -1.);
            let max_toi = 100.;
            let solid = false;
            /* only query fixed rigidbodies and colliders with no body */
            let filter = QueryFilter::only_fixed();
            if let Some((_entity, toi)) = rapier_context.cast_ray(ray_origin, ray_dir, max_toi, solid, filter) {
                let distance_to_floor = (ray_dir * toi).y.abs();
                let player_height = 50.;
                if distance_to_floor < player_height + 10. {
                    ext.impulse = Vec2::new(0., JUMP_IMPULSE);
                }
            }
        }
    }
}




fn apply_swing(
    p1_q: Query<(Entity, &Transform), With<Player1Marker>>,
    p2_q: Query<(Entity, &Transform), With<Player2Marker>>,
    players_q: Query<(&OneShot, &Direction)>,

    mut balls_q: Query<(&Transform, &mut Velocity, &mut FromPlayer)>,
    mut gizmos: Gizmos,
    mut commands: Commands
) {
    let p1 = p1_q.single();
    let p2 = p2_q.single();
    let p1_to_p2 = p2.1.translation - p1.1.translation;

    for (e_player, translation, to_other) in [
        (p1.0, p1.1.translation, p1_to_p2), 
        (p2.0, p1.1.translation, -p1_to_p2)
    ] {
        if let Ok((_, pressed_direction)) = players_q.get(e_player) {
            let player_origin = translation.xy();
            let d: Vec2 = pressed_direction.into();
            let offset = d * 80.;

            /* box size: 100, 100 */

            gizmos.ray_2d(player_origin, offset, Color::DARK_GREEN);
            gizmos.rect_2d(player_origin + offset, 0., Vec2::new(100., 100.), Color::DARK_GREEN);
            let rect = Rect::from_center_size(player_origin + offset, Vec2::new(100., 100.));

            /* 
                player half width: 20
                player half height: 50
                hit velocity: 300
                */

            let foot = Vec2::new(
                player_origin.x + d.x * 20.,
                player_origin.y - 50.
            );

            gizmos.circle_2d(foot, 10., Color::GREEN);

            for (ball_transform, mut velocity, mut from_player) in balls_q.iter_mut() {
                if rect.contains(
                    ball_transform.translation.xy()
                ) {
                    from_player.0 = e_player;

                    commands.entity(e_player).remove::<OneShot>();

                    if *pressed_direction == Direction::Up {
                        velocity.linvel = 400. * Vec2::new(to_other.x.signum(), 0.);
                    }
                    else {
                        velocity.linvel = (ball_transform.translation.xy() - foot).normalize_or_zero() * 300.;         
                    }

                    // velocity.linvel == match *pressed_direction {
                    //     Direction::Up => 300. * Vec2::new(to_other.signum(), 0.),
                    //     Direction::Right => 300. * 
                    // }
                }
            }
        }
    }
}





fn update_remove_timers<T: RemoveAfter + Component>(
    mut commands: Commands,
    mut timer_query: Query<(Entity, &mut T)>,
    time: Res<Time>
) {
    for (entity, mut timer) in timer_query.iter_mut() {
        if timer.tick(time.delta_seconds()) {
            commands.entity(entity).remove::<T>();
        }
    }
}

fn reset_updated_flags(
    mut sensors_q: Query<&mut BallSensor>
) {
    for mut b in sensors_q.iter_mut() {
        b.hit_on_last_update = false;
    }
}

fn update_log_system(
    mut log: EventReader<LogEvent>,
    mut display: Query<&mut Text, With<LogTextDisplayer>>
) {
    let mut text = match display.get_single_mut() {
        Ok(text) => text,
        Err(_) => return
    };

    if !log.is_empty() {
        for l in log.iter() {

            // have to clone?
            let message = l.0.clone();

            text.sections.push(
                TextSection::new(
                    message,
                    TextStyle {
                        font_size:20., color: Color::WHITE, ..default()
                    }
                )
            );
        }
        // log.clear();
        
        let len = text.sections.len();

        if len > 9 {
            text.sections.drain(0..(len - 9));
        }
    }
}


fn update_sensors(
    mut collision_events: EventReader<CollisionEvent>,
    mut ball_sensors: Query<(&mut BallSensor, &Transform)>,
    
    player_sensors: Query<(&PlayerSensor, Entity)>,
    balls: Query<&FromPlayer>,
 
    mut log: EventWriter<LogEvent>,
    mut commands: Commands
 ) {
    for collision_event in collision_events.iter() {
  
        if let CollisionEvent::Started(e1, e2, _flags) = *collision_event {
 
            if let Ok((mut sensor, _)) = ball_sensors.get_mut(e1) {
                sensor.hit_on_last_update = true;
 
                // log.send(LogEvent(format!(
                // 	"ball sensor: {e1:?}\n"
                // )));
 
 
 
                 // unstable
                 let ball = balls.get(e2).unwrap();
                 log.send(LogEvent(format!("ball sensor: {:?} from: {:?}\n", e1, ball.0)));
 
             }
             else if let Ok((mut sensor, _)) = ball_sensors.get_mut(e2) {
                 sensor.hit_on_last_update = true;
 
                 // log.send(LogEvent(format!(
                 // 	"ball sensor: {e1:?}\n"
                 // )));
 
                 // unstable
                 let ball = balls.get(e1).unwrap();
                 log.send(LogEvent(format!("ball sensor: {:?} from: {:?}\n", e2, ball.0)));
             }
 
            if let Ok((sensor, sensor_e)) = player_sensors.get(e1) {
                log.send(LogEvent(format!("player sensor was hit: {e1:?}\n")));
                if sensor.despawn_on_enter {
                    commands.entity(sensor_e).despawn();
                }
            }
            else if let Ok((sensor, sensor_e)) = player_sensors.get(e2) {
                log.send(LogEvent(format!("player sensor was hit: {e2:?}\n")));
                if sensor.despawn_on_enter {
                    commands.entity(sensor_e).despawn();
                }
            }
 
 
             // use bitwise?
             // if flags == CollisionEventFlags::SENSOR {
             // 	if let Ok((mut b, _)) = sensors.get_mut(e1) {
             // 		b.hit_on_last_update = true;
             // 	}
             // 	else if let Ok((mut b, _)) = sensors.get_mut(e2) {
             // 		b.hit_on_last_update = true;
             // 	}
             // }
         }
     }
 }