import adapter from '@sveltejs/adapter-node';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			// default options are shown
			out: 'build',
			precompress: false,
			envPrefix: ''
		}),
		files: {
			assets: 'static',
			lib: 'src/lib',
			routes: 'src/routes'
		},
		prerender: {
			enabled: false
		},
		paths: {
			assets: 'https://connectiveunconscious.com/'
		}
	}
};

export default config;
