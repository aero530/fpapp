import type { ContributionOptions } from "./ContributionOptions";
import type { EmployerMatch } from "./EmployerMatch";
import type { PercentInput } from "./PercentInput";
import type { Table } from "./Table";
import type { TaxStatus } from "./TaxStatus";
import type { WithdrawalOptions } from "./WithdrawalOptions";
import type { YearInput } from "./YearInput";

export interface Retirement<T extends string | number> { name: string, table: Table<T>, contributions: Table<T> | null, earnings: Table<T> | null, withdrawals: Table<T> | null, employerContributions: Table<T> | null, startIn: YearInput, endIn: YearInput, startOut: YearInput, endOut: YearInput, contributionValue: number, contributionType: ContributionOptions, yearlyReturn: PercentInput, withdrawalType: WithdrawalOptions, withdrawalValue: number, taxStatus: TaxStatus, incomeLink: string | null, matching: EmployerMatch | null, notes: string | null, }