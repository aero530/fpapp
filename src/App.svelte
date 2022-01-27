<script>
	
	import { listen, emit } from "@tauri-apps/api/event";
	import { open, save } from "@tauri-apps/api/dialog";
	
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount, onDestroy } from "svelte";

	import { form_inputs, dark} from './stores.js';
	
	import Dashboard from './pages/Dashboard.svelte';
	import Settings from './pages/Settings.svelte';
	import College from './pages/College.svelte';
	import Expenses from './pages/Expenses.svelte';
	import Hsa from './pages/Hsa.svelte';
	import Income from './pages/Income.svelte';
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
	
	let unlisten;
	onMount(async () => {
		form_inputs.reset();
		unlisten = await listen("rust-event", event => {
			switch (event.payload.name) {
				case 'file-open' :
					open()
					.then(function (pathString) {
						openFile(pathString);
					});
					break;
				case 'file-save' :
					break;
				case 'file-saveas' :
					save();
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
	
	// function log() {
	//     invoke("log_operation", {
	//     event: "tauri-click",
	//     payload: "this payload is optional because we used Option in Rust",
	//     });
	// }
		
	// function performRequest() {
	//     invoke("perform_request", {
	//     endpoint: "dummy endpoint arg",
	//     body: {
	//         id: 5,
	//         name: "test",
	//     },
	//     })
	//     .then(onMessage)
	//     .catch(onMessage);
	// }
			
	// function emitEvent() {
	//     emit("js-event", "this is the payload string");
	// }
							
</script>


<!-- <div class="flex flex-row flex-wrap py-4">
	<aside class="w-full sm:w-1/3 md:w-1/4 px-2">
		<div class="sticky top-0 p-4 w-full">
			<ul class="flex flex-col overflow-hidden">
				{#each pages as page}
					<li on:click={() => {selected = page;}}>
						<a href="javascript:void(0)">
							{page.text}
						</a>
					</li>
				{/each}
			</ul>
			<hr />
			<button class="bg-sky-600 hover:bg-sky-700" on:click={toggleDark}>Toggle Dark Mode</button>
		</div>
	</aside>
	<main class="w-full sm:w-2/3 md:w-3/4 pt-1 px-2">
		<svelte:component this={selected.value}/>
	</main>
</div> -->

<!-- <div class="flex flex-row flex-wrap py-4">
	<aside class="w-full sm:w-1/3 md:w-1/4 px-2"> 
	<div class="sticky top-0 p-4 w-full bg-slate-300">
			<ul class="flex flex-col overflow-hidden">
				{#each pages as page}
					<a href="javascript:void(0)">
						<li on:click={() => {selected = page;}} class="hover:bg-violet-500 active:bg-violet-600 py-2 px-4">
							{page.text}
						</li>
					</a>
				{/each}
			</ul>
			<hr />
			<button class="text-white bg-blue-700 hover:bg-blue-800 font-medium rounded-lg text-sm px-5 py-2.5 text-center mr-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 " on:click={toggleDark}>Toggle Dark Mode</button>
			
	</div>
	</aside>
	<main class="w-full sm:w-2/3 md:w-3/4 pt-1 px-2">
		<svelte:component this={selected.value}/>
	</main>
</div> -->


<aside class="top-0 left-0 w-64 h-screen fixed bg-slate-200 dark:bg-slate-600">
		<ul class="flex flex-col overflow-hidden">
			{#each pages as page}
				<a href="javascript:void(0)">
					<li on:click={() => {selected = page;}} class="hover:bg-violet-500 active:bg-violet-600 py-2 px-4">
						{page.text}
					</li>
				</a>
			{/each}
		</ul>
		<hr />
		<button class="text-white bg-blue-700 hover:bg-blue-800 font-medium rounded-lg text-sm px-5 py-2.5 text-center mx-4 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 " on:click={toggleDark}>Toggle Dark Mode</button>
</aside>
<main class="top-0 right-0 pl-64 mx-4">
	<svelte:component this={selected.value}/>
</main>



<!-- You can put your "global" style configurations here! -->
<style global lang="postcss">
    @tailwind base;
    @tailwind components;
    @tailwind utilities;
/* 
    body {
        @apply bg-slate-200 dark:bg-slate-800 dark:text-slate-200;
    }

    button {
        @apply bg-slate-300 hover:bg-slate-200 border border-slate-800 px-3 py-1 rounded-sm;
    }

    table {
        @apply border-collapse;
    }

    th,
    td {
        @apply p-3;
    } */
	:global(body) {
		background-color: #ebebeb;
		color: #000000;
	}
	:global(body.dark) {
		background-color: #272727;
		color: #e2e2e2;
	} 
</style>
