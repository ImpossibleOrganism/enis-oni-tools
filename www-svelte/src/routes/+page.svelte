<script lang="ts">
    import { greet } from 'oni-tools';

    function withWASM() {
        greet();
    }

    function noWASM() {
        alert("Hello from TypeScript!");
    }
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

<button onclick="{withWASM}">WASM!</button>
<button onclick={noWASM}>WASM-less</button>
