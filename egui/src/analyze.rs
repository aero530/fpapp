use std::collections::HashMap;
use accounts::{AccountWrapper, PlotDataSet, UserData, YearlyTotals};
use serde_json::Value;

pub fn run_analysis(
    data: &Value,
) -> Result<(HashMap<String, Vec<PlotDataSet>>, YearlyTotals), String> {
    let user_data: UserData<AccountWrapper> =
        serde_json::from_value(data.clone()).map_err(|e| format!("Parse error: {}", e))?;
    let boxed = user_data.into();
    accounts::run(boxed).map_err(|e| e.to_string())
}
