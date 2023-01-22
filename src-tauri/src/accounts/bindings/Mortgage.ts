import type { PaymentOptions } from "./PaymentOptions";
import type { PercentInput } from "./PercentInput";
import type { Table } from "./Table";
import type { YearInput } from "./YearInput";

export interface Mortgage<T extends string | number> { name: string, table: Table<T>, startOut: YearInput, endOut: YearInput, paymentType: PaymentOptions, paymentValue: number, rate: PercentInput, compoundTime: number, mortgageInsurance: number, ltvLimit: number, escrowValue: number, homeValue: number, notes: string | null, }