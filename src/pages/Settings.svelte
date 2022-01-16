<script>
	import { invoke } from "@tauri-apps/api/tauri";
    import { Button } from "carbon-components-svelte";

    function performRequest() {
		invoke("do_a_thing", {
			body: {
				id: 5,
				name: "test",
			},
		})
		.then((reply) => alert(reply));
	}

</script>
  
<h3>Settings</h3>
  
<Button on:click={performRequest}>
    Perform Request
</Button>