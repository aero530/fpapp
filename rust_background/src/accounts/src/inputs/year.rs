//! User input year (date) values

use serde::{Deserialize, Serialize};

use super::settings;
use crate::Dates;

/// Options for strings on year inputs
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum YearEvalType {
    StartIn,
    EndIn,
    StartOut,
    EndOut,
}

/// Struct to hold info about computed year values
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct YearComputation {
    base: YearSuggestion,
    delta: i32,
}

/// These values can be input as constants or as computed values (strings)
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum YearInput {
    /// Calculated value based on suggested options
    Calculate(YearComputation),
    /// Suggested values
    Suggested(YearSuggestion),
    /// Constant value
    ConstantInt(u32),
}

impl YearInput {
    pub fn value(
        &self,
        settings: &settings::Settings,
        linked_dates: Option<Dates>,
        eval_type: YearEvalType,
    ) -> u32 {
        match self {
            Self::Calculate(input) => {
                (input.base.value(settings, linked_dates, eval_type) as i32 + input.delta) as u32
            }
            Self::Suggested(input) => input.value(settings, linked_dates, eval_type),
            Self::ConstantInt(input) => *input,
        }
    }
}

/// Options for strings on year inputs
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum YearSuggestion {
    /// Start of simulation
    //#[serde(rename(deserialize="yearStart"))]
    YearStart,
    /// When you plan to retire
    //#[serde(rename="yearRetire")]
    YearRetire,
    /// When you plan to die
    //#[serde(rename="yearDie")]
    YearDie,
    /// Last year of the simulation
    //#[serde(rename="yearEnd")]
    YearEnd,
    /// Pull date from linked account
    IncomeLink,
}

impl YearSuggestion {
    pub fn value(
        &self,
        settings: &settings::Settings,
        linked_dates: Option<Dates>,
        eval_type: YearEvalType,
    ) -> u32 {
        match self {
            Self::YearStart => settings.year_start(),
            Self::YearRetire => settings.year_retire(),
            Self::YearDie => settings.year_die(),
            Self::YearEnd => settings.year_end(),
            Self::IncomeLink => match eval_type {
                YearEvalType::StartIn => linked_dates.unwrap().year_in.unwrap().start,
                YearEvalType::EndIn => linked_dates.unwrap().year_in.unwrap().end,
                YearEvalType::StartOut => linked_dates.unwrap().year_out.unwrap().start,
                YearEvalType::EndOut => linked_dates.unwrap().year_out.unwrap().end,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::simulation::YearRange;
    use super::*;

    fn test_settings_values() -> settings::Settings {
        settings::Settings {
            age_retire: 50,
            age_die: 100,
            year_born: 1980,
            year_start: 2000,
            inflation_base: 5.0,
            tax_income: 20.0,
            tax_capital_gains: 10.0,
            retirement_cost_of_living: 80.0,
            ssa: settings::SsaSettings {
                breakpoints: settings::Span {
                    low: 30000_f64,
                    high: 40000_f64,
                },
                taxable_income_percentage: settings::Span {
                    low: 50_f64,
                    high: 80_f64,
                },
            },
        }
    }

    #[test]
    fn year_input_constant() {
        let settings = test_settings_values();
        let w1 = YearInput::ConstantInt(1900);
        assert_eq!(w1.value(&settings, None, YearEvalType::StartIn), 1900);
    }

    #[test]
    fn year_input_calcualted() {
        let settings = test_settings_values();
        let w1 = YearInput::Calculate(YearComputation {
            base: YearSuggestion::YearStart,
            delta: 5,
        });
        let w2 = YearInput::Calculate(YearComputation {
            base: YearSuggestion::YearEnd,
            delta: -5,
        });
        assert_eq!(w1.value(&settings, None, YearEvalType::StartIn), 2005);
        assert_eq!(w2.value(&settings, None, YearEvalType::EndIn), 2075);
    }

    #[test]
    fn year_input_suggested() {
        let settings = test_settings_values();
        let dates = Dates {
            year_in: Some(YearRange {
                start: 1432,
                end: 1776,
            }),
            year_out: Some(YearRange {
                start: 1900,
                end: 1901,
            }),
        };

        let w1 = YearInput::Suggested(YearSuggestion::YearStart);
        let w2 = YearInput::Suggested(YearSuggestion::YearRetire);
        let w3 = YearInput::Suggested(YearSuggestion::YearDie);
        let w4 = YearInput::Suggested(YearSuggestion::YearEnd);
        let w5 = YearInput::Suggested(YearSuggestion::IncomeLink);
        let w6 = YearInput::Suggested(YearSuggestion::IncomeLink);
        let w7 = YearInput::Suggested(YearSuggestion::IncomeLink);
        let w8 = YearInput::Suggested(YearSuggestion::IncomeLink);

        assert_eq!(w1.value(&settings, None, YearEvalType::StartIn), 2000);
        assert_eq!(w2.value(&settings, None, YearEvalType::StartIn), 2030);
        assert_eq!(w3.value(&settings, None, YearEvalType::StartIn), 2080);
        assert_eq!(w4.value(&settings, None, YearEvalType::StartIn), 2080);
        assert_eq!(w5.value(&settings, Some(dates), YearEvalType::StartIn), 1432);
        assert_eq!(w6.value(&settings, Some(dates), YearEvalType::EndIn), 1776);
        assert_eq!(w7.value(&settings, Some(dates), YearEvalType::StartOut), 1900);
        assert_eq!(w8.value(&settings, Some(dates), YearEvalType::EndOut), 1901);
    }
}
