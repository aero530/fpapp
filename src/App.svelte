<script>
    import { listen, emit } from "@tauri-apps/api/event";
    import { open, save } from "@tauri-apps/api/dialog";

    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount, onDestroy } from "svelte";

    import Drawer, { AppContent, Content } from '@smui/drawer';
    import List, { Item, Text } from '@smui/list';

    import { form_inputs } from './stores.js';
 
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
        {title: 'Dashboard', component: Dashboard},
        {title: 'Settings', component: Settings},
        {title: 'College', component: College},
        {title: 'Expenses', component: Expenses},
        {title: 'HSA', component: Hsa},
        {title: 'Income', component: Income},
        {title: 'Mortgage', component: Mortgage},
        {title: 'Retirement', component: Retirement},
        {title: 'Savings', component: Savings},
        {title: 'SSA', component: Ssa},
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

<div class="drawer-container">
<Drawer>
    <Content>
        <List>
            {#each pages as page}
                <Item
                    href="javascript:void(0)"
                    on:click={() => {
                        selected = page;
                    }}
                >
                    <Text>{page.title}</Text>
                </Item>
            {/each}
        </List>
    </Content>
</Drawer>

<AppContent class="app-content">
<main class="main-content">
    <svelte:component this={selected.component}/>
</main>
</AppContent>
</div>


<style>
    /* These classes are only needed because the
      drawer is in a container on the page. */
    .drawer-container {
      position: relative;
      display: flex;
      height: 100%;
      border: 1px solid
        var(--mdc-theme-text-hint-on-background, rgba(0, 0, 0, 0.1));
      overflow: hidden;
      z-index: 0;
    }
    * :global(.app-content) {
      flex: auto;
      overflow: auto;
      position: relative;
      flex-grow: 1;
    }
    .main-content {
      overflow: auto;
      padding: 16px;
      height: 100%;
      box-sizing: border-box;
    }
  </style>