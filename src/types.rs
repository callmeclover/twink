use std::hash::Hash;

#[derive(Debug, Hash)]
pub enum Order {
    Ascending,
    Descending,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ascending => "Ascending",
                Self::Descending => "Descending",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Color {
    #[default]
    Black,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
}

impl Color {
    pub const ALL: &'static [Color] = &[
        Color::Black,
        Color::Red,
        Color::Orange,
        Color::Yellow,
        Color::Green,
        Color::Blue,
        Color::Purple,
    ];
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Black => "Black",
            Self::Red => "Red",
            Self::Orange => "Orange",
            Self::Yellow => "Yellow",
            Self::Green => "Green",
            Self::Blue => "Blue",
            Self::Purple => "Purple",
        })
    }
}

impl From<Color> for iced::Color {
    fn from(value: Color) -> Self {
        match value {
            Color::Black => iced::Color::from_rgb8(0, 0, 0),
            Color::Red => iced::Color::from_rgb8(220, 50, 47),
            Color::Orange => iced::Color::from_rgb8(203, 75, 22),
            Color::Yellow => iced::Color::from_rgb8(181, 137, 0),
            Color::Green => iced::Color::from_rgb8(133, 153, 0),
            Color::Blue => iced::Color::from_rgb8(38, 139, 210),
            Color::Purple => iced::Color::from_rgb8(108, 113, 196),
        }
    }
}

#[derive(Clone, Debug, Eq)]
pub struct Item {
    pub name: String,
    pub color: Color,
}

impl Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl From<&str> for Item {
    fn from(s: &str) -> Self {
        Self {
            name: s.to_owned(),
            color: Color::default(),
        }
    }
}
