use iced_graphics::Color;
use iced_style::{button, pick_list};

const SURFACE: Color = Color::from_rgb(
    0x30 as f32 / 255.0,
    0x34 as f32 / 255.0,
    0x3B as f32 / 255.0,
);

pub struct PickListStyle;

impl iced_style::pick_list::StyleSheet for PickListStyle {
    type Style = iced_style::Theme;

    fn active(&self, _style: &Self::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: Color::WHITE,
            background: iced_graphics::Background::Color(Color::BLACK),
            placeholder_color: Color::WHITE,
            border_radius: 2.0,
            border_width: 1.0,
            border_color: SURFACE,
            icon_size: 0.7,
        }
    }

    fn hovered(&self, _style: &Self::Style) -> pick_list::Appearance {
        let accent = Color::from_rgba8(160, 81, 255, 1.0);
        pick_list::Appearance {
            text_color: Color::WHITE,
            background: iced_graphics::Background::Color(Color::BLACK),
            // background: iced_graphics::Background::Color(Color::from_rgb8(42, 42, 42)),
            placeholder_color: Color::WHITE,
            border_radius: 2.0,
            border_width: 1.0,
            // border_color: Color::from_rgb8(42, 42, 42),
            border_color: accent,
            icon_size: 0.7,
        }
    }
}

pub struct MenuStyle;

impl iced_style::menu::StyleSheet for MenuStyle {
    type Style = iced_style::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced_style::menu::Appearance {
        let accent = Color::from_rgba8(160, 81, 255, 1.0);
        iced_style::menu::Appearance {
            text_color: Color::WHITE,
            background: iced_graphics::Background::Color(Color::BLACK),
            border_width: 1.0,
            border_radius: 0.0,
            border_color: SURFACE,
            selected_text_color: Color::WHITE,
            selected_background: iced_graphics::Background::Color(accent),
        }
    }
}

pub struct ButtonStyle;

impl iced_style::button::StyleSheet for ButtonStyle {
    type Style = iced_style::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: Color::WHITE,
            border_color: SURFACE,
            border_width: 1.0,
            background: Some(iced_graphics::Background::Color(Color::BLACK)),
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        let accent = Color::from_rgba8(160, 81, 255, 1.0);
        button::Appearance {
            text_color: Color::WHITE,
            border_color: accent,
            border_width: 1.0,
            background: Some(iced_graphics::Background::Color(Color::BLACK)),
            ..Default::default()
        }
    }
}

pub struct CheckboxStyle;

impl iced_style::checkbox::StyleSheet for CheckboxStyle {
    type Style = iced_style::Theme;

    fn active(&self, _style: &Self::Style, is_checked: bool) -> iced_style::checkbox::Appearance {
        let active = Color::from_rgba8(160, 81, 255, 1.0);
        iced_style::checkbox::Appearance {
            background: if is_checked { active } else { SURFACE }.into(),
            text_color: Some(Color::WHITE),
            checkmark_color: Color::WHITE,
            border_radius: 2.0,
            border_width: 1.0,
            border_color: active,
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> iced_style::checkbox::Appearance {
        let active = Color::from_rgba8(160, 81, 255, 1.0);
        iced_style::checkbox::Appearance {
            background: Color {
                a: 0.8,
                ..if is_checked { active } else { SURFACE }
            }
            .into(),
            ..self.active(style, is_checked)
        }
    }
}