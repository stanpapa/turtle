use std::default::Default;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HFType {
    #[default]
    RHF,
    UHF,
    ROHF,
    CASSCF,
}

impl HFType {
    pub const ALL: [HFType; 4] = [HFType::RHF, HFType::UHF, HFType::ROHF, HFType::CASSCF];
}

impl std::fmt::Display for HFType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HFType::RHF => "RHF",
                HFType::UHF => "UHF",
                HFType::ROHF => "ROHF",
                HFType::CASSCF => "CASSCF",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CorrelationMethod {
    #[default]
    None,
    CID,
    CISD,
    CISDT,
    CCD,
    CCSD,
    CCSDT,
}

impl CorrelationMethod {
    pub const ALL: [CorrelationMethod; 7] = [
        CorrelationMethod::None,
        CorrelationMethod::CID,
        CorrelationMethod::CISD,
        CorrelationMethod::CISDT,
        CorrelationMethod::CCD,
        CorrelationMethod::CCSD,
        CorrelationMethod::CCSDT,
    ];
}

impl std::fmt::Display for CorrelationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CorrelationMethod::None => "None",
                CorrelationMethod::CID => "CID",
                CorrelationMethod::CISD => "CISD",
                CorrelationMethod::CISDT => "CISDT",
                CorrelationMethod::CCD => "CCD",
                CorrelationMethod::CCSD => "CCSD",
                CorrelationMethod::CCSDT => "CCSDT",
            }
        )
    }
}
