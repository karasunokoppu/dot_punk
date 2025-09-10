#[derive(Default, PartialEq, Debug, Clone)]
pub enum Direction {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    #[default]
    Top,
    Bottom,
    Right,
    Left,
}
