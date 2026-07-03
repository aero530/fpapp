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
                // do the math in i64 so a large negative delta clamps to zero
                // instead of wrapping around to a huge year
                let year =
                    input.base.value(settings, linked_dates, eval_type) as i64 + input.delta as i64;
                if year < 0 {
                    log::warn!(
                        "year computation {:?} produced negative year {} — clamping to 0",
                        input,
                        year
                    );
                    0
                } else {
                    year as u32
                }
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
    YearStart,
    /// When you plan to retire
    YearRetire,
    /// When you plan to die
    YearDie,
    /// Last year of the simulation
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
            Self::IncomeLink => {
                let range = match eval_type {
                    YearEvalType::StartIn | YearEvalType::EndIn => {
                        linked_dates.and_then(|d| d.year_in)
                    }
                    YearEvalType::StartOut | YearEvalType::EndOut => {
                        linked_dates.and_then(|d| d.year_out)
                    }
                };
                match (range, eval_type) {
                    (Some(r), YearEvalType::StartIn | YearEvalType::StartOut) => r.start,
                    (Some(r), YearEvalType::EndIn | YearEvalType::EndOut) => r.end,
                    // A missing link used to silently resolve to year 0, hiding
                    // the misconfiguration; fall back to the simulation bounds instead
                    (None, YearEvalType::StartIn | YearEvalType::StartOut) => {
                        log::warn!(
                            "incomeLink year used but no linked account dates available — using year_start"
                        );
                        settings.year_start()
                    }
                    (None, YearEvalType::EndIn | YearEvalType::EndOut) => {
                        log::warn!(
                            "incomeLink year used but no linked account dates available — using year_end"
                        );
                        settings.year_end()
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_fixtures::test_settings_values;
    use super::*;
    use crate::simulation::YearRange;

    #[test]
    fn year_input_constant() {
        let settings = test_settings_values();
        let w1 = YearInput::ConstantInt(1900);
        assert_eq!(w1.value(&settings, None, YearEvalType::StartIn), 1900);
    }

    #[test]
    fn year_input_calculated() {
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
    fn year_input_calculated_negative_result_clamps_to_zero() {
        // Regression test for B6: a huge negative delta must not wrap to a huge year.
        let settings = test_settings_values();
        let w = YearInput::Calculate(YearComputation {
            base: YearSuggestion::YearStart,
            delta: -3000,
        });
        assert_eq!(w.value(&settings, None, YearEvalType::StartIn), 0);
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

    #[test]
    fn year_input_income_link_without_link_falls_back_to_simulation_bounds() {
        // Regression test for B6: no silent year-0 resolution.
        let settings = test_settings_values();
        let w = YearInput::Suggested(YearSuggestion::IncomeLink);
        assert_eq!(w.value(&settings, None, YearEvalType::StartIn), 2000);
        assert_eq!(w.value(&settings, None, YearEvalType::EndIn), 2080);
        assert_eq!(w.value(&settings, None, YearEvalType::StartOut), 2000);
        assert_eq!(w.value(&settings, None, YearEvalType::EndOut), 2080);
    }
}
