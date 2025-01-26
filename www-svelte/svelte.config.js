import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter({
			// These are ostensibly the default options.
			// Directory to write pre-rendered pages to
			// pages: 'build',
			// Directory to write static assets to
			// assets: 'build',
			// Fallback page for single-page application mode
			// Not sure that I understand why those concepts are related.
			// Single page application could be useful, but might not mesh
			//  well with git submodules.
			// fallback: undefined,
			// Compresses everything into .br and .gz files
			// precompress: false,
			// Make sure all parts of the site are successfully pre-rendered.
			// strict: true
		})
	}
};

export default config;
