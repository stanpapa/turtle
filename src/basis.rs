use std::default::Default;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BasisSet {
    #[default]
    SVP,
    DZP,
    TZVP,
}

impl BasisSet {
    pub const ALL: [BasisSet; 3] = [BasisSet::SVP, BasisSet::DZP, BasisSet::TZVP];
}

impl std::fmt::Display for BasisSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BasisSet::SVP => "SVP",
                BasisSet::DZP => "DZP",
                BasisSet::TZVP => "TZVP",
            }
        )
    }
}
