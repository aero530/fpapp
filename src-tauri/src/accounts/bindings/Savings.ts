import type { ContributionOptions } from "./ContributionOptions";
import type { PercentInput } from "./PercentInput";
import type { Table } from "./Table";
import type { TaxStatus } from "./TaxStatus";
import type { WithdrawalOptions } from "./WithdrawalOptions";
import type { YearInput } from "./YearInput";

export interface Savings<T extends string | number> { name: string, table: Table<T>, contributions: Table<T> | null, earnings: Table<T> | null, withdrawals: Table<T> | null, startIn: YearInput, endIn: YearInput, startOut: YearInput, endOut: YearInput, contributionValue: number, contributionType: ContributionOptions, yearlyReturn: PercentInput, withdrawalType: WithdrawalOptions, withdrawalValue: number, taxStatus: TaxStatus, notes: string | null, }