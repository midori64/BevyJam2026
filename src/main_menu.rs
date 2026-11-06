use bevy::{
    color::palettes::css::{BLACK, GRAY, RED, WHITE},
    picking::hover::Hovered,
    prelude::*,
    ui::{InteractionDisabled, Pressed},
    ui_widgets::{Activate, Button, observe},
};

use crate::screens::Screen;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MainMenuSet;

pub(crate) fn plugin(app: &mut App) {
    app.configure_sets(Update, MainMenuSet.run_if(in_state(Screen::MainMenu)))
        .add_systems(OnEnter(Screen::MainMenu), setup)
        .add_systems(Update, update_button.in_set(MainMenuSet));
}

#[derive(Component)]
struct StartGameButton;

fn setup(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(Screen::MainMenu),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            ..default()
        },
        children![(
            StartGameButton,
            button(),
            observe(
                |_activate: On<Activate>, mut next_screen: ResMut<NextState<Screen>>| {
                    next_screen.set(Screen::Gameplay)
                }
            ),
        ),],
    ));
}

fn button() -> impl Bundle {
    (
        Node {
            width: px(150),
            height: px(65),
            border: UiRect::all(px(5)),
            border_radius: BorderRadius::MAX,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Button,
        Hovered::default(),
        BorderColor::all(Color::BLACK),
        BackgroundColor(NORMAL_BUTTON),
        children![(
            Text::new("Start"),
            TextFont {
                font_size: FontSize::Px(33.0),
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    )
}

// TODO: When the button is released, its `Pressed` state stays `true` until
// the mouse is moved away. Seems like a Bevy bug?
fn update_button(
    mut buttons: Query<
        (
            Has<Pressed>,
            &Hovered,
            Has<InteractionDisabled>,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (
            Or<(
                Changed<Pressed>,
                Changed<Hovered>,
                Added<InteractionDisabled>,
            )>,
            With<StartGameButton>,
        ),
    >,
) {
    for (pressed, hovered, disabled, mut color, mut border_color) in &mut buttons {
        set_button_style(
            disabled,
            hovered.get(),
            pressed,
            &mut color,
            &mut border_color,
        );
    }
}

fn set_button_style(
    disabled: bool,
    hovered: bool,
    pressed: bool,
    color: &mut BackgroundColor,
    border_color: &mut BorderColor,
) {
    match (disabled, hovered, pressed) {
        // Disabled button
        (true, _, _) => {
            *color = NORMAL_BUTTON.into();
            border_color.set_all(GRAY);
        }

        // Pressed and hovered button
        (false, true, true) => {
            *color = PRESSED_BUTTON.into();
            border_color.set_all(RED);
        }

        // Hovered, unpressed button
        (false, true, false) => {
            *color = HOVERED_BUTTON.into();
            border_color.set_all(WHITE);
        }

        // Unhovered button (either pressed or not).
        (false, false, _) => {
            *color = NORMAL_BUTTON.into();
            border_color.set_all(BLACK);
        }
    }
}
