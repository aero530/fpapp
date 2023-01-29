<script>
	
	import { listen, emit } from "@tauri-apps/api/event";
	import { open, save } from "@tauri-apps/api/dialog";
	
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount, onDestroy } from "svelte";

	import { path, form_inputs, dark} from './stores';
	
	import Dashboard from './pages/Dashboard.svelte';
	import Settings from './pages/Settings.svelte';
	import College from './pages/College.svelte';
	import Expenses from './pages/Expenses.svelte';
	import Hsa from './pages/Hsa.svelte';
	import Income from './pages/Income.svelte';
	import Loan from './pages/Loan.svelte';
	import Mortgage from './pages/Mortgage.svelte';
	import Retirement from './pages/Retirement.svelte';
	import Savings from './pages/Savings.svelte';
	import Ssa from './pages/Ssa.svelte';
	
	const pages = [
		{text: 'Dashboard', value: Dashboard, to: 'Dashboard'},
		{text: 'Settings', value: Settings, to: 'Settings'},
		{text: 'College', value: College, to: 'College'},
		{text: 'Expenses', value: Expenses, to: 'Expenses'},
		{text: 'HSA', value: Hsa, to: 'Hsa'},
		{text: 'Income', value: Income, to: 'Income'},
		{text: 'Loan', value: Loan, to: 'Loan'},
		{text: 'Mortgage', value: Mortgage, to: 'Mortgage'},
		{text: 'Retirement', value: Retirement, to: 'Retirement'},
		{text: 'Savings', value: Savings, to: 'Savings'},
		{text: 'SSA', value: Ssa, to: 'Ssa'},
	];
	let selected = pages[0];

	function openFile(pathString) {
		invoke("file_open", {
			path: pathString,
		})
		.then((data) => {
			form_inputs.set(data);
		})
		.catch((error) => alert(error));
	}

	function saveFile(pathString, data) {
		invoke("file_save", {
			path: pathString,
			data: data,
		})
		.catch((error) => alert(error));
	}
	
	let unlisten;
	onMount(async () => {
		form_inputs.reset();
		unlisten = await listen("rust-event", event => {
			switch (event.payload.name) {
				case 'file-open' :
					open()
					.then(function (pathString) {
						if (pathString) {
							// @ts-ignore
							path.set(pathString);
							openFile($path);
						}
					});
					break;
				case 'file-save' :
					saveFile($path, $form_inputs);
					break;
				case 'file-saveas' :
					save()
					.then(function (pathString) {
						if (pathString) {
							path.set(pathString);
							saveFile($path, $form_inputs);
						}
					});
					break;
				default : 
					alert("not sure what to do");
			}
		})
	})
	
	onDestroy(() => {
		if (unlisten) {
			unlisten()
		}
	})

	function toggleDark() {
		dark.set(!$dark);
		window.document.body.classList.toggle('dark')
	}
						
</script>

<aside class="top-0 left-0 w-64 h-screen fixed bg-background-200 dark:bg-darkbackground-500">
		<ul class="flex flex-col overflow-hidden">
			{#each pages as page}
				<li 
					on:click={() => {selected = page;}}
					on:keypress={() => {}}
					class="hover:bg-primary-200 active:bg-primary-300 py-2 px-4 font-semibold"
					class:bg-primary-400={selected === page}
				>
					{page.text}
				</li>
			{/each}
		</ul>
		<hr />
		<button 
			class="text-light bg-primary-500 hover:bg-primary-400 font-medium rounded-lg text-sm px-5 py-2.5 text-center mx-4 mb-2 mt-4 dark:bg-primary-300 dark:hover:bg-primary-200 "
			on:click={toggleDark}
		>
			Toggle Dark Mode
		</button>
</aside>
<main class="top-0 right-0 pl-64 mx-4">
	<svelte:component this={selected.value}/>
</main>

<!-- You can put your "global" style configurations here! -->
<style global lang="postcss">
    @tailwind base;
    @tailwind components;
    @tailwind utilities;

	:global(body) {
		@apply bg-background-500;
		@apply text-dark;
	}
	:global(body.dark) {
		@apply bg-darkbackground-500;
		@apply text-light;
	}
</style>