use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    Usd,
    Idr,
}

impl Currency {
    /// Returns the number of decimal places for display.
    #[must_use]
    pub const fn decimal_places(&self) -> u32 {
        match self {
            Self::Usd => 2,
            Self::Idr => 0,
        }
    }

    /// Format a float amount as a display string.
    #[must_use]
    pub fn format_amount(&self, stored: f64) -> String {
        match self {
            Self::Usd => format!("{stored:.2}"),
            Self::Idr => format!("{stored:.0}"),
        }
    }

    pub fn parse_from_str(s: &str) -> Option<Self> {
        match s {
            "USD" => Some(Self::Usd),
            "IDR" => Some(Self::Idr),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Usd => "USD",
            Self::Idr => "IDR",
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    User,
    Admin,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User => write!(f, "USER"),
            Self::Admin => write!(f, "ADMIN"),
        }
    }
}

impl Role {
    pub fn parse_str(s: &str) -> Option<Self> {
        match s {
            "USER" => Some(Self::User),
            "ADMIN" => Some(Self::Admin),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserStatus {
    Active,
    Inactive,
    Disabled,
    Locked,
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "ACTIVE"),
            Self::Inactive => write!(f, "INACTIVE"),
            Self::Disabled => write!(f, "DISABLED"),
            Self::Locked => write!(f, "LOCKED"),
        }
    }
}

impl UserStatus {
    pub fn parse_str(s: &str) -> Option<Self> {
        match s {
            "ACTIVE" => Some(Self::Active),
            "INACTIVE" => Some(Self::Inactive),
            "DISABLED" => Some(Self::Disabled),
            "LOCKED" => Some(Self::Locked),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryType {
    Expense,
    Income,
}

impl fmt::Display for EntryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expense => write!(f, "expense"),
            Self::Income => write!(f, "income"),
        }
    }
}

impl EntryType {
    pub fn parse_str(s: &str) -> Option<Self> {
        match s {
            "expense" => Some(Self::Expense),
            "income" => Some(Self::Income),
            _ => None,
        }
    }
}

/// Supported units of measure.
pub const SUPPORTED_UNITS: &[&str] = &[
    "liter", "ml", "kg", "g", "km", "meter", "gallon", "lb", "oz", "mile", "piece", "hour",
];

pub fn is_supported_unit(unit: &str) -> bool {
    SUPPORTED_UNITS.contains(&unit)
}

#[cfg(test)]
mod tests {
    // `unwrap`/`expect`/`panic`, exact float comparisons, and unseparated
    // numeric literals are idiomatic in tests.
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::float_cmp,
        clippy::unreadable_literal
    )]

    use super::*;

    #[test]
    fn currency_usd_decimal_places() {
        assert_eq!(Currency::Usd.decimal_places(), 2);
    }

    #[test]
    fn currency_idr_decimal_places() {
        assert_eq!(Currency::Idr.decimal_places(), 0);
    }

    #[test]
    fn currency_format_usd() {
        assert_eq!(Currency::Usd.format_amount(10.50), "10.50");
        assert_eq!(Currency::Usd.format_amount(3000.00), "3000.00");
    }

    #[test]
    fn currency_format_idr() {
        assert_eq!(Currency::Idr.format_amount(150000.0), "150000");
    }

    #[test]
    fn currency_parse_from_str() {
        assert_eq!(Currency::parse_from_str("USD"), Some(Currency::Usd));
        assert_eq!(Currency::parse_from_str("IDR"), Some(Currency::Idr));
        assert_eq!(Currency::parse_from_str("EUR"), None);
        assert_eq!(Currency::parse_from_str("US"), None);
    }

    #[test]
    fn supported_units() {
        assert!(is_supported_unit("liter"));
        assert!(is_supported_unit("gallon"));
        assert!(!is_supported_unit("fathom"));
    }

    #[test]
    fn role_display() {
        assert_eq!(Role::User.to_string(), "USER");
        assert_eq!(Role::Admin.to_string(), "ADMIN");
    }

    #[test]
    fn user_status_display() {
        assert_eq!(UserStatus::Active.to_string(), "ACTIVE");
        assert_eq!(UserStatus::Locked.to_string(), "LOCKED");
    }

    #[test]
    fn entry_type_display() {
        assert_eq!(EntryType::Expense.to_string(), "expense");
        assert_eq!(EntryType::Income.to_string(), "income");
    }
}
