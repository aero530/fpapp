<script>
    import QuestionField from './QuestionField.svelte'

    export let label;
    export let value;
    export let questionText="";
</script>

<QuestionField>
    <div slot="input">
        <input
            label={label}
            bind:value={value}
        />
    </div>
    <div slot="questionTip">
        {questionText}
    </div>
</QuestionField>
