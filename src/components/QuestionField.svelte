<script lang="ts">
    import QuestionHelper from './QuestionHelper.svelte'
</script>

<!-- style="display: flex; flex-wrap: wrap; align-items: center;" -->
<!-- class="container" -->
<div class="container" style="width: 100%;">
    <slot name="input"/>
    <QuestionHelper>
        <slot name="questionTip"/>
    </QuestionHelper>
</div>