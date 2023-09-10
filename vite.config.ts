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
					src: "node_modules/bootstrap/dist/js/bootstrap.bundle.min.js",
					dest: "static/bootstrap/js"
				},
				{
					src: "node_modules/bootstrap/dist/js/bootstrap.bundle.min.js.map",
					dest: "static/bootstrap/js"
				},
				{
					src: "node_modules/bootstrap/dist/css/bootstrap.min.css",
					dest: "static/bootstrap/css"
				},
				{
					src: "node_modules/bootstrap/dist/css/bootstrap.min.css.map",
					dest: "static/bootstrap/css"
				}
			]
		})
	]
});
