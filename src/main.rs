use bevy::prelude::*;
use bevy_rapier2d::physics::{RapierConfiguration, RapierPhysicsPlugin};
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};
use bevy_rapier2d::render::RapierRenderPlugin;

fn main() {
    App::build()
        .add_event::<SpawnEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_physics.system())
        .add_system(my_simple_system.system())
        .add_system(my_simple_system2.system())
        .run();
}

fn setup_graphics(commands: &mut Commands, mut configuration: ResMut<RapierConfiguration>) {
    configuration.scale = 10.0;

    commands
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(1000.0, 100.0, 2000.0)),
            ..Default::default()
        })
        .spawn(Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            ..Camera2dBundle::default()
        });
}

fn setup_physics(commands: &mut Commands) {
    /*
     * Ground
     */
    let ground_size = 10.0;

    let rigid_body = RigidBodyBuilder::new_static();
    let collider = ColliderBuilder::cuboid(ground_size, 1.2);
    commands.spawn((rigid_body, collider));

    let rigid_body = RigidBodyBuilder::new_static()
        .rotation(std::f32::consts::FRAC_PI_2)
        .translation(ground_size, ground_size * 2.0);
    let collider = ColliderBuilder::cuboid(ground_size * 2.0, 1.2);
    commands.spawn((rigid_body, collider));

    let body = RigidBodyBuilder::new_static()
        .rotation(std::f32::consts::FRAC_PI_2)
        .translation(-ground_size, ground_size * 2.0);
    let collider = ColliderBuilder::cuboid(ground_size * 2.0, 1.2);
    commands.spawn((body, collider));

    /*
     * Create the cubes
     */
    let num = 1;
    let rad = 0.5;

    let shift = rad * 2.0;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift / 2.0;

    for i in 0..num {
        for j in 0usize..num * 5 {
            let x = i as f32 * shift - centerx;
            let y = j as f32 * shift + centery + 2.0+10.0;

            // Build the rigid body.
            let body = RigidBodyBuilder::new_dynamic().translation(x, y);
            let collider = ColliderBuilder::cuboid(rad, rad).density(1.0);
            commands.spawn((body, collider));
        }
    }
}

#[derive(Default)]
struct SpawnEvent;

fn my_simple_system2(commands: &mut Commands, events: Res<Events<SpawnEvent>>, mut event_reader: Local<EventReader<SpawnEvent>>) {
    println!("What?");
    for _ev in event_reader.iter(&events) {
        let body = RigidBodyBuilder::new_dynamic().translation(0., 30.);
        let collider = ColliderBuilder::cuboid(0.5, 0.5).density(1.0);
        commands.spawn((body, collider));
    }
}

fn my_simple_system(mut ev_spawn: ResMut<Events<SpawnEvent>>, keys: Res<Input<KeyCode>>) {
    // Keyboard input
    if keys.just_pressed(KeyCode::Space) {
        ev_spawn.send(SpawnEvent)
    }
}