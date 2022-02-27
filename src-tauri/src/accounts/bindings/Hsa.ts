import type { ContributionOptions } from "./ContributionOptions";
import type { PercentInput } from "./PercentInput";
import type { Table } from "./Table";
import type { TaxStatus } from "./TaxStatus";
import type { WithdrawalOptions } from "./WithdrawalOptions";
import type { YearInput } from "./YearInput";

export interface Hsa<T> { name: string, table: Table<T>, startIn: YearInput, endIn: YearInput, startOut: YearInput, endOut: YearInput, contributionValue: number, contributionType: ContributionOptions, employerContribution: number, yearlyReturn: PercentInput, withdrawalType: WithdrawalOptions, withdrawalValue: number, taxStatus: TaxStatus, notes: string | null, }