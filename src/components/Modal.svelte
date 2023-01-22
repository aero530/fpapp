<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { fade } from "svelte/transition";

    export let title: string = "";
    let dispatch = createEventDispatcher();
</script>

<div
    class="fixed inset-0 w-screen h-screen bg-black opacity-50"
    transition:fade
    on:click={() => dispatch("close")}
    on:keypress={()=>{}}
/>

<div
    class="fixed inset-0 m-auto bg-background-500 w-fit h-fit rounded"
    transition:fade
>
    {#if title !== ""}
        <div class="bg-secondary-500 rounded-t w-full p-2">
            <h1 class="text-light font-bold text-xl">{title}</h1>
        </div>
    {/if}

    <div class="p-6">
        <slot />
    </div>
</div>
