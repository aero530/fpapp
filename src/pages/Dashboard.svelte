<script>
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";

	onMount(() => {
	    invoke('my_custom_command');
	});

</script>

<h1 class="text-lg">Dashboard</h1>
