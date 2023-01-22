import type { PercentInput } from "./PercentInput";
import type { Table } from "./Table";
import type { YearInput } from "./YearInput";

export interface Income<T extends string | number> { name: string, table: Table<T>, base: number, startIn: YearInput, endIn: YearInput, raise: PercentInput, notes: string | null, }