use bevy::prelude::*;

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // add thing to app
        app.add_startup_system(add_people.system())
            .add_system(hello_world.system())
            .add_system(greet_people.system());
    }
}

struct Entity(u64);

struct Person;

struct Name(String);

struct Position {
    x: f32,
    y: f32,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("position: {:?}", transform.translation);
    }
}
fn hello_world() {
    println!("hello, world!");
}
fn add_people(commands: &mut Commands) {
    commands
        .spawn((Person, Name("Bob Bobson".to_string())))
        .spawn((Person, Name("Flip Flipper".to_string())))
        .spawn((Person, Name("Henny Henderson".to_string())));
}
fn greet_people(query: Query<&Name, With<Person>>) {
    for Name(name) in query.iter() {
        println!("hello {}!", name);
    }
}
