import { writable } from 'svelte/store';
import { invoke } from "@tauri-apps/api/tauri";
import { v4 as uuid } from 'uuid';

import type { AccountWrapperUI as Account } from "../src-tauri/src/accounts/bindings/AccountWrapperUI";

import type {Settings} from "../src-tauri/src/accounts/bindings/Settings";

import {defaultCollege, defaultExpense, defaultHsa, defaultIncome, defaultLoan, defaultMortgage, defaultRetirement, defaultSavings, defaultSsa} from "./accountDefaults";

import type { PointArray } from "./components/Charts/chart.type";

function run_analysis(inputs) {
    invoke("run_analysis", {
        input: {...inputs},
    })
    .then((results) => {
        plot_data.set(results[0])
        summary_data.set(results[1])
        console.log(results);
    });
}

export type Table = {
    [year: string] : number
}

type Accounts = {
    [id: string] : Account
}

type InputData = {
    accounts: Accounts,
    settings: Settings,
}

let defaultInput : InputData = {
    accounts: {},
    settings: {
        ageRetire: 65,
        ageDie: 90,
        yearBorn: 1950,
        yearStart: 2000,
        inflationBase: 10,
        taxIncome: 10,
        taxCapitalGains: 10,
        retirementCostOfLiving: 100,
        ssa: {
            breakpoints: {low:0,high:0},
            taxableIncomePercentage: {low:0,high:0},
        },
    },
};


export enum AccountType {
    college,
    expense,
    hsa,
    income,
    loan,
    mortgage,
    retirement,
    savings,
    ssa
}

function createInput() {
	const { subscribe, set, update } = writable(defaultInput);

	return {
		subscribe,
        set: (inputs: InputData) => {
            set(inputs);
            run_analysis(inputs);
        },
        removeTableRow: (id: string, table: string, year: string) => {
            update(current => {
                delete current.accounts[id][table][year]
                run_analysis(current);
                return current;
            });
        },
        addTableRow: (id: string, table: string, year: string, value: number) => {
            update(current => {
                current.accounts[id][table][year] = value;
                run_analysis(current);
                return current;
            });
        },
        removeAccount: (id: string) => {
            update(current => {
                delete current.accounts[id]
                run_analysis(current);
                return current;
            });
        },
        addAccount: (type: AccountType) => {
            let id = uuid();
            update(current => {
                switch (type) {
                    case AccountType.college :
                        current.accounts[id] = JSON.parse(JSON.stringify(defaultCollege));
                        break;
                    case AccountType.expense:
                        current.accounts[id] = JSON.parse(JSON.stringify(defaultExpense));
                        break;
                    case AccountType.hsa:
                        current.accounts[id] = JSON.parse(JSON.stringify(defaultHsa));
                        break;
                    case AccountType.income:
                        current.accounts[id] = JSON.parse(JSON.stringify(defaultIncome));
                        break;
                    case AccountType.loan:
                        current.accounts[id] = JSON.parse(JSON.stringify(defaultLoan));
                        break;
                    case AccountType.mortgage:
                        current.accounts[id] = JSON.parse(JSON.stringify(defaultMortgage));
                        break;
                    case AccountType.retirement:
                        current.accounts[id] = JSON.parse(JSON.stringify(defaultRetirement));
                        break;
                    case AccountType.savings:
                        current.accounts[id] = JSON.parse(JSON.stringify(defaultSavings));
                        break;
                    case AccountType.ssa:
                        current.accounts[id] = JSON.parse(JSON.stringify(defaultSsa));
                        break;
                }
                run_analysis(current);
                return current;
            });
        },
		reset: () => set(defaultInput)
	};
}
export const form_inputs = createInput();


function createPath() {
	const { subscribe, set } = writable("");

	return {
		subscribe,
        set: (input: string) => set(input),
		reset: () => set("")
	};
}
export const path = createPath();



function createPlotData() {
	const { subscribe, set, update } = writable({});

	return {
		subscribe,
        set: (input) => set(input),
		reset: () => set({})
	};
}
export const plot_data = createPlotData();



type SummaryData = {
    col: PointArray<number,number>,
    expense: PointArray<number,number>,
    healthcare_expense: PointArray<number,number>,
    hsa: PointArray<number,number>,
    income: PointArray<number,number>,
    income_during_retirement: PointArray<number,number>,
    income_taxable: PointArray<number,number>,
    net: PointArray<number,number>,
    saving: PointArray<number,number>,
    tax_burden: PointArray<number,number>
}


let defaultSummary : SummaryData = {
    col: [],
    expense: [],
    healthcare_expense: [],
    hsa: [],
    income: [],
    income_during_retirement: [],
    income_taxable: [],
    net: [],
    saving: [],
    tax_burden: [],
};

function createSummaryData() {
	const { subscribe, set, update } = writable(defaultSummary);

	return {
		subscribe,
        set: (input) => set(input),
		reset: () => set({})
	};
}
export const summary_data = createSummaryData();

export const dark = writable(false);












