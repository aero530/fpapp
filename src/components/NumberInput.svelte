<script lang="ts">
    
    import QuestionField from './QuestionField.svelte'

    export let label : string;
    export let value : number;
    export let questionText : string="";
    export let step : number = 1;
    let inputValue : string = '';

    function handleUpdate() {
        bind:value = round(parseFloat(inputValue), step);
    }

    function round(number: number, increment: number) {
        return Math.ceil((number) / increment ) * increment;
    }
</script>

<QuestionField>
    <div slot="input">
        <input
            label={label}
            bind:value={inputValue}
            type="number"
            on:change={() => handleUpdate()}
        />
    </div>
    <div slot="questionTip">
        {questionText}
    </div>
</QuestionField>