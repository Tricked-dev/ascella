<script context="module" lang="ts">
	import type { Load } from '@sveltejs/kit';

	import SwaggerUISvelte from '../components/swagger/SwaggerUI.svelte';
	export const load: Load = async ({ fetch }) => {
		let swagger = await fetch('/swagger.json').then((r) => r.text());
		return {
			props: { swagger: JSON.parse(swagger) }
		};
	};
</script>

<script>
	export let swagger;
</script>

<div class="bg-white">
	<SwaggerUISvelte {swagger} url={'/swagger.json'} />
</div>
