<script>
    import QuestionField from './QuestionField.svelte'

    export let label;
    export let value="";
    export let questionText="";
</script>


<QuestionField {questionText} {label}>
    <textarea
        style="width: 100%;"
        textarea
        bind:value={value}
        class="p-0 m-0 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400"
    />
</QuestionField>
