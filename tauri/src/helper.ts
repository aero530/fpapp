export function addTableRow(store, id: string, table: string, year: string, value: number) {
    store.addTableRow(id, table, year, value);
}

export function removeTableRow(store, id: string, table: string, year: string) {
    store.removeTableRow(id, table, year);
}