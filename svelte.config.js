import adapter from '@sveltejs/adapter-auto';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter(),
		files: {
			assets: 'static',
			lib: 'src/lib',
			routes: 'src/routes'
		},
		prerender: {
			enabled: false
		},
	}
};

export default config;
