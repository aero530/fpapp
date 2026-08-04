use eframe::egui;
use serde_json::{Value, json};

/// Render label (col 0) + text edit (col 1) for a string JSON field.
/// Call inside a 2-column Grid; the caller adds ui.end_row().
pub fn string_field(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Value,
    field: &str,
    tooltip: &str,
) -> bool {
    ui.label(label).on_hover_text(tooltip);
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
    tooltip: &str,
) -> bool {
    ui.label(label).on_hover_text(tooltip);
    let mut n = value[field].as_f64().unwrap_or(0.0);
    let changed = ui.add(egui::DragValue::new(&mut n).speed(speed)).changed();
    if changed {
        value[field] = json!(n);
    }
    changed
}

/// Render label (col 0) + DragValue (col 1) for a u32 JSON field,
/// clamped to `range` so absurd values can't be entered.
pub fn u32_field(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Value,
    field: &str,
    range: std::ops::RangeInclusive<u32>,
    tooltip: &str,
) -> bool {
    ui.label(label).on_hover_text(tooltip);
    let mut n = value[field].as_u64().unwrap_or(0) as u32;
    // clamp_existing_to_range(false): user edits and drags still clamp, but a
    // legacy out-of-range value is not silently rewritten just by rendering
    // the page (egui's default would mark it changed and dirty the app)
    let changed = ui
        .add(
            egui::DragValue::new(&mut n)
                .range(range)
                .clamp_existing_to_range(false),
        )
        .changed();
    if changed {
        value[field] = json!(n);
    }
    changed
}

const YEAR_FORMAT_HINT: &str = "\n\nFormat: a year (2025), a keyword (yearStart, yearRetire, yearDie, yearEnd, incomeLink), \
     or an expression (yearRetire+5, yearDie-2)";

/// A buffered single-line text edit for expression-style fields.
///
/// While the field has focus the in-progress text is kept in egui temp memory
/// (shown in red when it does not parse) and nothing is committed — so typing
/// "yearRetire+" is not reverted mid-keystroke and typing "2025" does not
/// commit the intermediate year "2". The value is parsed and committed when
/// the field loses focus (including via Enter); Escape cancels the edit.
/// Returns Some(parsed) only for a successful commit; invalid text reverts to
/// the canonical value.
///
/// `id_salt` must be unique per (widget kind, owner, label) — the owner part
/// (e.g. the account uuid) prevents a buffer or focus from one account's field
/// leaking into the same field on another account's page.
fn expression_input(
    ui: &mut egui::Ui,
    id_salt: (&str, &str, &str),
    canonical: String,
    parse: impl Fn(&str) -> Option<Value>,
) -> Option<Value> {
    let id = ui.id().with(id_salt);
    let buffer: Option<String> = ui.data_mut(|d| d.get_temp(id));
    let mut text = buffer.unwrap_or_else(|| canonical.clone());

    let valid = text.trim().is_empty() || parse(&text).is_some();
    let mut edit = egui::TextEdit::singleline(&mut text).desired_width(f32::INFINITY);
    if !valid {
        edit = edit.text_color(egui::Color32::from_rgb(220, 60, 60));
    }
    let response = ui.add(edit);

    if response.lost_focus() {
        ui.data_mut(|d| d.remove::<String>(id));
        // Escape surrenders focus too, but means "cancel", not "commit"
        let cancelled = ui.input(|i| i.key_pressed(egui::Key::Escape));
        if !cancelled && !text.trim().is_empty() && text != canonical {
            return parse(&text);
        }
    } else if response.has_focus() {
        ui.data_mut(|d| d.insert_temp(id, text));
    } else {
        // Not focused and not just blurred: any leftover buffer is stale
        // (e.g. focus disappeared while this widget was not being rendered).
        // Drop it so the canonical value shows again.
        ui.data_mut(|d| d.remove::<String>(id));
    }
    None
}

/// Render label (col 0) + YearInput text edit (col 1).
/// Accepts: "2025", "yearRetire", "yearStart+5", "yearDie-2".
/// `owner_id` scopes the edit buffer (use the account uuid or a page name).
pub fn year_input(
    ui: &mut egui::Ui,
    label: &str,
    owner_id: &str,
    value: &mut Value,
    tooltip: &str,
) -> bool {
    let full_tip = format!("{}{}", tooltip, YEAR_FORMAT_HINT);
    ui.label(label).on_hover_text(full_tip);
    if let Some(parsed) = expression_input(
        ui,
        ("year_input", owner_id, label),
        year_input_to_string(value),
        parse_year_input,
    ) {
        *value = parsed;
        return true;
    }
    false
}

fn year_input_to_string(v: &Value) -> String {
    // ConstantInt — raw JSON integer
    if let Some(n) = v.as_u64() {
        return n.to_string();
    }
    // Suggested — raw JSON string like "yearRetire"
    if let Some(s) = v.as_str() {
        return s.to_string();
    }
    // Calculate — {"base": "yearRetire", "delta": -1}
    if let (Some(base), Some(delta)) = (
        v.get("base").and_then(|b| b.as_str()),
        v.get("delta").and_then(|d| d.as_i64()),
    ) {
        return if delta >= 0 {
            format!("{}+{}", base, delta)
        } else {
            format!("{}{}", base, delta)
        };
    }
    String::new()
}

const YEAR_KEYWORDS: &[&str] = &[
    "yearRetire",
    "yearStart",
    "yearDie",
    "yearEnd",
    "incomeLink",
];

fn parse_year_input(s: &str) -> Option<Value> {
    let s = s.trim();
    // ConstantInt — raw integer
    if let Ok(n) = s.parse::<u32>() {
        return Some(json!(n));
    }
    for &kw in YEAR_KEYWORDS {
        // Suggested — raw keyword string
        if s == kw {
            return Some(json!(kw));
        }
        // Calculate — {"base": "yearRetire", "delta": N}
        if let Some(rest) = s.strip_prefix(kw)
            && let Ok(delta) = rest.parse::<i64>()
        {
            return Some(json!({"base": kw, "delta": delta}));
        }
    }
    None
}

const PERCENT_FORMAT_HINT: &str = "\n\nFormat: a number (3.5) or a keyword (inflationBase)";

/// Render label (col 0) + PercentInput text edit (col 1).
/// Accepts: "3.5", "inflationBase".
/// `owner_id` scopes the edit buffer (use the account uuid or a page name).
pub fn percent_input(
    ui: &mut egui::Ui,
    label: &str,
    owner_id: &str,
    value: &mut Value,
    tooltip: &str,
) -> bool {
    let full_tip = format!("{}{}", tooltip, PERCENT_FORMAT_HINT);
    ui.label(label).on_hover_text(full_tip);
    if let Some(parsed) = expression_input(
        ui,
        ("percent_input", owner_id, label),
        percent_input_to_string(value),
        parse_percent_input,
    ) {
        *value = parsed;
        return true;
    }
    false
}

fn percent_input_to_string(v: &Value) -> String {
    // ConstantFloat — raw JSON number
    if let Some(n) = v.as_f64() {
        return format!("{}", n);
    }
    // ConstantString or Calculate(PercentSuggestions) — raw JSON string
    if let Some(s) = v.as_str() {
        return s.to_string();
    }
    String::new()
}

const PERCENT_KEYWORDS: &[&str] = &["inflationBase"];

fn parse_percent_input(s: &str) -> Option<Value> {
    let s = s.trim();
    // ConstantFloat — raw number
    if let Ok(n) = s.parse::<f64>() {
        return Some(json!(n));
    }
    // ConstantString / Calculate — raw keyword string
    if PERCENT_KEYWORDS.contains(&s) {
        return Some(json!(s));
    }
    None
}

/// Render "Notes:" label (col 0) + multi-line text edit (col 1).
pub fn notes_field(ui: &mut egui::Ui, value: &mut Value) -> bool {
    ui.label("Notes:")
        .on_hover_text("General information to store with this account");
    let mut text = value["notes"].as_str().unwrap_or("").to_string();
    let changed = ui
        .add(
            egui::TextEdit::multiline(&mut text)
                .desired_width(f32::INFINITY)
                .desired_rows(4),
        )
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
    tooltip: &str,
) -> bool {
    let current = value[field].as_str().unwrap_or("").to_string();
    let current_label = options
        .iter()
        .find(|(v, _)| *v == current)
        .map(|(_, l)| *l)
        .unwrap_or(current.as_str());
    let mut changed = false;

    ui.label(label).on_hover_text(tooltip);
    egui::ComboBox::new(id_salt, "")
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
        rows.sort_by(|a, b| {
            a.0.parse::<u32>()
                .unwrap_or(0)
                .cmp(&b.0.parse::<u32>().unwrap_or(0))
        });
        rows
    } else {
        Vec::new()
    };

    let mut to_remove: Option<usize> = None;
    let mut add_row = false;

    ui.push_id(field, |ui| {
        const YEAR_W: f32 = 80.0;
        const AMOUNT_W: f32 = 140.0;
        let row_h = ui.spacing().interact_size.y;

        egui::Grid::new("table_rows")
            .num_columns(3)
            .striped(true)
            .show(ui, |ui| {
                // Fixed-width header labels match the column widths below
                ui.add_sized([YEAR_W, row_h], egui::Label::new("Year"));
                ui.add_sized([AMOUNT_W, row_h], egui::Label::new("Amount"));
                ui.label("");
                ui.end_row();

                for (i, (year, amount)) in rows.iter_mut().enumerate() {
                    let mut year_n = year.parse::<u32>().unwrap_or(0);
                    // same plausible-year clamp the settings page uses; edits
                    // clamp but existing values are not rewritten on render
                    if ui
                        .add_sized(
                            [YEAR_W, row_h],
                            egui::DragValue::new(&mut year_n)
                                .speed(1)
                                .range(1850..=2200)
                                .clamp_existing_to_range(false),
                        )
                        .changed()
                    {
                        *year = year_n.to_string();
                        changed = true;
                    }
                    if ui
                        .add_sized([AMOUNT_W, row_h], egui::DragValue::new(amount).speed(100.0))
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
            // Always overwrite so the last (most recently edited) value wins on duplicates
            obj.insert(year, json!(amount));
        }
        *table = Value::Object(obj);
    }
    changed
}

/// Format a dollar value with thousands separators: "$1,234,567" or "-$42".
pub(crate) fn fmt_dollars(v: f64) -> String {
    let sign = if v < 0.0 { "-" } else { "" };
    let s = (v.abs() as i64).to_string();
    let with_commas = s
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(|b| std::str::from_utf8(b).unwrap())
        .collect::<Vec<_>>()
        .join(",");
    format!("{sign}${with_commas}")
}

/// Build a Plot pre-configured with a dollar-value hover label showing
/// "Series Name\n2035: $123,456" when the cursor is near a line.
pub(crate) fn dollar_plot(
    id: impl std::hash::Hash + std::fmt::Debug,
    height: f32,
) -> egui_plot::Plot<'static> {
    egui_plot::Plot::new(id)
        .height(height)
        .legend(egui_plot::Legend::default())
        .label_formatter(|pos| match pos {
            egui_plot::HoverPosition::NearDataPoint {
                plot_name,
                position,
                ..
            } => {
                let year = position.x as u32;
                let dollars = fmt_dollars(position.y);
                if plot_name.is_empty() {
                    Some(format!("{year}: {dollars}"))
                } else {
                    Some(format!("{plot_name}\n{year}: {dollars}"))
                }
            }
            egui_plot::HoverPosition::Elsewhere { .. } => None,
        })
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
    dollar_plot(id, height).show(ui, |plot_ui| {
        for ds in datasets {
            let points: egui_plot::PlotPoints = ds.data.iter().map(|p| [p.x as f64, p.y]).collect();
            plot_ui.line(egui_plot::Line::new(&ds.label, points));
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_year_input_accepts_all_forms() {
        assert_eq!(parse_year_input("2025"), Some(json!(2025)));
        assert_eq!(parse_year_input(" yearRetire "), Some(json!("yearRetire")));
        assert_eq!(parse_year_input("yearEnd"), Some(json!("yearEnd")));
        assert_eq!(parse_year_input("incomeLink"), Some(json!("incomeLink")));
        assert_eq!(
            parse_year_input("yearRetire+5"),
            Some(json!({"base": "yearRetire", "delta": 5}))
        );
        assert_eq!(
            parse_year_input("yearDie-2"),
            Some(json!({"base": "yearDie", "delta": -2}))
        );
    }

    #[test]
    fn parse_year_input_rejects_partial_and_garbage() {
        assert_eq!(parse_year_input("yearRetire+"), None);
        assert_eq!(parse_year_input("year"), None);
        assert_eq!(parse_year_input("-5"), None);
        assert_eq!(parse_year_input(""), None);
    }

    #[test]
    fn year_input_round_trips_through_display() {
        // to_string(parse(s)) == s for every accepted form
        for s in ["2025", "yearRetire", "yearEnd", "yearStart+5", "yearDie-2"] {
            let parsed = parse_year_input(s).unwrap();
            assert_eq!(year_input_to_string(&parsed), s, "round-trip of {s}");
        }
    }

    #[test]
    fn parse_percent_input_accepts_numbers_and_keywords() {
        assert_eq!(parse_percent_input("3.5"), Some(json!(3.5)));
        assert_eq!(
            parse_percent_input(" inflationBase "),
            Some(json!("inflationBase"))
        );
        assert_eq!(parse_percent_input("banana"), None);
    }

    #[test]
    fn fmt_dollars_formats_with_separators_and_sign() {
        assert_eq!(fmt_dollars(0.0), "$0");
        assert_eq!(fmt_dollars(1234567.89), "$1,234,567");
        assert_eq!(fmt_dollars(-42.0), "-$42");
        assert_eq!(fmt_dollars(999.0), "$999");
        assert_eq!(fmt_dollars(1000.0), "$1,000");
    }

    use egui_kittest::kittest::Queryable;

    /// Drive a lone year_input with kittest.  The bound JSON value starts as
    /// Null so the field starts empty.
    fn year_input_harness() -> egui_kittest::Harness<'static, Value> {
        egui_kittest::Harness::new_ui_state(
            |ui, value: &mut Value| {
                egui::Grid::new("test_grid").num_columns(2).show(ui, |ui| {
                    year_input(ui, "Year:", "test-owner", value, "tooltip");
                    ui.end_row();
                });
            },
            Value::Null,
        )
    }

    #[test]
    fn year_input_commits_on_enter() {
        let mut harness = year_input_harness();
        harness.run();
        harness
            .get_by_role(egui::accesskit::Role::TextInput)
            .click();
        harness.run();
        harness
            .get_by_role(egui::accesskit::Role::TextInput)
            .type_text("yearRetire+5");
        harness.run();
        // still focused: nothing committed yet (no mid-typing commits)
        assert_eq!(*harness.state(), Value::Null);

        harness.key_press(egui::Key::Enter);
        harness.run();
        assert_eq!(*harness.state(), json!({"base": "yearRetire", "delta": 5}));
    }

    /// Click the field, type into it, and verify the text landed (so the
    /// cancel/revert tests can't pass vacuously by typing into nothing).
    fn focus_and_type(harness: &mut egui_kittest::Harness<'static, Value>, text: &str) {
        harness
            .get_by_role(egui::accesskit::Role::TextInput)
            .click();
        harness.run();
        harness
            .get_by_role(egui::accesskit::Role::TextInput)
            .type_text(text);
        harness.run();
        let shown = harness
            .get_by_role(egui::accesskit::Role::TextInput)
            .value()
            .unwrap_or_default();
        assert_eq!(shown, text, "typed text did not reach the field");
    }

    #[test]
    fn year_input_escape_cancels() {
        let mut harness = year_input_harness();
        harness.run();
        focus_and_type(&mut harness, "2025");

        harness.key_press(egui::Key::Escape);
        harness.run();
        assert_eq!(*harness.state(), Value::Null);
        // and the stale buffer is gone: the field shows the canonical value again
        harness.run();
        let shown = harness
            .get_by_role(egui::accesskit::Role::TextInput)
            .value()
            .unwrap_or_default();
        assert_eq!(shown, "");
    }

    #[test]
    fn year_input_invalid_text_reverts_on_blur() {
        let mut harness = year_input_harness();
        harness.run();
        focus_and_type(&mut harness, "yearRetire+");

        // Enter (blur) with unparseable text: nothing committed
        harness.key_press(egui::Key::Enter);
        harness.run();
        assert_eq!(*harness.state(), Value::Null);
    }
}
