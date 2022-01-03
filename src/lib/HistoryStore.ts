import { writable, get } from "svelte/store";

export interface HistoryItem {
    expression: string;
    result: string;
}

let history_store = writable([]);

function AddToHistory(expression: string, result: string) {
    history_store.update(history => [...history, {expression, result}]);
}

function GetPrevExpression(index: number): null | string {
    const history = get(history_store);
    if (history.length == 0 || history.length < index) {
        return null;
    }

    return history.at(-index).expression;
}

export { AddToHistory, GetPrevExpression, history_store };
