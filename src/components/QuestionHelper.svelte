<script lang="ts">
    import Tooltip, { Wrapper, Content } from '@smui/tooltip';
    import Question from '../icons/Question.svelte';
</script>

<Wrapper rich>
    <Question/>
    <Tooltip>
        <Content>
            <slot />
        </Content>
    </Tooltip>
</Wrapper>