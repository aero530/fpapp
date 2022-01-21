<script>
	import { invoke } from "@tauri-apps/api/tauri";
    import Button, { Label } from '@smui/button';

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
    <Label>Perform Request</Label>
</Button>