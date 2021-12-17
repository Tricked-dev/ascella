<script lang="ts">
	import '../../css/github.scss';
	export const ssr = true;
	export const prerender = true;
	export const hydrate = false;
	export const router = false;
</script>

<div class="max-w-[80rem] m-auto prose-invert prose-pink doc">
	<slot />
</div>
