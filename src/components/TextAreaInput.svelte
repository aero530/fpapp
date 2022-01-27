<script>
    import QuestionField from './QuestionField.svelte'

    export let label;
    export let value;
    export let questionText;
</script>


<QuestionField>
    <div slot="input" style="width: 100%;">
        <textarea
            style="width: 100%;"
            textarea
            label={label}
            bind:value={value}
        />
    </div>
    <div slot="questionTip">
        {questionText}
    </div>
</QuestionField>
