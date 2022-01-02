<script lang="ts">
    export let expression: string;
    export let result: string;
</script>

<div class="container">
    <div class="expression"><span class="arrow">&gt</span> {expression}</div>
    <div class="result"><pre>{result}</pre></div>
</div>

<style>
    .arrow {
        color: #FFB800;
    }

    .expression {
        font-size: 2em;
        color: white;
    }

    .result {
        font-size: 1.5rem;
        color: #2AC327;
    }

    .result pre {
        margin: 0;
        padding: 0;
    }

    .container {
        margin-top: 1rem;
        margin-bottom: 1rem;
        margin-left: .5rem;
    }
</style>
