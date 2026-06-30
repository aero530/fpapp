import type { Settings } from "./Settings";

export interface UserData<T> { settings: Settings, accounts: Record<string, T>, }