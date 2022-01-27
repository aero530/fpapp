<script lang="ts">

    import ChevronDown from '../icons/ChevronDown.svelte';
    import QuestionField from './QuestionField.svelte'

    export let label: string;
    export let value: string;
    
    /// Paid with taxed income, earnings are not taxed, withdrawals are not taxed
    ///
    /// Contributions count as an expense (will be subtracted from net for the year).
    /// Contributions do not impact taxable income (as they are made with dollars that have already been taxed).
    /// Withdrawals count as income but do not to taxable income.
    /// aka 0
    // ContributeTaxedEarningsUntaxedWhenUsed,
    // contribute_taxed_earnings_untaxed_when_used
    
    /// Paid with taxed income, earnings are taxed in year earned as capital gains, withdrawals are not taxed (tax free as long as used for intended purpose)
    ///
    /// Contributions count as an expense (will be subtracted from net for the year).
    /// Contributions do not impact taxable income (as they are made with dollars that have already been taxed).
    /// Withdrawals count as income but do not to taxable income.
    /// aka 1
    // ContributeTaxedEarningsTaxed,
    // contribute_taxed_earnings_taxed

    // not implemented.
    // NOT IMPLEMENTED ## 2=payed with taxed income, earnings are taxed in year taken out as capital gains, withdrawals are not taxed
    // aka 2
    // NotImpliemented,

    /// Paid with pretax income and taxed in year of use as income
    ///
    /// Contributions count as an expense (will be subtracted from net for the year).
    /// Contributions reduce taxable income (they are a deduction).
    /// Withdrawals count as income and add to taxable income.
    /// aka 3
    // ContributePretaxTaxedWhenUsed,
    // contribute_pretax_taxed_when_used

    /// Paid with pretax income and not taxed as income (use with HSA)
    ///
    /// Contributions count as an expense (will be subtracted from net for the year).
    /// Contributions reduce taxable income (they are a deduction).
    /// Withdrawals count as income but do not add to taxable income.
    /// aka 4
    // ContributePretaxUntaxedWhenUsed,
    // contribute_pretax_untaxed_when_used



    type TaxOption = { value: string; text: string; description: string };
    let items : TaxOption[] = [
        {
            value: 'contribute_taxed_earnings_untaxed_when_used',
            text: 'contribute taxed income - earnings taxed deferred',
            description:
            'payed with taxed income, earnings are tax deferred, withdrawals are not taxed',
        },
        {
            value: 'contribute_taxed_earnings_taxed',
            text: 'contribute taxed income - earings are capital gains',
            description:
            'payed with taxed income, earnings are taxed in year earned as capital gains, withdrawals are not taxed (tax free as long as used for intended purpose)',
        },
        {
            value: 'not_impliemented',
            text: 'not implemented',
            description:
            'NOT IMPLEMENTED ## 2=payed with taxed income, earnings are taxed in year taken out as capital gains, withdrawals are not taxed',
        },
        {
            value: 'contribute_pretax_taxed_when_used',
            text: 'contribute pretax income - taxed as income when used',
            description: 'payed pretax and taxed in year of use as income',
        },
        {
            value: 'contribute_pretax_untaxed_when_used',
            text: 'contribute pretax income - withdrawal not taxed as income (HSA)',
            description: 'payed pretax and not taxed as income (use with HSA)',
        }
    ];

</script>


<QuestionField>
    <div slot="input">
        <select
            label={label}
            bind:value={value}
        >
        {#each items as item}
            <option value={item.value}>{item.text}</option>
        {/each}
    </select>
    </div>
    <div slot="questionTip">
        How taxes impact this account.
        {#each items as item}
            <p>{item.text} : {item.description}</p>
        {/each}
    </div>
</QuestionField>
