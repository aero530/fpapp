use std::collections::HashMap;
use accounts::{Account, AccountWrapper, PlotDataSet, UserData, YearlyTotals};
use serde_json::Value;

pub fn run_analysis(
    data: &Value,
) -> Result<(HashMap<String, Vec<PlotDataSet>>, YearlyTotals), String> {
    let user_data: UserData<AccountWrapper> =
        serde_json::from_value(data.clone()).map_err(|e| format!("Parse error: {}", e))?;
    let boxed: UserData<Box<dyn Account>> = user_data
        .try_into()
        .map_err(|e: Box<dyn std::error::Error>| e.to_string())?;
    accounts::run(boxed).map_err(|e| e.to_string())
}
