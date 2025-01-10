use bevy::prelude::*;
use crate::widgets::text_input::{TextInputSize, TextInputColor};
use crate::widgets::color::*;

pub const PLACEHOLDER_COLOR: Color = Color::srgba(0.749, 0.749, 0.749, 1.0);
pub const TEXT_INPUT_VALUE_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.922);

pub fn default_input_node() -> Node {
    Node {
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Center,
        padding: UiRect {
            left: Val::Px(15.0),
            right: Val::Px(15.0),
            top: Val::Px(5.0),
            bottom: Val::Px(5.0),
        },
        margin: UiRect {
            top: Val::Px(5.0),
            right: Val::Px(0.0),
            left: Val::Px(0.0),
            bottom: Val::Px(5.0),
        },
        height: Val::Auto,
        width: Val::Percent(100.0),
        border: UiRect::all(Val::Px(2.0)),
        ..default()
    }
}

pub fn outlined_border_width() -> UiRect {
    UiRect::all(Val::Px(2.0))
}

pub fn underlined_border_width() -> UiRect {
    UiRect {
        left: Val::Px(0.0),
        right: Val::Px(0.0),
        top: Val::Px(0.0),
        bottom: Val::Px(2.0),
    }
}

pub fn outlined_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Px(6.0))
}

pub fn underlined_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Px(0.0))
}

pub fn round_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Percent(50.0))
}

pub fn rectangle_border_radius() -> BorderRadius {
    BorderRadius::all(Val::Px(0.0))
}

pub fn get_text_size(size: &TextInputSize) -> f32 {
    let size_small = 16.0;
    let size_normal = 20.0;
    let size_large = 24.0;

    let text_size = match size {
        TextInputSize::Small => size_small,
        TextInputSize::Normal => size_normal,
        TextInputSize::Large => size_large,
    };
    text_size
}

pub fn get_input_background_color(color: &TextInputColor) -> BackgroundColor {
    match color {
        TextInputColor::Primary => BackgroundColor(PRIMARY_DARK_COLOR),
        TextInputColor::Secondary => BackgroundColor(SECONDARY_DARK_COLOR),
        TextInputColor::Success => BackgroundColor(SUCCESS_DARK_COLOR),
        TextInputColor::Danger => BackgroundColor(DANGER_DARK_COLOR),
        TextInputColor::Warning => BackgroundColor(WARNING_DARK_COLOR),
        TextInputColor::Info => BackgroundColor(INFO_DARK_COLOR),
        _ => BackgroundColor::default()
    }
}

pub fn get_input_border_color(color: &TextInputColor) -> BorderColor {
    match color {
        TextInputColor::Primary => BorderColor(PRIMARY_COLOR),
        TextInputColor::Secondary => BorderColor(SECONDARY_COLOR),
        TextInputColor::Success => BorderColor(SUCCESS_COLOR),
        TextInputColor::Danger => BorderColor(DANGER_COLOR),
        TextInputColor::Warning => BorderColor(WARNING_COLOR),
        TextInputColor::Info => BorderColor(INFO_COLOR),
        _ => BorderColor(Color::srgba(0.902, 0.902, 0.902, 0.922))
    }
}
