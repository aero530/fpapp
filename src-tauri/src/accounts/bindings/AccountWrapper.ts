import type { College } from "./College";
import type { Expense } from "./Expense";
import type { Hsa } from "./Hsa";
import type { Income } from "./Income";
import type { Loan } from "./Loan";
import type { Mortgage } from "./Mortgage";
import type { Retirement } from "./Retirement";
import type { Savings } from "./Savings";
import type { Ssa } from "./Ssa";

export type AccountWrapper = { type: "income" } & Income<string> | { type: "ssa" } & Ssa | { type: "retirement" } & Retirement<string> | { type: "hsa" } & Hsa<string> | { type: "college" } & College<string> | { type: "expense" } & Expense<string> | { type: "loan" } & Loan<string> | { type: "mortgage" } & Mortgage<string> | { type: "savings" } & Savings<string>;