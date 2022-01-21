<script>
    import Textfield from '@smui/textfield';
    import QuestionField from './QuestionField.svelte'

    export let label;
    export let value;
    export let questionText;
</script>


<QuestionField>
    <span slot="input" style="width: 100%;">
        <Textfield
            style="width: 100%;"
            helperLine$style="width: 100%;"
            textarea
            label={label}
            bind:value={value}
        >
    </Textfield>
    </span>
    <span slot="questionTip">
        {questionText}
    </span>
</QuestionField>
