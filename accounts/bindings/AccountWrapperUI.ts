import type { College } from "./College";
import type { Expense } from "./Expense";
import type { Hsa } from "./Hsa";
import type { Income } from "./Income";
import type { Loan } from "./Loan";
import type { Mortgage } from "./Mortgage";
import type { Retirement } from "./Retirement";
import type { Savings } from "./Savings";
import type { Ssa } from "./Ssa";

export type AccountWrapperUI = { type: "income" } & Income<number> | { type: "ssa" } & Ssa | { type: "retirement" } & Retirement<number> | { type: "hsa" } & Hsa<number> | { type: "college" } & College<number> | { type: "expense" } & Expense<number> | { type: "loan" } & Loan<number> | { type: "mortgage" } & Mortgage<number> | { type: "savings" } & Savings<number>;