#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Align {
    #[default]
    Min,
    Center,
    Max,
}

impl Align {
    pub const LEFT: Self = Self::Min;
    pub const TOP: Self = Self::Min;
    pub const CENTER: Self = Self::Center;
    pub const RIGHT: Self = Self::Max;
    pub const BOTTOM: Self = Self::Max;
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Align2 {
    pub x: Align,
    pub y: Align,
}

impl Align2 {
    pub const LEFT_TOP: Self = Self {
        x: Align::LEFT,
        y: Align::TOP,
    };

    pub const CENTER_TOP: Self = Self {
        x: Align::CENTER,
        y: Align::TOP,
    };

    pub const RIGHT_TOP: Self = Self {
        x: Align::RIGHT,
        y: Align::TOP,
    };

    pub const LEFT_CENTER: Self = Self {
        x: Align::LEFT,
        y: Align::CENTER,
    };

    pub const CENTER_CENTER: Self = Self {
        x: Align::CENTER,
        y: Align::CENTER,
    };

    pub const RIGHT_CENTER: Self = Self {
        x: Align::RIGHT,
        y: Align::CENTER,
    };

    pub const LEFT_BOTTOM: Self = Self {
        x: Align::LEFT,
        y: Align::BOTTOM,
    };

    pub const CENTER_BOTTOM: Self = Self {
        x: Align::CENTER,
        y: Align::BOTTOM,
    };

    pub const RIGHT_BOTTOM: Self = Self {
        x: Align::RIGHT,
        y: Align::BOTTOM,
    };
}
