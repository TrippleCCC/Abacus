<script lang="ts">
    import { afterUpdate } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';

    interface HistoryItem {
        expression: string;
        result: string;
    }
    
    let history: HistoryItem[] = [];

    let current_string: string = "";

    let history_reference: HTMLElement;

    afterUpdate(() => {
        history_reference.scrollTo(0, history_reference.scrollHeight);
    });

    async function addToHistory() {
        var result: string;

        try {
            result = await invoke('eval', { expression: current_string });
            console.log(result);
        } catch (err) {
            result = err;
            console.log(err);
        }

        history = [
            ...history,
            { expression: current_string, result }
        ];

        current_string = "";
    }

    function onKeyPress(e: KeyboardEvent) {
        if (e.key === "Enter") {
            addToHistory();
        }
    }
</script>

<div class="screen">
    <div bind:this={history_reference} class="history-container">
        {#each history as item}
            <div>
                <p>{item.expression}</p>
                <p>{item.result}</p>
            </div>
        {/each}
    </div>
    <input class="expression-input" type="text" bind:value={current_string} on:keypress={onKeyPress} />
</div>

<style>
    @import url('https://fonts.googleapis.com/css2?family=Roboto+Mono&display=swap');

    :global(body) {
        height: 100%;
        margin: 0;
        font-family: 'Roboto Mono', monospace;
    }

    :global(body #svelte) {
        height: 100%;
        margin: 0;
    }

    :global(html) {
        height: 100%;
        display: flex;
        flex-flow: column;
    }

    .screen {
        display: flex;
        flex-flow: column;
        height: 100%;
        justify-content: flex-end;
    }

    .history-container {
        height: 100%;
        background-color: blue;
        color: white;
        overflow: auto;
    }

    .history-container p {
        font-size: 40px;
    }

    .expression-input {
        height: 50px;
        font-size: 40px;
    }
</style>
