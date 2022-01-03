<script lang="ts">
    import { afterUpdate, onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import HistoryItem from '$lib/HistoryItem.svelte';
    import { GetPrevExpression, history_store, AddToHistory } from '$lib/HistoryStore';

    let current_string: string = "";
    let saved_string: string;

    let history_reference: HTMLElement;
    let input_reference: HTMLInputElement;

    let history_counter: number = 0;

    onMount(() => {
        input_reference.focus();
    });

    afterUpdate(() => {
        history_reference.scrollTo(0, history_reference.scrollHeight);
    });

    async function addToHistory() {
        var result: string;

        try {
            result = await invoke('eval', { expression: current_string });
            // console.log(result);
        } catch (err) {
            result = err;
            // console.log(err);
        }

        if (result === "") {
            result = " ";
        }

        AddToHistory(current_string, result);

        current_string = "";
    }

    function onKeyPress(e: KeyboardEvent) {
        if (e.key === "Enter")
            addToHistory();
        else if (e.key === "ArrowUp" || e.key === "Up")
            onKeyUp();
        else if (e.key === "ArrowDown" || e.key === "Down")
            onKeyDown();
    }

    function onKeyUp() {
        if (history_counter == 0)
            saved_string = current_string;

        history_counter = history_counter + 1;

        let history_item = GetPrevExpression(history_counter);
        if (history_item == null) {
            history_counter = history_counter - 1;
        } else {
            current_string = history_item;
        }
    }

    function onKeyDown() {
        if (history_counter == 0) {
            return;
        }

        history_counter -= 1;

        if (history_counter == 0) {
            // Set the current_string to the saved string
            current_string = saved_string;
        } else {
            current_string = GetPrevExpression(history_counter);
        }
    }
</script>

<div class="screen">
    <div bind:this={history_reference} class="history-container">
        {#each $history_store as item}
            <HistoryItem {...item} />
        {/each}
    </div>
    <input class="expression-input" type="text" bind:this={input_reference} bind:value={current_string} on:keyup={onKeyPress} />
</div>

<style>
    @import url('https://fonts.googleapis.com/css2?family=Roboto+Mono&display=swap');

    :global(html) {
        height: 100%;
        display: flex;
        flex-flow: column;
        font-family: 'Roboto Mono', monospace;
        font-size: 20px;
    }

    :global(body) {
        height: 100%;
        margin: 0;
    }

    :global(body #svelte) {
        height: 100%;
        margin: 0;
    }

    .screen {
        display: flex;
        flex-flow: column;
        height: 100%;
        justify-content: flex-end;
    }

    .history-container {
        height: 100%;
        background-color: #23384D;
        overflow: auto;
    }

    .expression-input {
        all: unset;
        font-family: 'Roboto Mono', monospace;

        height: 50px;
        font-size: 2rem;
        background-color: #1a2a3b;
        color: white;
    }

    .expression-input:focus {
        outline: none;
    }
</style>
