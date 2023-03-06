use std::default::Default;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RunType {
    #[default]
    SinglePoint,
    GeometryOptimisation,
}

impl RunType {
    pub const ALL: [RunType; 2] = [RunType::SinglePoint, RunType::GeometryOptimisation];
}

impl std::fmt::Display for RunType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RunType::SinglePoint => "Single Point",
                RunType::GeometryOptimisation => "Geometry Optimisation",
            }
        )
    }
}
