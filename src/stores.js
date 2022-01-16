import { derived, writable } from 'svelte/store';

function createInput() {
	const { subscribe, set, update } = writable({
        accounts: {},
        settings: {},
        college: {},
        expense: {},
        hsa: {},
        income: {},
        mortgage: {},
        retirement: {},
        savings: {},
        ssa: {},
    });

	return {
		subscribe,
		initialize: (input) => {
            // loop through accounts and split by account type
            var college = {}, expense = {}, hsa = {}, income = {}, mortgage = {}, retirement = {}, savings = {}, ssa = {};

            Object.entries(input.accounts).forEach(([id,value]) => {
                switch (value.type) {
                    case 'college':
                        college[id] = value;
                        break;
                    case 'expense':
                        expense[id] = value;
                        break;
                    case 'hsa':
                        hsa[id] = value;
                        break;
                    case 'income':
                        income[id] = value;
                        break;
                    case 'mortgage':
                        mortgage[id] = value;
                        break;
                    case 'retirement':
                        retirement[id] = value;
                        break;
                    case 'savings':
                        savings[id] = value;
                        break;
                    case 'ssa':
                        ssa[id] = value;
                        break;
                }
            });

            set({
                accounts: input.accounts,
                settings: input.settings,
                college,
                expense,
                hsa,
                income,
                mortgage,
                retirement,
                savings,
                ssa,
            })}
        ,
		reset: () => set({
            accounts: {},
            settings: {},
            college: {},
            expense: {},
            hsa: {},
            income: {},
            mortgage: {},
            retirement: {},
            savings: {},
            ssa: {},
        })
	};
}

export const form_inputs = createInput();

export const analysis_inputs = derived(
	form_inputs,
	$form_inputs => {
        let output = {};
        output.accounts = $form_inputs.accounts;
        output.settings = $form_inputs.settings;
        return output;
    }
);





function createPlotData() {
	const { subscribe, set, update } = writable({});

	return {
		subscribe,
		initialize: (input) => set(input),
		reset: () => set({})
	};
}

export const plot_data = createPlotData();


function createSummaryData() {
	const { subscribe, set, update } = writable({});

	return {
		subscribe,
		initialize: (input) => set(input),
		reset: () => set({})
	};
}

export const summary_data = createSummaryData();