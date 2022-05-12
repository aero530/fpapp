<div class="grid grid-rows-1 gap-2 border rounded p-4 bg-background-600 dark:bg-darkbackground-600">
    <div class="flex flex-wrap gap-2">
        <div class="flex-initial w-fit">
            <slot name="inputs"/>
        </div>
        <div class="flex-auto w-[32rem]">
            <slot name="chart"/>
        </div>
    </div>
    <div class="flex flex-wrap gap-4">
        <slot name="balance"/>
        <slot name="emp_contributions"/>
        <slot name="contributions"/>
        <slot name="earnings"/>
        <slot name="withdrawals"/>
    </div>
</div>

