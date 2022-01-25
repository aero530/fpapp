<script>
    import TextField from '@smui/textfield';
    import QuestionField from './QuestionField.svelte'

    export let label;
    export let value;
    export let questionText;
</script>


<QuestionField>
    <span slot="input">
        <TextField
            label={label}
            bind:value={value}
        >
    </TextField>
    </span>
    <span slot="questionTip">
        {questionText}
    </span>
</QuestionField>
