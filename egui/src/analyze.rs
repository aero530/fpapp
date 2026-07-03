use accounts::{AccountWrapper, AnalysisOutput, SimAccount, UserData};
use serde::Deserialize;
use serde_json::Value;

pub fn run_analysis(data: &Value) -> Result<AnalysisOutput, String> {
    // Deserialize by reference — no deep clone of the whole data blob per run
    let user_data =
        UserData::<AccountWrapper>::deserialize(data).map_err(|e| format!("Parse error: {}", e))?;
    let sim: UserData<SimAccount> = user_data
        .try_into()
        .map_err(|e: accounts::Error| e.to_string())?;
    accounts::run(sim).map_err(|e| e.to_string())
}
