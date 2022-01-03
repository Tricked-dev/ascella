<script lang="ts">
	import '../../css/github.scss';
	import '@fontsource/righteous';
	export const ssr = true;
	export const prerender = true;
	export const hydrate = false;
	export const router = false;
</script>

<div
	class="prose prose-emerald prose-invert doc max-w-[80rem] prose-headings:hover:text-emerald-500 m-auto lg:prose-2xl md:prose-lg doc prose-headings:no-underline"
>
	<slot />
</div>
