<script lang="ts">
	import {v4 as uuidv4 } from 'uuid';

	export let questionText = "";
	export let label="";
	let size = 24;
    let padding = 2;

	let id = uuidv4();
</script>

<div class="flex items-center">
	<div class="mr-3 text-md">
		<label for="{id}" class="text-dark dark:text-light">{label}</label>
	</div>
	<div class="flex grow items-center">
		<slot id="{id}"/>
		<div class="w-6 h-6 ml-2 has-tooltip">
			<svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" style="padding:{padding}px; height:{size}px; width:{size}px; margin:0px;" viewBox="0 0 32 32">
				<defs>
				<style>.cls-1{fill:none;}</style>
				</defs>
				<path d="M16,2A14,14,0,1,0,30,16,14,14,0,0,0,16,2Zm0,26A12,12,0,1,1,28,16,12,12,0,0,1,16,28Z"/>
				<circle cx="16" cy="23.5" r="1.5"/>
				<path d="M17,8H15.5A4.49,4.49,0,0,0,11,12.5V13h2v-.5A2.5,2.5,0,0,1,15.5,10H17a2.5,2.5,0,0,1,0,5H15v4.5h2V17a4.5,4.5,0,0,0,0-9Z"/>
				<rect class="cls-1" width="32" height="32"/>
			</svg >
			<span class="tooltip px-1 py-1 -mt-10 ml-6 text-center border rounded-sm bg-background-400 dark:bg-darkbackground-300 border-dark">{questionText}</span>
		</div>
	</div>
</div>

<style>
	.tooltip {
		@apply invisible absolute;
	}

	.has-tooltip:hover .tooltip {
		@apply visible z-50
	}
</style>