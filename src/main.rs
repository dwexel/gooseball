/*



sensor classes
-- make sure that the system that updates the sensors gets run before the system that reads them
-- also could you have one that uses genrics?

"closeness to ground" component





todo:
-- use the css stuff to add log elements rather than pushing strings...


audio feebback for the player speed
(like footsteps get faster)





kinematic player vs. dynamic player
-- with dynamic: i should put in linear damping on it if there's no direction being pressed down


so an animation has 2 separate things: one, when it's done, and two, when it's ready to be repeated




SO FOR THE HIT SYSTEM:
it's Directional,
up: big hit towards othe player (straight out)
up side: up and toward other
side: goes a good distance

maybe, don't need up side for now ...
but i should try it later



ok "balls" isn't a good subject for a separate file

a better way to separate code owuld be by subject and schedule, like "player updates"

ok I do want to do a thing with the corner directions.....
especially because when you're pressing two directions at once you slow down, which could mesh 
with another mechanic ...
or crouching too is something to think about.


sound approaches
-- have a looping sound that I set the playback speed of based on velocity
-- have a one-shot sound that I play every time the player moves a set amount (modulo)


how to manually set framrate?


animation components

for each:
    indices, timer
    
shared:
    texture atlas, state




ok...
for stuff that won't change at game time, maybe i should let it be added with the commands


refactoring idea:
    run a system that stores a "direction-to-other" component in each player before 
    applying input stuff

 */

/* 
    the thing that handle points to has to implement 
        type uuid
        type path
        copy
    
     */


/*
ok so,
when are the actual collision event being generated? before or after "update" schedule?
when they're generated != when they're read





 */





use bevy::{
    prelude::*, 
    sprite::Anchor,
    asset::ChangeWatcher, 
    utils::Duration, 
};


use bevy::math::{Vec3Swizzles, Vec2Swizzles};
use bevy::sprite::{MaterialMesh2dBundle};
use bevy::transform::TransformSystem;


use bevy_oddio::oddio::Sample;
use bevy_oddio::builtins::{
    stream
};
use bevy_oddio::{AudioSource, AudioApp};
use bevy_oddio::{
    // AudioApp,
    AudioPlugin
};
use bevy_oddio::Audio;
use bevy_oddio::output::AudioSink;

use bevy_rapier2d::prelude::{*, QueryFilter};



mod components;
mod balls;
mod menu;
mod builder;
mod bundles;
mod camera;

use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use bundles::PlayerSensorBundle;
use components::*;





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
            RapierDebugRenderPlugin::default(),
            ShapePlugin,
            AudioPlugin::default()
        ))

        .add_audio_source::<Sample, stream::Stream<Sample>>()


        .add_systems(Startup, (
            setup,
            spawn_players_system,
            menu::setup_menu_system
        ))
        
        .add_systems(PreUpdate, update_sensors)
        
        .configure_sets(Update, (CollectInput, ApplyInput).chain())


        .add_systems(Update,
            (
                bevy::window::close_on_esc,

                // detect runtime scene changes
                builder::added_system,
                builder::changed_system,
                
                // update camera
                // camera::camera_system,
                camera::camera_system.run_if(resource_exists_and_equals(Settings_camera_system(true))),
                camera::camera_system_2.run_if(resource_exists_and_equals(Settings_camera_system(false))),

                // get button input
                (
                    get_input::<InputMapArrow>,
                    get_input::<InputMapWASD>
                )
                .in_set(CollectInput),

                (   
                    (apply_input_system, apply_swing).chain(),
                )
                .in_set(ApplyInput),
                
                (
                    balls::update_pie,
                    balls::drop_ball
                )
                .run_if(resource_exists_and_equals(Settings_balls(true))),

                modify_character_controller_slopes,

                /*
                    this system despawns balls!
                */
                balls::manage_balls,

                pause_menu_button_system,
                update_log_system,             
                update_balls_visuals,

                take_damage

            )
            .run_if(in_state(AppState::InGame))
        )

        .add_systems(PostUpdate, (
            // balls::manage_balls,
            reset_updated_flags.run_if(in_state(AppState::InGame))
        ))

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


        .insert_resource(LogText(Vec::new()))
        .insert_resource(Settings_players(2))
        .insert_resource(Settings_balls(true))
        .insert_resource(Settings_log(true))
        .insert_resource(Settings_camera_system(false))

        .add_event::<LogEvent>()
        .run();
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct CollectInput;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct ApplyInput;


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    InGame,
    PauseMenu,
    Reset
}



const SCENE_FILE_PATH: &str = "main.scn.ron";
// const BALL_SIZE: f32 = 20.;
// const SWING_RIGHT: bool = true;
// const ANIM_LENGTH: f32 = 0.5;
// const PADDLE_DISTANCE: f32 = 100.;
const DROP_HEIGHT: f32 = 200.;
// const ACC: f32 = 16.;
// const BALL_TIME: f32 = 2.;
const GRAVITY_SCALE: f32 = 5.;
// const JUMP_IMPULSE: f32 = 100.;



mod shapes {
    // re-use previous bevy?
    use bevy::prelude::*;
    use bevy::render::render_resource::AsBindGroup;
    use bevy::reflect::{TypeUuid, TypePath};
    use bevy::sprite::{Material2d, Material2dPlugin};

    // This is the struct that will be passed to your shader
    #[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
    #[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
    pub struct PieMaterial {
        #[uniform(0)]
        pub color: Color,
        #[uniform(0)]
        pub radius: f32,
        #[uniform(0)]
        pub percentage: f32
    }

    impl Default for PieMaterial {
        fn default() -> Self {
            Self {color: default(), radius: 0.5, percentage: 1.0}
        }
    }

    impl Material2d for PieMaterial {
        fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
            "custom_material.wgsl".into()
        }
    }

    pub struct ShapePlugin;

    impl Plugin for ShapePlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins(Material2dPlugin::<PieMaterial>::default());
        }
    }
}

use shapes::{
    ShapePlugin,
    PieMaterial,
};





// ------------------ setup





fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PieMaterial>>,

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


    commands.spawn(PlayerSensorBundle {
        player_sensor: PlayerSensor { despawn_on_enter: true },
        transform: Transform::from_xyz(-250., 20., 0.),
        ..default()
    });

    
    let mat_handle = materials.add(PieMaterial {
        color: Color::PURPLE,
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Quad::new(Vec2::new(50., 50.)).into()).into(),
        material: mat_handle.clone(),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });

    commands.spawn(
        DropOnMeRate(Timer::from_seconds(3., TimerMode::Repeating))
    );

    // don't need to. use this in a system yet
    commands.insert_resource(BallTexture(
        asset_server.load("icon.png")
    ));

}



fn spawn_players_system(
    mut commands: Commands, 
    settings: Res<Settings_players>, 
    asset_server: Res<AssetServer>,
    
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    /*
        goose png: each tile is is 237 by 201
        5 tiles

                default anchor is -0.19, -0.25

     */

    let texture_handle = asset_server.load("combined.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(237., 201.), 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(
        bundles::PlayerBundle {
            transform: Transform::from_xyz(-100., 0., 0.),            
            texture: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 0, 
                anchor: Anchor::Custom(Vec2::new(-0.19, -0.25)), 
                ..default()
            },
            ..default()
        }
    )
    .insert(AnimationState::Normal)
    .insert(InputMapWASD)
    .insert(Player1Marker);


    if settings.0 != 2 {return}

    commands.spawn(
        bundles::PlayerBundle {
            transform: Transform::from_xyz(100., 0., 0.),
            texture: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 0, 
                anchor: Anchor::Custom(Vec2::new(-0.25, 0.0)), 
                ..default()
            },
            ..default()
        }
    )
    .insert(AnimationState::Normal)
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
    const JUMP: KeyCode = KeyCode::Comma;
    const SWING: KeyCode = KeyCode::Period;
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

    // YEAH
    // does this work with analog input?
    h.direction = h.direction.normalize_or_zero();
    
    for mut holder in q.iter_mut() {
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
            if value.x >= 0. { Ok(Self::Right) } else { Ok(Self::Left) }
        } else {
            if value.y >= 0. { Ok(Self::Up) } else { Ok(Self::Down) }
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


fn get_frame_index(start_frame: usize, num_frames: f32, percent: f32) -> usize {
    start_frame + (percent * num_frames).floor() as usize
}

fn apply_input_system(
    time: Res<Time>,
    mut commands: Commands,
    mut players_q: Query<(
        Entity, 
        &InputHolder,
        &mut KinematicCharacterController,
        &Transform,
        &mut CharacterVelocity,
        Option<&mut OneShot>,
        Option<&Direction>,
        &mut TextureAtlasSprite
    )>,
    rapier_context: Res<RapierContext>
) {
    for (e, input, mut controller, transform, mut c_vel, o, d, mut sprite) in players_q.iter_mut() {
        let grav = -0.2;
        c_vel.0.y += grav;
        c_vel.0.y = c_vel.0.y.clamp(-10., 10.); // terminal velocity

        /* if swing is timed out then let it start again */
    
        if let Some(mut oneshot) = o {
            oneshot.timer.tick(time.delta());
            if oneshot.timer.just_finished() {
                commands.entity(e).remove::<OneShot>();

                // default index
                sprite.index = 0;
            }
            else {
                sprite.index = match d {
                    Some(Direction::Up) => 
                        get_frame_index(1, 2.0, oneshot.timer.percent()),
                    Some(Direction::Right) | Some(Direction::Left) => 
                        get_frame_index(3, 2.0, oneshot.timer.percent()),
                    _ => 0
                }
            }    
        }
        else {
            if let Ok(dir) = Direction::try_from(input.direction) {
                commands.entity(e).insert(dir);
            }

            /* get input */

            if input.swing {
                commands.entity(e).insert(
                    OneShot::from_seconds(0.5),
                );
            }

            /* flip sprite if not animating */

            if input.direction.x != 0. {
                sprite.flip_x = input.direction.x < 0.;
                sprite.anchor = Anchor::Custom(Vec2::new(input.direction.x.signum() * -0.19, -0.25))
            }
        }

        /* raycast to ground */
        if input.jump {
            if let Some((_entity, _toi)) = rapier_context.cast_ray(
                transform.translation.xy(), 
                Vec2::new(0., -1.), 
                60., 
                false, 
                QueryFilter::only_fixed()
            ) {
                c_vel.0.y = 10.;
            }
        }

        /* apply velocity to translation */
        // todo: apply delta
        controller.translation = Some(Vec2::new(
            3. * input.direction.x,
            c_vel.0.y
        ));
    }
}




/* Read the character controller collisions stored in the character controller’s output. */
fn modify_character_controller_slopes(
    mut characters: Query<(
        &KinematicCharacterControllerOutput, 
        &mut CharacterVelocity,
   )>,

   mut q_balls: Query<&mut Velocity, With<FromPlayer>>
) {
    /* apply translation back to velocity */
    
    for (output, mut c_vel) in characters.iter_mut() {
        c_vel.0.y = output.effective_translation.y;

        // let effective_vel = output.effective_translation.xy();
        // let effective_speed = effective_vel.x.abs();
        // update sound

        let impact = output.desired_translation.xy() - output.effective_translation.xy();

        for collision in output.collisions.iter() {
            if let Ok(mut velocity) = q_balls.get_mut(collision.entity) {
                velocity.linvel += impact;
            }
        }
    }
}

/*
    this runs on;y when the swing is animating
 */

fn apply_swing(
    p1_q: Query<(Entity, &Transform), With<Player1Marker>>,
    p2_q: Query<(Entity, &Transform), With<Player2Marker>>,
    /* query for additional components */
    mut players_q: Query<(
        &mut OneShot, 
        &Direction,
        &mut TextureAtlasSprite
    )>,
    mut balls_q: Query<(&Transform, &mut Velocity, &mut FromPlayer)>,
    mut gizmos: Gizmos,
) {
    let p1 = p1_q.single();
    let p2 = match p2_q.get_single() {
        Ok(p2) => p2,
        Err(_) => {
            info!("one player only!");
            return;
        }
    };

    let p1_to_p2 = p2.1.translation - p1.1.translation;

    for (e_player, translation, to_other) in [
        (p1.0, p1.1.translation, p1_to_p2), 
        (p2.0, p2.1.translation, -p1_to_p2)
    ] {
        if let Ok((mut o_s, pressed_direction, mut sprite)) = players_q.get_mut(e_player) {

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
                hit velocity: 300 / 400
                */

            let foot = Vec2::new(
                player_origin.x + d.x * 20.,
                player_origin.y - 50.
            );

            gizmos.circle_2d(foot, 10., Color::GREEN);

            /* flip sprite based on other's position */
            if *pressed_direction == Direction::Up {
                sprite.flip_x = to_other.x < 0.;
            }

            /* if swing-action is used up, don't process anymore */
            if o_s.used_up {continue}

            for (ball_transform, mut velocity, mut from_player) in balls_q.iter_mut() {
                if rect.contains(
                    ball_transform.translation.xy()
                ) {
                    from_player.0 = e_player;

                    /* don't remove the component but set it so it can't be used again */
                    o_s.used_up = true;

                    if *pressed_direction == Direction::Up {
                        velocity.linvel = 400. * Vec2::new(to_other.x.signum(), 0.);
                    } else {
                        velocity.linvel = (ball_transform.translation.xy() - foot).normalize_or_zero() * 300.;         
                    }
                }
            }
        }
    }
}


fn take_damage(
    q_player: Query<Entity, Added<HitByBall>>,
    mut log: EventWriter<LogEvent>,
    mut commands: Commands
) {
    for e in q_player.iter() {
        log.send(
            LogEvent(format!("you took damage\n"))
        );
        
        commands.entity(e).remove::<HitByBall>();
    }
}


// -------------------------------
// workers
// ---


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
        let len = text.sections.len();
        if len > 9 {
            text.sections.drain(0..(len - 9));
        }
    }
}


/*
    pre-update:
        update sensors/reactive components
    
    update:
        do stuff

    post-update:
        reset reactive components

    
    you have the option to use change detection

 */

fn update_sensors(
    mut collision_events: EventReader<CollisionEvent>,
    //
    // mut ball_sensors: Query<(&mut BallSensor, &Transform)>,
    // player_sensors: Query<(&PlayerSensor, Entity)>,
    //
    ground: Query<&RigidBody>,
    balls: Query<(Entity, &FromPlayer)>,
    players: Query<&InputHolder>,

    mut log: EventWriter<LogEvent>,
    mut commands: Commands
) {

    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(e_1, e_2, _flags) = *collision_event {
            for (entity, other_entity) in [(e_1, e_2), (e_2, e_1)] {

                // if rigidbody is static/fixed then...
                if let Ok(RigidBody::Fixed) = ground.get(entity) {

                    // log.send(
                    //     LogEvent(format!("{:?} {:?}\n", entity, balls.get(other_entity)))
                    // );

                    // println!("{:?} {:?}", entity, balls.get(other_entity));


                    if balls.get(other_entity).is_ok() {
                        commands.entity(other_entity).insert(HasTouchedGround);
                    }
                }

                if balls.get(entity).is_ok() && players.get(other_entity).is_ok() {
                    log.send(
                        LogEvent(format!("ball {:?} and player\n", entity))
                    );

                    commands.entity(other_entity).insert(HitByBall);
                }
            }
        }
    }

                // // check if sensor
                // if flags == CollisionEventFlags::SENSOR {
                //     // check if e1 is playersensor and e2 is player
                // }


    // for collision_event in collision_events.iter() {
    //     if let CollisionEvent::Started(entity_1, entity_2, _flags) = *collision_event {
    //         /* for each entity check if is sensor first, then if other is required type */
    //         for (entity, other_entity) in [(entity_1, entity_2), (entity_2, entity_1)] {

    //             // if let Ok((mut sensor, _)) = ball_sensors.get_mut(entity) {
    //             //     if let Ok(_from_player) = balls.get(other_entity) {
    //             //         // ball hit
    //             //         sensor.hit_on_last_update = true;
    //             //         log.send(
    //             //             LogEvent(format!("ball sensor: {:?} hit from: {:?}\n", entity, other_entity))
    //             //         );
    //             //     }
    //             // }
    //             // else if let Ok((_sensor, _sensor_e)) = player_sensors.get(entity) {
    //             //     if let Ok(_) = players.get(other_entity) {
    //             //     }
    //             // }
    //         }
    //     }
    // }
}


fn reset_updated_flags(
    mut sensors_q: Query<&mut BallSensor>
) {
    for mut b in sensors_q.iter_mut() {
        b.hit_on_last_update = false;
    }
}


fn update_balls_visuals(
    mut q_balls: Query<&mut Visibility, Added<HasTouchedGround>>
) {
    for mut vis in q_balls.iter_mut() {
        *vis = Visibility::Hidden;
    }
}