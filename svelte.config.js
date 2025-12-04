import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    // Consult https://svelte.dev/docs/kit/integrations
    // for more information about preprocessors
    preprocess: vitePreprocess(),

    kit: {
        // adapter-auto only supports some environments, see https://svelte.dev/docs/kit/adapter-auto for a list.
        // If your environment is not supported, or you settled on a specific environment, switch out the adapter.
        // See https://svelte.dev/docs/kit/adapters for more information about adapters.
        adapter: adapter({
            pages: 'dist',
            assets: 'dist',
            fallback: undefined,
            precompress: false,
            strict: true
        }),
        prerender: {
            handleHttpError: ({ path, referrer, message }) => {
                // ignore favicon errors
                if (path === '/favicon.png') {
                    return;
                }
                throw new Error(message);
            }
        }
    }
};

export default config;