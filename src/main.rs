// Import necessary modules from Bevy
use bevy::{app::AppExit, prelude::*};

// Import custom settings from the settings module
mod settings;

fn main() {
    // Create a new Bevy app
    App::new()
        // Insert a resource to set the clear color of the window
        .insert_resource(ClearColor(Color::LIME_GREEN))
        // Add default Bevy plugins and configure the primary window
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                position: WindowPosition::Automatic, // Let the OS handle window position
                resizable: true,                     // Allow the window to be resized
                resolution: (settings::WINDOW_WIDTH, settings::WINDOW_HEIGHT).into(), // Set window dimensions (game + sidebar)
                title: settings::GAME_TITLE.into(), // Set the window title
                ..default()
            }),
            ..default()
        }),))
        // Add startup and update systems
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input_system)
        // Run the Bevy app
        .run();
}

// This function runs once when the program starts
fn setup(mut commands: Commands) {
    // Spawn a 2D camera entity bundle
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 1.0)), // Adjust the z-coordinate to ensure the rectangle is visible
        ..Default::default()
    });

    // Spawn a rectangle entity bundle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(settings::CELL_SIZE, settings::CELL_SIZE)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(
                -settings::WINDOW_WIDTH / 2.0, // Set the x-coordinate to position the rectangle on the left side of the window
                settings::WINDOW_HEIGHT / 2.0, // Set the y-coordinate to position the rectangle at the top of the window
                0.0,                           // Set the z-coordinate
            ),
            ..Default::default()
        },
        ..Default::default()
    });
}

// This system will run once when the game starts and when you press ESC to exit the game
fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        info!("'ESC' currently pressed");
        app_exit_event_writer.send(AppExit); // Send an AppExit event to exit the app
    }
}
