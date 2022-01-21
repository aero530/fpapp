<script>
    import Textfield from '@smui/textfield';
    import QuestionField from './QuestionField.svelte'

    export let label;
    export let value;
    export let questionText;
</script>


<QuestionField>
    <span slot="input">
        <Textfield
            label={label}
            bind:value={value}
        >
    </Textfield>
    </span>
    <span slot="questionTip">
        {questionText}
    </span>
</QuestionField>
