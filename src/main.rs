use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_ball))
        .add_systems(FixedUpdate, project_poistions)
        .run();
}
fn spawn_camera(mut commands: Commands) {
    commands
        .spawn((
                Camera2d,
                Transform::from_xyz(0., 0., 0.)
                ));
}

const BALL_SIZE: f32 = 20.;
const BALL_SHAPE: Circle = Circle::new(BALL_SIZE);
const BALL_COLOR: Color = Color::srgb(1.,0.,0.);

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    // `Assets::add` will load these into memory and return a `Handle` (an ID)
    // to these assets. When all references to this `Handle` are cleaned up
    // the asset is cleaned up.
    let mesh = meshes.add(BALL_SHAPE);
    let material = materials.add(BALL_COLOR);

    println!("Spawning Ball...");

    // Here we are using `spawn` instead of `spawn_empty` followed by an
    // `insert` they mean the same thing, letting us spawn many components on a
    // entity at once.
    commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material)));
}

fn project_poistions(mut positionables: Query<(&mut Transform, &Position)>) {
    // Here we are iterating over the query to get the
    // components from our game world
    for (mut transform, position) in &mut positionables {
        // Extend is going to turn this from a Vec2 to Vec 3
        transform.translation = position.0.extend(0.);
    }
}

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component)]
#[require(Position)]
struct Ball;
