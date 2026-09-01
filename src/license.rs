/// OSI approved license categorized as (popular / strong community)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum License {
    /// Apache 2.0
    Apache2_0,
    /// Common Development and Distribution License 1.0
    Cddl1_0,
    /// Eclipse Public License 2.0
    Epl2_0,
    /// GNU General Public License 2
    Gpl2_0,
    /// GNU General Public License 3
    Gpl3_0,
    /// GNU Lesser General Public License version 2.1
    Lgpl2_1,
    /// GNU Lesser General Public License version 3
    Lgpl3_0,
    /// GNU Library General Public License version 2
    Lgpl2_0,
    /// Mozilla Public License 2.0
    Mpl2_0,
    /// 2-Clause BSD
    Bsd2Clause,
    /// 3-Clause BSD
    Bsd3Clause,
    /// MIT 
    Mit,
}

impl License {
    /// Get the full name of license
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Apache2_0 => "Apache License, Version 2.0",
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
            Self::Mit => "The MIT License",
        }
    }

    /// Get the SPDX id associated with a license.
    /// Internally, `oseda check` will recognize one of these strings
    pub const fn spdx_id(&self) -> &'static str {
        match self {
            Self::Apache2_0 => "Apache-2.0",
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
            Self::Mit => "MIT",
        }
    }

}

impl std::fmt::Display for License{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.spdx_id())
    }
}