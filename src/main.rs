/*

sort components into areas:
some components manage the entity's state (by their presence or abscense)


with the paddle swing
-- how long does it take to reach the apex where it hits?
-- how long does it take until you can swing again


ideas;
increase ball weight
change paddle to dynamic?



idea:
maybe combine pause/unpause keypress with menu-related keypresses
abstract input sets......


todo:
make ball sensor flash on hit


figure out the swing thing
maybe the ball only gets a hit for one frame?
like it turns to a sensor after one frame or gets collision group removed?


figure out font stuff


todo:
make ui show getting hit,
like a counter


ok now.
balls systemesssj


player radius: 50
bat length: 40
ball radius: 20

ok it seemed to me like the game was recting too slowly when the frames were lower
like in a not normal way.
check that out



todo:
make a class for "input map" / "keybinds" 
that you can pass to a generic system


debug text resource


ok so. I would like to make it so only the balls have the component that lets their collisions 
be detected, at least for now.

but maybe I will be implementing pivkups later...

todo:
use the css stuff to add log elements rather than pushing strings...


ball thrower


 */


use bevy::transform::TransformSystem;
use bevy::{
    asset::ChangeWatcher, 
    prelude::*, 
    utils::Duration, 
};
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;



mod components;
use components::*;

mod jump;
mod balls;
mod menu;
mod builder;
mod bundles;


#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct CollectInput;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct ApplyInput;


fn main() {
    App::new()

        .add_plugins(DefaultPlugins
            .set(AssetPlugin {
                // This tells the AssetServer to watch for changes to assets.
                // It enables our scenes to automatically reload in game when we modify their files.
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

        /*
            so before/after/chain work on system sets
            and bevy makes it so a tuple of systems can be coerced into a system set
            (or for that metter a single system)

            but you can also create explicit sets with labels
         */

        .configure_sets(Update,
            (CollectInput, ApplyInput).chain()
        )

        .add_systems(Update,
            (
                bevy::window::close_on_esc,

                // zoom_2d,
                camera_system,

                // detect runtime scene changes
                builder::added_system,
                builder::changed_system,

                (
                    get_input_wasd_system,
                    get_input_arrow_system
                )
                .in_set(CollectInput),

                (   
                    apply_input_system,
                    apply_movement_to_paddles,
                    jump::apply_jump_query,
                )
                .in_set(ApplyInput),



                // update timers
                update_remove_timers::<JumpTimer>,
                update_remove_timers::<OneShot>,

                balls::ball_thrower,

                balls::drop_ball,
                balls::manage_balls,
                balls::display_events,

                pause_menu_button_system,

                update_log_system

            )
            .run_if(in_state(AppState::InGame))
        )
        .add_systems(PostUpdate, 
            (reset_updated_flags).run_if(in_state(AppState::InGame))
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
                // state is set here but ... when will it be changed?
                reset_system, 
                spawn_players_system
            )
            .chain()
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

        .init_resource::<PlayerInfo>()


        .insert_resource(LogText(
            vec!["try me!".to_string()]
        ))
        
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

const SWING_RIGHT: bool = true;
const ANIM_LENGTH: f32 = 0.5;
const PADDLE_DISTANCE: f32 = 100.;
const DROP_HEIGHT: f32 = 200.;


//
const ACC: f32 = 12.;
const BALL_TIME: f32 = 2.;

// wacko
const GRAVITY_SCALE: f32 = 2.;


const JUMP_CHECK_HEIGHT: f32 = 80.0;
const JUMP_IMPULSE: f32 = 100.;
const JUMP_TIME: f32 = 2.;


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

    // spawn ball sensor
    // ball detector component?
    commands.spawn((
        // RigidBody::Fixed,
        // ball sensor has to have a sensor
        // bundle would make sense
        // BallSensor::new(),

        Sensor,
        // ActiveEvents::COLLISION_EVENTS,
        // incideental
        Collider::ball(100.),
        TransformBundle::from(Transform::from_xyz(100., 100., 0.)),
    ));

    // spawn pickup
    // requires sensor and pickupsensor component


}

fn spawn_players_system(
    info: Res<PlayerInfo>, 
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    
    let icon_handle = asset_server.load("goose2.png");

    let c1 = commands.spawn(
        bundles::PaddleBundle {..default()}
    )
    .id();

    commands.spawn(
        bundles::PlayerBundle {
            texture: icon_handle.clone(),
            transform: Transform::from_xyz(0., -100., 0.),
            gravity_scale: GravityScale(GRAVITY_SCALE),
            restitution: Restitution::coefficient(0.7),
            ..default()
        }
    )
    .insert(InputMethod_wasd)
    .insert(SingleChild(c1))
    .insert(Player1Marker);

    if info.players != 2 {return}

    let c2 = commands.spawn(
        bundles::PaddleBundle {..default()}
    )
    .id();

    commands.spawn(
        bundles::PlayerBundle {
            texture: icon_handle.clone(),
            transform: Transform::from_xyz(100., -100., 0.),
            gravity_scale: GravityScale(GRAVITY_SCALE),
            restitution: Restitution::coefficient(0.7),
            ..default()
        }
    )
    .insert(InputMethod_arrow)
    .insert(SingleChild(c2))
    .insert(Player2Marker);
}



/* query all RigidBodies and despawn them, then set gamestate back to in-game */
fn reset_system (
    mut commands: Commands, 
    bodies: Query<Entity, With<RigidBody>>,
    mut next: ResMut<NextState<AppState>>
) {
    for entity in bodies.iter() {
        commands.entity(entity).despawn();
    }

    next.set(AppState::InGame);
}



// -------------------- update

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
            _ => {}
        }
    }
}



fn zoom_2d(
    mut q: Query<&mut OrthographicProjection>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let zoom_speed = 10.;
    let mut projection = q.single_mut();
    // zoom out
    if keys.pressed(KeyCode::Minus) {
        projection.scale += zoom_speed * time.delta_seconds();
    }
    // zooom in
    if keys.pressed(KeyCode::Equals) {
        projection.scale -= zoom_speed * time.delta_seconds();
    }

    projection.scale = projection.scale.clamp(0.5, 5.0);
}

fn camera_system(
    mut q_camera: Query<(&mut Transform, &mut OrthographicProjection)>,
    q_targets: Query<&Transform, (With<CameraTarget>, Without<OrthographicProjection>)>,

    //
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {


    let (mut camera, mut projection) = q_camera.single_mut();

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

    // set camera pos
    camera.translation = centroid;

    // -------------

    let cam = Vec2::new(camera.translation.x, camera.translation.y);

    let furthest: Vec2;

    if q_targets.iter().count() == 2 {
        let f_tran = q_targets.iter().next().unwrap().translation;
        furthest = Vec2::new(f_tran.x, f_tran.y);
    }
    else {
        furthest = Vec2::ZERO;
    }


    let cam_to_target = furthest - cam;

    let padding = 50.;
    let window_x = 400.;
    let window_y = 300.;

    // projection.scale =
    //         (cam_to_target.x.abs() / window_x)
    //     .max(cam_to_target.y.abs() / window_y)
    //     .max(1.);

    if true {

        projection.scale =
            ((cam_to_target.x.abs() + padding) / window_x)
        .max((cam_to_target.y.abs() + padding) / window_y)
        .max(1.);

    }
    else {
        if keys.pressed(KeyCode::Minus) {
            projection.scale += 10. * time.delta_seconds();
        }
        // zooom in
        if keys.pressed(KeyCode::Equals) {
            projection.scale -= 10. * time.delta_seconds();
        }
    
        projection.scale = projection.scale.clamp(0.5, 5.0);
    }
}


// fn get_input(mut q: Query<(&mut InputHolder, InputMethod)>) {
//     for (mut holder, keybinds) in q.iter_mut() {
//         match keybinds {
//             // yeah
//         }
//     }
// }


    // good for analog input
// note: could probably genericize this

fn get_input_wasd_system(
    mut players_query: Query<&mut InputHolder, With<InputMethod_wasd>>,
    keys: Res<Input<KeyCode>>
) {
    let mut h = InputHolder { direction: Vec2::ZERO, jump: false, swing: false };

    if keys.pressed(KeyCode::S) {
        h.direction.y -= 1.;
    }
    // if keys.pressed(KeyCode::W) {
    //     h.direction.y += 1.;
    // }

    if keys.just_pressed(KeyCode::W) {
        h.jump = true;
    }
    if keys.pressed(KeyCode::D) {
        h.direction.x += 1.;
    }
    if keys.pressed(KeyCode::A) {
        h.direction.x -= 1.;
    }
    if keys.just_pressed(KeyCode::F) {
        h.swing = true;
    }

    h.direction = h.direction.normalize();

    for mut holder in players_query.iter_mut() {
        *holder = h.clone();
    }
}

// yeah
fn get_input_arrow_system(
    mut players_query: Query<&mut InputHolder, With<InputMethod_arrow>>,
    keys: Res<Input<KeyCode>>
) {
    let mut h = InputHolder { direction: Vec2::ZERO, jump: false, swing: false };

    if keys.pressed(KeyCode::Down) {
        h.direction.y -= 1.;
    }
    if keys.just_pressed(KeyCode::Up) {
        h.jump = true;
    }
    if keys.pressed(KeyCode::Right) {
        h.direction.x += 1.;
    }
    if keys.pressed(KeyCode::Left) {
        h.direction.x -= 1.;
    }
    if keys.any_just_pressed([KeyCode::M]) {
        h.swing = true;
    }
    
    // apply input
    for mut holder in players_query.iter_mut() {
        *holder = h.clone();
    }
}


// update state of the player entity
fn apply_input_system(
    mut commands: Commands,
    mut players_query: Query<(&InputHolder, &SingleChild, &mut Velocity)>
) {
    for (input, child, mut velocity) in players_query.iter_mut() {

        // check input magnitude
        // length squared is faster
        
        // let input = Vec2::new()

        if input.direction.length_squared() >= 0.01 {


            velocity.linvel += input.direction * ACC;
        }


        if input.swing {
            // use special child component
            // add oneshot entity
            let paddle = child.0;
            commands.entity(paddle).insert(OneShot {length: ANIM_LENGTH, ..default()});
        }
    }
}





/*
apply motion to paddles
*/
fn apply_movement_to_paddles(
    mut paddles: Query<(&mut Transform, Option<&OneShot>), With<PaddleMarker>>,
    players: Query<(&SingleChild, &Transform), Without<PaddleMarker>>
) {
    // if player is facing right, we want the sign to be negative
    // yea

    for (child, p_transform) in players.iter() {
        if let Ok((mut c_transform, anim)) = paddles.get_mut(child.0) {
            let sign = match SWING_RIGHT { true => 1., false => -1.};
            let span = Vec3::new(sign * PADDLE_DISTANCE, 0., 0.);
            if let Some(a) = anim {
                // define an arc as the range
                let arc_width = PI / 2.;
                let arc_start = PI / 4.;

                // positive rotation = counterclockwise
                // negitive rotation = clockwise
                // animation starts and stops at a designated point
                let theta = a.normalized();
                
                let rot = Quat::from_rotation_z((sign * theta * arc_width) + sign * arc_start);
                c_transform.translation = p_transform.translation + (rot * span);
                c_transform.rotation = rot;
                
            }
            else {
                let easing_speed = 0.3;

                // resting mode
                let target = p_transform.translation + span;
                let current = c_transform.translation;
                c_transform.translation += (target - current) * easing_speed;
                c_transform.rotation = Quat::IDENTITY;
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
