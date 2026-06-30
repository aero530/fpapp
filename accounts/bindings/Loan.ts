import type { PaymentOptions } from "./PaymentOptions";
import type { PercentInput } from "./PercentInput";
import type { Table } from "./Table";
import type { YearInput } from "./YearInput";

export interface Loan<T> { name: string, table: Table<T>, startOut: YearInput, endOut: YearInput, paymentType: PaymentOptions, paymentValue: number, rate: PercentInput, notes: string | null, }