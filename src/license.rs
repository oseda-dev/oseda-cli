use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

/// OSI approved license categorized as (popular / strong community)
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, EnumIter, IntoStaticStr, EnumString, Serialize, Deserialize,
)]
#[serde(into = "&'static str", try_from = "String")]
pub enum License {
    // strum serialize is compatible with serde trait here
    // this will basically allow a complex internal license representation
    // but with ease of de/serialization with just the spdx id
    #[strum(serialize = "Apache-2.0")]
    Apache2_0,
    #[strum(serialize = "MIT")]
    Mit,
    #[strum(serialize = "CDDL-1.0")]
    Cddl1_0,
    #[strum(serialize = "EPL-2.0")]
    Epl2_0,
    #[strum(serialize = "GPL-2.0")]
    Gpl2_0,
    #[strum(serialize = "GPL-3.0")]
    Gpl3_0,
    #[strum(serialize = "LGPL-2.1")]
    Lgpl2_1,
    #[strum(serialize = "LGPL-3.0")]
    Lgpl3_0,
    #[strum(serialize = "LGPL-2.0")]
    Lgpl2_0,
    #[strum(serialize = "MPL-2.0")]
    Mpl2_0,
    #[strum(serialize = "BSD-2-Clause")]
    Bsd2Clause,
    #[strum(serialize = "BSD-3-Clause")]
    Bsd3Clause,
}

impl License {
    /// Get the full name of license. I'm not sure if this really ever be useful, but i figured its best to include it
    pub const fn name(&self) -> &str {
        match self {
            Self::Apache2_0 => "Apache License, Version 2.0",
            Self::Mit => "The MIT License",
            Self::Cddl1_0 => "Common Development and Distribution License 1.0",
            Self::Epl2_0 => "Eclipse Public License version 2.0",
            Self::Gpl2_0 => "GNU General Public License version 2",
            Self::Gpl3_0 => "GNU General Public License version 3",
            Self::Lgpl2_1 => "GNU Lesser General Public License version 2.1",
            Self::Lgpl3_0 => "GNU Lesser General Public License version 3",
            Self::Lgpl2_0 => "GNU Library General Public License version 2",
            Self::Mpl2_0 => "Mozilla Public License 2.0",
            Self::Bsd2Clause => "The 2-Clause BSD License",
            Self::Bsd3Clause => "The 3-Clause BSD License",
        }
    }

    /// Get the SPDX id associated with a license.
    /// Internally, `oseda check` will recognize one of these strings
    pub const fn spdx_id(&self) -> &'static str {
        match self {
            Self::Apache2_0 => "Apache-2.0",
            Self::Mit => "MIT",
            Self::Cddl1_0 => "CDDL-1.0",
            Self::Epl2_0 => "EPL-2.0",
            Self::Gpl2_0 => "GPL-2.0",
            Self::Gpl3_0 => "GPL-3.0",
            Self::Lgpl2_1 => "LGPL-2.1",
            Self::Lgpl3_0 => "LGPL-3.0",
            Self::Lgpl2_0 => "LGPL-2.0",
            Self::Mpl2_0 => "MPL-2.0",
            Self::Bsd2Clause => "BSD-2-Clause",
            Self::Bsd3Clause => "BSD-3-Clause",
        }
    }
}

impl std::fmt::Display for License {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.spdx_id())
    }
}

// useful for oseda check attempting to parse a license spdx id
impl TryFrom<String> for License {
    type Error = strum::ParseError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}
