<script>
    import Textfield from '@smui/textfield';
    import QuestionField from './QuestionField.svelte'

    export let label;
    export let value;
    export let questionText;
    export let step;
</script>


<QuestionField>
    <span slot="input">
        <Textfield
            label={label}
            bind:value={value}
            type="number"
            input$step={step}
        >
    </Textfield>
    </span>
    <span slot="questionTip">
        {questionText}
    </span>
</QuestionField>
