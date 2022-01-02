<script lang="ts">
    import {beforeUpdate, afterUpdate} from 'svelte';
    
    let history: string[] = [];

    let current_string: string = "";

    let history_reference: HTMLElement;

    let auto_scroll: boolean;

    beforeUpdate(() => {
        auto_scroll = history_reference && 
            (history_reference.offsetHeight + history_reference.scrollTop) > (history_reference.scrollHeight - 20);
    });

    afterUpdate(() => {
        if (auto_scroll) history_reference.scrollTo(0, history_reference.scrollHeight);
    });

    function addToHistory() {
        history = [
            ...history,
            current_string
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
            <p>{item}</p>
        {/each}
    </div>
    <input class="expression-input" type="text" bind:value={current_string} on:keypress={onKeyPress} />
</div>

<style>
    :global(body) {
        height: 100%;
        margin: 0;
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

    

    .expression-input {
        height: 50px;
    }
</style>
