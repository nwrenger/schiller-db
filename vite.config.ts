import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import copy from "rollup-plugin-copy";

export default defineConfig({
	server: {
		proxy: {
			"/api": {
				target: "http://127.0.0.1:8000"
			}
		}
	},
	plugins: [
		sveltekit(),
		copy({
			targets: [
				{
					src: "node_modules/bootstrap/dist/*",
					dest: "static/bootstrap"
				}
			]
		})
	]
});
