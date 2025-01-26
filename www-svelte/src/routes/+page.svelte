<script lang="ts">
    function noWASM() {
        alert("Hello from TypeScript!");
    }
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

<button>WASM!</button>
<button onclick={noWASM}>WASM-less</button>
