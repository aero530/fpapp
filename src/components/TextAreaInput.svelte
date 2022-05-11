<script>
    import QuestionField from './QuestionField.svelte'

    export let label;
    export let value="";
    export let questionText="";
</script>


<QuestionField {questionText} {label}>
    <textarea
        textarea
        bind:value={value}
        class="p-0 m-0 pl-1 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400"
    />
</QuestionField>
