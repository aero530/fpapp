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
        class="p-0 m-0 pl-1 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400"
    />
</QuestionField>
