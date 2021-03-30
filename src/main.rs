use bevy::app::AppExit;
use bevy::prelude::*;
use kurinji::{Kurinji, KurinjiPlugin};
use std::fs;
struct GreetTimer(Timer);
struct Entity(u64);
struct Person;
struct Name(String);
struct Position {
    x: f32,
    y: f32,
}
pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // add thing to app
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_plugin(KurinjiPlugin::default())
            .add_startup_system(setup.system())
            .add_startup_system(add_people.system())
            .add_system(greet_people_system.system())
            .add_system(action_system.system());
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
fn setup(mut kurinji: ResMut<Kurinji>) {
    let binding_json =
        fs::read_to_string("./config/bindings.json").expect("Error! could not open config file");
    kurinji.set_bindings_with_json(&binding_json);
}
fn add_people(commands: &mut Commands) {
    commands
        .spawn((Person, Name("Bob Bobson".to_string())))
        .spawn((Person, Name("Flip Flipper".to_string())))
        .spawn((Person, Name("Henny Henderson".to_string())));
}
fn action_system(kurinji: Res<Kurinji>, mut app_exit_events: ResMut<Events<AppExit>>) {
    if kurinji.is_action_active("MOVE_LEFT") {
        println!("move left");
    }
    if kurinji.is_action_active("MOVE_RIGHT") {
        println!("move right");
    }
    if kurinji.is_action_active("MOVE_FORWARD") {
        println!("move forward");
    }
    if kurinji.is_action_active("MOVE_BACKWARD") {
        println!("move backward");
    }
    if kurinji.is_action_active("QUIT_APP") {
        println!("Quiting...");
        app_exit_events.send(AppExit);
    }
}
fn greet_people_system(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }
    for Name(name) in query.iter() {
        println!("hello {}!", name);
    }
}
