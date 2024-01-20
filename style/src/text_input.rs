//! Change the appearance of a text input.
use iced_core::{Background, Border, Color};

/// The appearance of a text input.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Default)]
pub struct Appearance {
    /// The [`Background`] of the text input.
    pub background: Background,
    /// The [`Border`] of the text input.
    pub border: Border,
    /// The icon [`Color`] of the text input.
    pub icon_color: Color,
}

/// A set of rules that dictate the style of a text input.
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the style of an active text input.
    fn active(&self, style: &Self::Style) -> Appearance;

    /// Produces the style of a focused text input.
    fn focused(&self, style: &Self::Style) -> Appearance;

    /// Produces the [`Color`] of the placeholder of a text input.
    fn placeholder_color(&self, style: &Self::Style) -> Color;

    /// Produces the [`Color`] of the value of a text input.
    fn value_color(&self, style: &Self::Style) -> Color;

    /// Produces the [`Color`] of the value of a disabled text input.
    fn disabled_color(&self, style: &Self::Style) -> Color;

    /// Produces the [`Color`] of the selection of a text input.
    fn selection_color(&self, style: &Self::Style) -> Color;

    /// Produces the style of an hovered text input.
    fn hovered(&self, style: &Self::Style) -> Appearance {
        self.focused(style)
    }

    /// Produces the style of a disabled text input.
    fn disabled(&self, style: &Self::Style) -> Appearance;
}
