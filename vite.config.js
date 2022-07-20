import { sveltekit } from '@sveltejs/kit/vite';

/** @type {import('vite').UserConfig} */
const config = {
	plugins: [sveltekit()],
	// server: {
	// 	fs: {
	// 		searchForWorkspaceRoot(process.cwd()),
	// 		allow: ['photos']
	// 	}
	// }
};

export default config;
