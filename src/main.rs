use bevy::{app::AppExit, prelude::*}; // Import AppExit for exiting the app

mod settings;

// This system will run once when the game starts and when you press ESC to exit the game
fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        info!("'ESC' currently pressed");
        app_exit_event_writer.send(AppExit); // Send an AppExit event
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                position: WindowPosition::Automatic, // Let the OS handle window position
                resizable: true,                     // Allow the window to be resized
                resolution: (settings::WINDOW_WIDTH, settings::WINDOW_HEIGHT).into(), // Set window dimensions (game + sidebar)
                title: settings::GAME_TITLE.into(),
                ..default()
            }),
            ..default()
        }),))
        .add_systems(Startup, setup)
        .run();
}

// This function runs once when the program starts
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
