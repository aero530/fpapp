import type { SsaSettings } from "./SsaSettings";

export interface Settings { ageRetire: number, ageDie: number, yearBorn: number, yearStart: number, inflationBase: number, taxIncome: number, taxCapitalGains: number, retirementCostOfLiving: number, ssa: SsaSettings, }