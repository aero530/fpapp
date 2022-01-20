import { derived, writable } from 'svelte/store';
import { invoke } from "@tauri-apps/api/tauri";



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

function createInput() {
	const { subscribe, set, update } = writable({
        accounts: {},
        settings: {},
    });

	return {
		subscribe,
        set: (inputs) => {
            var reply = set(inputs);
            console.log(inputs);
            run_analysis(inputs);
            reply
        },
		reset: () => set({
            accounts: {},
            settings: {},
        })
	};
}
export const form_inputs = createInput();


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
