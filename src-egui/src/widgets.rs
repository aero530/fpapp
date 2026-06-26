use eframe::egui;
use serde_json::{Value, json};

/// Render label (col 0) + text edit (col 1) for a string JSON field.
/// Call inside a 2-column Grid; the caller adds ui.end_row().
pub fn string_field(ui: &mut egui::Ui, label: &str, value: &mut Value, field: &str) -> bool {
    ui.label(label);
    let mut text = value[field].as_str().unwrap_or("").to_string();
    let changed = ui
        .add(egui::TextEdit::singleline(&mut text).desired_width(f32::INFINITY))
        .changed();
    if changed {
        value[field] = json!(text);
    }
    changed
}

/// Render label (col 0) + DragValue (col 1) for an f64 JSON field.
pub fn f64_field(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Value,
    field: &str,
    speed: f64,
) -> bool {
    ui.label(label);
    let mut n = value[field].as_f64().unwrap_or(0.0);
    let changed = ui.add(egui::DragValue::new(&mut n).speed(speed)).changed();
    if changed {
        value[field] = json!(n);
    }
    changed
}

/// Render label (col 0) + DragValue (col 1) for a u32 JSON field.
pub fn u32_field(ui: &mut egui::Ui, label: &str, value: &mut Value, field: &str) -> bool {
    ui.label(label);
    let mut n = value[field].as_u64().unwrap_or(0) as u32;
    let changed = ui.add(egui::DragValue::new(&mut n)).changed();
    if changed {
        value[field] = json!(n);
    }
    changed
}

/// Render label (col 0) + YearInput text edit (col 1).
/// Accepts: "2025", "yearRetire", "yearStart+5", "yearDie-2".
pub fn year_input(ui: &mut egui::Ui, label: &str, value: &mut Value) -> bool {
    ui.label(label);
    let mut text = year_input_to_string(value);
    let changed = ui
        .add(egui::TextEdit::singleline(&mut text).desired_width(f32::INFINITY))
        .changed();
    if changed {
        *value = parse_year_input(&text);
    }
    changed
}

fn year_input_to_string(v: &Value) -> String {
    if let Some(n) = v.get("constantInt").and_then(|n| n.as_u64()) {
        return n.to_string();
    }
    if let Some(s) = v.get("suggested").and_then(|s| s.as_str()) {
        return s.to_string();
    }
    if let Some(calc) = v.get("calculate") {
        let base = calc.get("base").and_then(|b| b.as_str()).unwrap_or("");
        let delta = calc.get("delta").and_then(|d| d.as_i64()).unwrap_or(0);
        return if delta >= 0 {
            format!("{}+{}", base, delta)
        } else {
            format!("{}{}", base, delta)
        };
    }
    String::new()
}

const YEAR_KEYWORDS: &[&str] = &["yearRetire", "yearStart", "yearDie", "yearBorn"];

fn parse_year_input(s: &str) -> Value {
    let s = s.trim();
    if let Ok(n) = s.parse::<u32>() {
        return json!({"constantInt": n});
    }
    for &kw in YEAR_KEYWORDS {
        if s == kw {
            return json!({"suggested": kw});
        }
        if let Some(rest) = s.strip_prefix(kw) {
            if let Ok(delta) = rest.parse::<i64>() {
                return json!({"calculate": {"base": kw, "delta": delta}});
            }
        }
    }
    json!({"constantInt": 0})
}

/// Render label (col 0) + PercentInput text edit (col 1).
/// Accepts: "3.5", "inflationBase".
pub fn percent_input(ui: &mut egui::Ui, label: &str, value: &mut Value) -> bool {
    ui.label(label);
    let mut text = percent_input_to_string(value);
    let changed = ui
        .add(egui::TextEdit::singleline(&mut text).desired_width(f32::INFINITY))
        .changed();
    if changed {
        *value = parse_percent_input(&text);
    }
    changed
}

fn percent_input_to_string(v: &Value) -> String {
    if let Some(n) = v.get("constantFloat").and_then(|n| n.as_f64()) {
        return format!("{}", n);
    }
    if let Some(s) = v.get("constantString").and_then(|s| s.as_str()) {
        return s.to_string();
    }
    if let Some(s) = v.get("calculate").and_then(|s| s.as_str()) {
        return s.to_string();
    }
    String::new()
}

const PERCENT_KEYWORDS: &[&str] = &["inflationBase", "taxIncome", "taxCapitalGains"];

fn parse_percent_input(s: &str) -> Value {
    let s = s.trim();
    if let Ok(n) = s.parse::<f64>() {
        return json!({"constantFloat": n});
    }
    for &kw in PERCENT_KEYWORDS {
        if s == kw {
            return json!({"calculate": kw});
        }
    }
    json!({"constantFloat": 0.0})
}

/// Render "Notes:" label (col 0) + single-line text edit (col 1).
pub fn notes_field(ui: &mut egui::Ui, value: &mut Value) -> bool {
    ui.label("Notes:");
    let mut text = value["notes"].as_str().unwrap_or("").to_string();
    let changed = ui
        .add(egui::TextEdit::singleline(&mut text).desired_width(f32::INFINITY))
        .changed();
    if changed {
        value["notes"] = if text.is_empty() {
            Value::Null
        } else {
            json!(text)
        };
    }
    changed
}

/// Render label (col 0) + ComboBox (col 1) for an enum-valued JSON field.
/// `options` is a slice of (json_value, display_label) pairs.
pub fn combo_field(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Value,
    field: &str,
    options: &[(&str, &str)],
    id_salt: &str,
) -> bool {
    let current = value[field].as_str().unwrap_or("").to_string();
    let current_label = options
        .iter()
        .find(|(v, _)| *v == current)
        .map(|(_, l)| *l)
        .unwrap_or(current.as_str());
    let mut changed = false;

    ui.label(label);
    let width = ui.available_width();
    egui::ComboBox::new(id_salt, "")
        .width(width)
        .selected_text(current_label)
        .show_ui(ui, |ui| {
            for (val, lbl) in options {
                if ui.selectable_label(current == *val, *lbl).clicked() {
                    value[field] = json!(val);
                    changed = true;
                }
            }
        });
    changed
}

/// Editable year/amount table for the "table" field of an account.
pub fn table_editor(ui: &mut egui::Ui, value: &mut Value, field: &str) -> bool {
    let mut changed = false;
    let table = &mut value[field];

    let mut rows: Vec<(String, f64)> = if let Some(obj) = table.as_object() {
        let mut rows: Vec<(String, f64)> = obj
            .iter()
            .map(|(k, v)| (k.clone(), v.as_f64().unwrap_or(0.0)))
            .collect();
        rows.sort_by(|a, b| a.0.cmp(&b.0));
        rows
    } else {
        Vec::new()
    };

    let mut to_remove: Option<usize> = None;
    let mut add_row = false;

    ui.push_id(field, |ui| {
        egui::Grid::new("table_rows")
            .num_columns(3)
            .striped(true)
            .min_col_width(60.0)
            .show(ui, |ui| {
                ui.label("Year");
                ui.label("Amount");
                ui.label("");
                ui.end_row();

                for (i, (year, amount)) in rows.iter_mut().enumerate() {
                    let mut year_n = year.parse::<u32>().unwrap_or(0);
                    if ui.add(egui::DragValue::new(&mut year_n).speed(1)).changed() {
                        *year = year_n.to_string();
                        changed = true;
                    }
                    if ui
                        .add(egui::DragValue::new(amount).speed(100.0))
                        .changed()
                    {
                        changed = true;
                    }
                    if ui.small_button("−").clicked() {
                        to_remove = Some(i);
                        changed = true;
                    }
                    ui.end_row();
                }
            });

        if ui.small_button("+ Add row").clicked() {
            add_row = true;
            changed = true;
        }
    });

    if let Some(i) = to_remove {
        rows.remove(i);
    }
    if add_row {
        let next_year = rows
            .last()
            .and_then(|(y, _)| y.parse::<u32>().ok())
            .map(|y| y + 1)
            .unwrap_or(2024)
            .to_string();
        rows.push((next_year, 0.0));
    }

    if changed {
        let mut obj = serde_json::Map::new();
        for (year, amount) in rows {
            obj.insert(year, json!(amount));
        }
        *table = Value::Object(obj);
    }
    changed
}

/// Plot one or more PlotDataSet series in a line chart.
pub fn plot_datasets(
    ui: &mut egui::Ui,
    id: &str,
    datasets: &[accounts::PlotDataSet],
    title: &str,
    height: f32,
) {
    ui.label(title);
    egui_plot::Plot::new(id)
        .height(height)
        .legend(egui_plot::Legend::default())
        .show(ui, |plot_ui| {
            for ds in datasets {
                let points: egui_plot::PlotPoints = ds.data.iter().map(|p| [p.x as f64, p.y]).collect();
                plot_ui.line(egui_plot::Line::new(points).name(&ds.label));
            }
        });
}
