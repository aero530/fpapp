use accounts::{AnalysisOutput, SimAccount, UserData};
use serde::Deserialize;
use serde_json::Value;

pub fn run_analysis(data: &Value) -> Result<AnalysisOutput, String> {
    // Deserialize by reference — no deep clone of the whole data blob per run
    let user_data =
        UserData::<SimAccount>::deserialize(data).map_err(|e| format!("Parse error: {}", e))?;
    accounts::run(user_data).map_err(|e| e.to_string())
}
