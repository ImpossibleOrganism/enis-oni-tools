import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';
import wasm from "vite-plugin-wasm";

export default defineConfig({
	plugins: [
		wasm(),
		sveltekit()
	],
	build: {
		target: "esnext"
	}
});
