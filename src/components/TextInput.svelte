<script>
    import QuestionField from './QuestionField.svelte'

    export let label;
    export let value="asdf";
    export let questionText="";
</script>

<QuestionField {questionText} {label}>
    <input
        type='text'
        bind:value={value}
        class="p-1 m-1 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400"
    />
</QuestionField>
