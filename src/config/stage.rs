use std::fmt;
use anyhow::Result;

#[derive(Debug, Clone, Default, PartialEq)]

pub enum Stage{
    Local,
    #[default]
    Development,
    Production,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stage: &str = match self {
            Stage::Local => "local",
            Stage::Development => "development",
            Stage::Production => "production",
        };
        write!(f, "{}", stage)
    }
}

impl Stage {
    pub fn from_str(stage: &str) ->Result<Self> {
        match stage {
            "local" => Ok(Self::Local),
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            _ => Err(anyhow::anyhow!("Invalid stage")),
        }
    }
}