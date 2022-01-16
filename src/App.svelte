<script>
    import { listen, emit } from "@tauri-apps/api/event";
    import { open, save } from "@tauri-apps/api/dialog";

    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount, onDestroy } from "svelte";

    import { Router } from "@roxi/routify";
    import { routes } from "../.routify/routes";

    import { input_data } from './stores.js';
 
    function openFile(pathString) {
		invoke("file_open", {
			path: pathString,
		})
		.then((data) => {
            console.log(data);
            input_data.initialize(data);
            console.log($input_data.settings);
            console.log($input_data);
        })
        .catch((error) => alert(error));
	}

    onMount(async () => {
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

<Router {routes} />