use std::collections::HashMap;
use std::error::Error;

use accounts::{Account, AccountType, AccountWrapper, Dates, PlotDataSet, UserData, YearlyTotals};
use serde_json::Value;

pub fn run_analysis(
    data: &Value,
) -> Result<(HashMap<String, Vec<PlotDataSet>>, YearlyTotals), String> {
    let user_data: UserData<AccountWrapper> =
        serde_json::from_value(data.clone()).map_err(|e| format!("Parse error: {}", e))?;
    let boxed: UserData<Box<dyn Account>> = user_data.into();
    analyze(boxed).map_err(|e| e.to_string())
}

fn analyze(
    mut data: UserData<Box<dyn Account>>,
) -> Result<(HashMap<String, Vec<PlotDataSet>>, YearlyTotals), Box<dyn Error>> {
    let mut account_order: Vec<String> = Vec::new();
    for type_id in AccountWrapper::order().iter() {
        for (uuid, account) in data.accounts.iter() {
            if account.type_id() == *type_id {
                account_order.push(uuid.clone());
            }
        }
    }

    let years: Vec<u32> = (data.settings.year_start()..data.settings.year_end()).collect();
    let mut yearly_totals = YearlyTotals::new();

    // Initialize all accounts before running the main year loop
    account_order.iter().for_each(|uuid| {
        let linked_dates: Option<Dates> = match data.accounts.get(uuid).unwrap().link_id() {
            Some(link_id) => Some(Dates {
                year_in: data
                    .accounts
                    .get(&link_id)
                    .expect("linked account not found")
                    .get_range_in(&data.settings, None),
                year_out: data
                    .accounts
                    .get(&link_id)
                    .expect("linked account not found")
                    .get_range_out(&data.settings, None),
            }),
            None => None,
        };

        let impacts = data
            .accounts
            .get_mut(uuid)
            .unwrap()
            .init(linked_dates, &data.settings)
            .unwrap();

        impacts.iter().for_each(|(year, impact)| {
            if !yearly_totals.contains_year(*year) {
                yearly_totals.add_year(*year, false).unwrap();
            }
            yearly_totals.update(*year, *impact);
        });
    });

    for year in years {
        if yearly_totals.add_year(year, true).is_ok() {
            for uuid in account_order.iter() {
                let link_id = data.accounts.get(uuid).unwrap().link_id();
                let link_value = match link_id {
                    Some(ref id) => match data.accounts.get(id) {
                        Some(la) if la.type_id() == AccountType::Income => la.get_value(year),
                        Some(la) => {
                            log::warn!(
                                "income_link on '{}' points to {:?}, not Income",
                                data.accounts.get(uuid).unwrap().name(),
                                la.type_id()
                            );
                            None
                        }
                        None => None,
                    },
                    None => None,
                };

                let account = data.accounts.get_mut(uuid).unwrap();
                let impact = account
                    .simulate(year, &yearly_totals, &data.settings, link_value)
                    .map_err(|e| {
                        format!("Account '{}', year {}: {}", account.name(), year, e)
                    })?;
                yearly_totals.update(year, impact);
            }

            yearly_totals.deposit_income_in_net(year);
            yearly_totals.pay_income_tax_from_net(year, data.settings.tax_income);
            yearly_totals.pay_expenses_from_net(year);
            yearly_totals.pay_healthcare_expenses_from_net(year);
        }
    }

    let mut plot_data: HashMap<String, Vec<PlotDataSet>> = HashMap::new();
    for (uuid, account) in data.accounts.iter() {
        plot_data.insert(uuid.clone(), account.get_plot_data());
    }

    Ok((plot_data, yearly_totals))
}
