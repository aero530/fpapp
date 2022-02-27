import { writable } from 'svelte/store';
import { invoke } from "@tauri-apps/api/tauri";

import type { AccountWrapperUI as Account } from "../src-tauri/src/accounts/bindings/AccountWrapperUI";

import type {Settings} from "../src-tauri/src/accounts/bindings/Settings";

// import type { College } from "../src-tauri/src/accounts/bindings/College";
// import type { Expense } from "../src-tauri/src/accounts/bindings/Expense";
// import type { Hsa } from "../src-tauri/src/accounts/bindings/Hsa";
// import type { Income } from "../src-tauri/src/accounts/bindings/Income";
// import type { Loan } from "../src-tauri/src/accounts/bindings/Loan";
// import type { Mortgage } from "../src-tauri/src/accounts/bindings/Mortgage";
// import type { Retirement } from "../src-tauri/src/accounts/bindings/Retirement";
// import type { Savings } from "../src-tauri/src/accounts/bindings/Savings";
// import type { Ssa } from "../src-tauri/src/accounts/bindings/Ssa";

// export class Income {
//     readonly type: "income" & IncomeRust<number>
//     match<Out>(matcher: AccountMatcher<Out>): Out {
//         return matcher[AccountType.Income](this);
//     }
// }
// export class Ssa {readonly type: "ssa"  & SsaRust
//     match<Out>(matcher: AccountMatcher<Out>): Out {
//         return matcher[AccountType.Ssa](this);
//     }
// }
// export class Retirement {readonly type: "retirement" & RetirementRust<number>
//     match<Out>(matcher: AccountMatcher<Out>): Out {
//         return matcher[AccountType.Retirement](this);
//     }
// }
// export class Hsa {readonly type: "hsa" & HsaRust<number>
//     match<Out>(matcher: AccountMatcher<Out>): Out {
//         return matcher[AccountType.Hsa](this);
//     }
// }
// export class College {readonly type: "college" & CollegeRust<number>
//     match<Out>(matcher: AccountMatcher<Out>): Out {
//         return matcher[AccountType.College](this);
//     }
// }
// export class Expense {readonly type: "expense" & ExpenseRust<number>
//     match<Out>(matcher: AccountMatcher<Out>): Out {
//         return matcher[AccountType.Expense](this);
//     }
// }
// export class Loan {readonly type: "loan" & LoanRust<number>
//     match<Out>(matcher: AccountMatcher<Out>): Out {
//         return matcher[AccountType.Loan](this);
//     }
// }
// export class Mortgage {readonly type: "mortgage" & MortgageRust<number>
//     match<Out>(matcher: AccountMatcher<Out>): Out {
//         return matcher[AccountType.Mortgage](this);
//     }
// }
// export class Savings {readonly type: "savings" & SavingsRust<number>
//     match<Out>(matcher: AccountMatcher<Out>): Out {
//         return matcher[AccountType.Savings](this);
//     }
// }

// export enum AccountType {
//     Income='income',
//     Ssa='ssa',
//     Retirement='retirement',
//     Hsa='hsa',
//     College='college',
//     Expense='expense',
//     Loan='loan',
//     Mortgage='mortgage',
//     Savings='savings',
// }

// export type AccountMatcher<Out> = {
//     [AccountType.Income]: (account: Income) => Out;
//     [AccountType.Ssa]: (account: Ssa) => Out;
//     [AccountType.Retirement]: (account: Retirement) => Out;
//     [AccountType.Hsa]: (account: Hsa) => Out;
//     [AccountType.College]: (account: College) => Out;
//     [AccountType.Expense]: (account: Expense) => Out;
//     [AccountType.Loan]: (account: Loan) => Out;
//     [AccountType.Mortgage]: (account: Mortgage) => Out;
//     [AccountType.Savings]: (account: Savings) => Out;
// };

// export type Account = College | Expense | Hsa | Income | Loan | Mortgage | Retirement | Savings | Ssa;

// type Account = College<number> | Expense<number> | Hsa<number> | Income<number> | Loan<number> | Mortgage<number> | Retirement<number> | Savings<number> | Ssa;

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
    settings: {},
};

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
		reset: () => set({
            accounts: {},
            settings: {}
        })
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

//
//
//  Make derived accounts store for each store type instead of putting that into the form_inputs store
//
//
// export const college_inputs = derived(
// 	form_inputs,
// 	$form_inputs => {
//         var college = {};
//         Object.entries($form_inputs.accounts).forEach(([id,value]) => {
//             if (value.type == 'college') {
//                 college[id] = value;
//             }
//         });
//         return college;
//     }
// );


function createPlotData() {
	const { subscribe, set, update } = writable({});

	return {
		subscribe,
        set: (input) => set(input),
		reset: () => set({})
	};
}
export const plot_data = createPlotData();


function createSummaryData() {
	const { subscribe, set, update } = writable({});

	return {
		subscribe,
        set: (input) => set(input),
		reset: () => set({})
	};
}
export const summary_data = createSummaryData();

export const dark = writable(false);