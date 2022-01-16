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

export const input_data = createInput();