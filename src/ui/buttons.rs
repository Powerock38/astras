use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
#[require(
    Button,
    Node(|| Node {
        padding: UiRect::all(Val::Px(5.0)),
        border: UiRect::all(Val::Px(2.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }),
    BorderColor(|| BorderColor(Color::BLACK)),
    BackgroundColor(|| BackgroundColor(NORMAL_BUTTON))
)]
pub struct UiButton;

pub fn update_ui_buttons(
    mut q_interactions: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut q_interactions {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = bevy::color::palettes::css::RED.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
