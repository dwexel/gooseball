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



#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]

pub enum AppState {
    #[default]
    InGame,
    PauseMenu,
    Reset
}




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
                }), ..default()
            })
        )
        .add_plugins((
            // what does this mean
            // RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0).with_default_system_setup(false),
            RapierDebugRenderPlugin::default()
        ))

 
        .add_systems(Startup, (
            setup,
            spawn_players_system,
            menu::setup_menu_system
        ))

        .add_systems(Update,
            (
                bevy::window::close_on_esc,

                zoom_2d,

                // todo
                camera_system.run_if(resource_equals(PlayerInfo {players: 2, balls: false, camera_system: true})),

                // detect runtime scene changes
                builder::added_system,
                builder::changed_system,


                // todo ordering
                get_input_wasd_system,
                get_input_arrow_system,

                // apply movement
                apply_input_system,
                apply_movement_to_paddles,
                jump::apply_jump_query,

                // update timers
                update_remove_timers::<JumpTimer>,
                update_remove_timers::<OneShot>,

                balls::drop_ball,
                balls::manage_balls,
                balls::display_events,

                pause_menu_button_system
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

        // // hmmmmm
        // todo
        // .register_type::<PlayerInfo>()

        .add_state::<AppState>()


        .insert_resource(BallTimer(Timer::from_seconds(BALL_TIME, TimerMode::Repeating)))
        .insert_resource(PlayerInfo {
            players: 2,
            balls: false,
            camera_system: false
        })
        
        .run();
}




const SCENE_FILE_PATH: &str = "main.scn.ron";



const SWING_RIGHT: bool = true;
const ANIM_LENGTH: f32 = 0.5;
const PADDLE_DISTANCE: f32 = 100.;
const DROP_HEIGHT: f32 = 200.;


//
const ACC: f32 = 20.0;
const BALL_TIME: f32 = 2.0;
const GRAVITY_SCALE: f32 = 6.0;








// ------------------ setup

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(DynamicSceneBundle {
        scene: asset_server.load(SCENE_FILE_PATH), ..default()
    });

    commands.spawn(Camera2dBundle::default());

    commands.spawn(
        TextBundle::from_section(
            "Nothing to see in this window! Check the console output!",
            TextStyle {
                font_size: 50.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            ..default()
        }),
    );

    // spawn ball sensor
    // ball detector component?
    commands.spawn((
        // RigidBody::Fixed,
        // ball sensor has to have a sensor
        // bundle would make sense
        BallSensor::new(),
        Sensor,
        ActiveEvents::COLLISION_EVENTS,


        // incideental
        Collider::ball(100.),
        TransformBundle::from(Transform::from_xyz(100., 100., 0.)),
    ));
}

fn spawn_players_system(info: Res<PlayerInfo>, mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon_handle = asset_server.load("icon.png");
    let c1 = commands.spawn((
        PaddleMarker,
        RigidBody::KinematicPositionBased,
        Collider::cuboid(40.0, 10.0),
        TransformBundle::default(),
        // in group 2
        // collide with everything except group 1 
        CollisionGroups::new(Group::GROUP_2, (Group::ALL ^ Group::GROUP_1))   
    )).id();

    // note: density, damping
    commands.spawn((
        Player1Marker,
        SingleChild(c1),
        CameraTarget,
        SpriteBundle {
            texture: icon_handle.clone(),
            transform: Transform::from_xyz(0., -100., 100.),
            sprite: Sprite {
                custom_size: Some(Vec2 {x: 100., y: 100.}),
                ..default()
            },
            ..default()
        },
        InputMethod_wasd,
        InputHolder {..default()},
        Velocity {..default()},
        RigidBody::Dynamic,
        Collider::ball(50.0),
        GravityScale(GRAVITY_SCALE),
        Restitution::coefficient(0.7),
        LockedAxes::ROTATION_LOCKED,
        // p;ayers are in group 1
        CollisionGroups::new(Group::GROUP_1, (Group::ALL ^ Group::GROUP_2)),
        ExternalImpulse::default()

    ));
    
    if info.players != 2 {
        return;
    }

    let c2 = commands.spawn((
        PaddleMarker,
        RigidBody::KinematicPositionBased,
        Collider::cuboid(40.0, 10.0),
        TransformBundle::default(),
        // in group 2
        // collide with everything except group 1 
        CollisionGroups::new(Group::GROUP_2, (Group::ALL ^ Group::GROUP_1))   
    )).id();

    let p2 = commands.spawn((
        Player2Marker,
        SingleChild(c2),
        CameraTarget,


        SpriteBundle {
            texture: icon_handle.clone(),
            transform: Transform::from_xyz(0., 100., 100.),
            sprite: Sprite {
                custom_size: Some(Vec2 {x: 100., y: 100.}),
                ..default()
            },
            ..default()
        },
        InputMethod_arrow,
        InputHolder {..default()},
        Velocity {..default()},
        RigidBody::Dynamic,
        Collider::ball(50.0),
        GravityScale(GRAVITY_SCALE),
        Restitution::coefficient(0.7),
        LockedAxes::ROTATION_LOCKED,
        CollisionGroups::new(Group::GROUP_1, (Group::ALL ^ Group::GROUP_2)),
        ExternalImpulse::default()
    )).id();
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
    mut gizmos: Gizmos,
    mut q_camera: Query<(&mut Transform, &Camera, &mut OrthographicProjection)>,
    q_targets: Query<&Transform, (With<CameraTarget>, Without<Camera>)>,

) {
    let (mut camera, c, mut projection) = q_camera.single_mut();

    if let Ok(target) = q_targets.get_single() {
        camera.translation = target.translation;
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

    // in reference to the camera position,
    // what components are we needing to loook at?
    let start = Vec2::new(camera.translation.x, camera.translation.y);

    // technically you should get the furthest 
    // but i'll just get the first
    let target = q_targets.iter().next().unwrap().translation;

    // target is the vector from the camera to the target
    let target = Vec2::new(target.x, target.y) - start;

    // window is 800, 600
    // pixels that is
    // todo: update  so it's reactive
    gizmos.rect_2d(Vec2::ZERO, 0., Vec2::new(800., 600.), Color::RED);

    // so which edges of the screen are you closest to
    // distance from x line,
    // distance from y line
    let close_to_x = (400. - target.x.abs()).abs();
    let close_to_y = (300. - target.y.abs()).abs();

    if close_to_x < close_to_y {
        projection.scale = target.x.abs() / 400.;
    }
    else {
        projection.scale = target.y.abs() / 400.;
    }

    projection.scale = projection.scale.max(0.7);
    // todo: padding
}



// note: could probably genericize this
fn get_input_wasd_system(
    mut players_query: Query<&mut InputHolder, With<InputMethod_wasd>>,
    keys: Res<Input<KeyCode>>
) {
    // good for analog input
    let mut h = InputHolder { direction: Vec2::ZERO, jump: false, swing: false };

    if keys.any_pressed([KeyCode::S]) {
        h.direction.y -= 1.;
    }
    if keys.any_pressed([KeyCode::W]) {
        h.jump = true;
    }
    if keys.any_pressed([KeyCode::D]) {
        h.direction.x += 1.;
    }
    if keys.any_pressed([KeyCode::A]) {
        h.direction.x -= 1.;
    }

    // if keys.any_pressed([KeyCode::F]) {
    if keys.any_just_pressed([KeyCode::F]) {
        h.swing = true;
    }
    
    // apply input
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

    if keys.any_pressed([KeyCode::Down]) {
        h.direction.y -= 1.;
    }
    if keys.any_pressed([KeyCode::Up]) {
        h.jump = true;
    }
    if keys.any_pressed([KeyCode::Right]) {
        h.direction.x += 1.;
    }
    if keys.any_pressed([KeyCode::Left]) {
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





// apply motion to paddles
fn apply_movement_to_paddles(
    mut paddles: Query<(&mut Transform, Option<&OneShot>), With<PaddleMarker>>,
    // p1_q: Query<(&SingleChild, &Transform), (With<Player1Marker>, Without<PaddleMarker>)>,
    // p2_q: Query<(&SingleChild, &Transform), (With<Player2Marker>, Without<PaddleMarker>)>,
    players: Query<(&SingleChild, &Transform), Without<PaddleMarker>>
) {

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


/*
    // unstable
    // get 2 players and the vector between them
    let (p1_child, p1_transform) = match p1_q.get_single() {
        Ok(c) => c,
        Err(QuerySingleError::NoEntities(_)) => {
            println!("Error: There is no player!");
            return;
        }
        Err(QuerySingleError::MultipleEntities(_)) => {
            println!("Error: There is more than one player!");
            return;
        }
    };

    let (p2_child, p2_transform) = match p2_q.get_single() {
        Ok(c) => c,
        Err(QuerySingleError::NoEntities(_)) => {
            println!("Error: There is no player!");
            return;
        }
        Err(QuerySingleError::MultipleEntities(_)) => {
            println!("Error: There is more than one player!");
            return;
        }
    };
    
    let p1_to_p2 = p2_transform.translation - p1_transform.translation;

    // for both children, update the pddles
    for (child, p_transform, vector) in [(p1_child.0, p1_transform, p1_to_p2), (p2_child.0, p2_transform, -p1_to_p2)] {
        
        let (mut c_transform, animation) = paddles.get_mut(child).unwrap();

        // if player is facing right, 
        // we want the sign to be negative
        let sign = match (vector.x >= 0.) { true => -1., false => 1.};
        let span = Vec3::new(sign * PADDLE_DISTANCE, 0., 0.);

        // if paddle is swinging
        if let Some(animation) = animation {
            let theta = animation.normalized();
            let rot = Quat::from_rotation_z(sign * theta * 2. * PI);
            c_transform.translation = p_transform.translation + (rot * span);
            c_transform.rotation = rot;
        }
        else {
            // resting mode
            let target = p_transform.translation + span;
            let current = c_transform.translation;
            c_transform.translation += (target - current) * 0.5;
            c_transform.rotation = Quat::IDENTITY;
        }
    }
 */
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