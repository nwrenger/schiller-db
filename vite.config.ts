import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import copy from 'rollup-plugin-copy';

export default defineConfig({
	plugins: [
		sveltekit(),
		copy({
			targets: [
				{
					src: 'node_modules/bootstrap/dist/**/*',
					dest: 'static/bootstrap'
				}
			]
		})
	]
});
