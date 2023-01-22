import type { ExpenseOptions } from "./ExpenseOptions";
import type { Table } from "./Table";
import type { YearInput } from "./YearInput";

export interface Expense<T extends string | number> { name: string, table: Table<T>, startOut: YearInput, endOut: YearInput, expenseType: ExpenseOptions, expenseValue: number, isHealthcare: boolean, hsaLink: string | null, notes: string | null, }