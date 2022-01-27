<script lang="ts">
    // import { Tooltip } from "smelte";
    import Question from '../icons/Question.svelte';
</script>


<div class="flex">
    <slot name="input"/>
    <!-- <Tooltip> -->
        <!-- <div slot="activator"> -->
            <Question/>
        <!-- </div> -->
        <slot name="questionTip"/>
    <!-- </Tooltip> -->
</div>