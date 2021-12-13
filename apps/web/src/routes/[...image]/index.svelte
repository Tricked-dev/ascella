<script context="module" lang="ts">
	import { getImage } from '$lib/api';

	export const ssr = true;
	export const prerender = true;
	export const hydrate = false;
	export const router = true;
	// export const prerender = true;
	/**
	 * @type {import('@sveltejs/kit').Load}
	 */
	export async function load({ page }) {
		const { image } = page.params;
		let params = image?.split('/');

		if (params && params[params?.length - 1].split('.')?.[1]) {
			return;
		}
		let data = await getImage(params[params?.length - 1]);
		if (data.error)
			return {
				status: 302,
				redirect: 'https://ascella.host'
			};
		if (data.redirect && data.content_type === 'redirect') {
			return {
				status: 302,
				redirect: data.redirect
			};
		}
		return {
			props: { ...data, url: params[params?.length - 1] }
		};
	}
</script>

<script lang="ts">
	export let user_name: string;
	export let image_size: string = '10kb';
	export let embed: Record<string, string> = {};
	export let url: string;

	import '../../css/app.scss';
</script>

<svelte:head>
	<title>{user_name} | Image Uploader</title>
	{#if embed.title}
		<meta name="title" content={embed.title} />
		<meta property="og:title" content={embed.title} />
		<meta property="twitter:title" content={embed.title} />
	{/if}
	{#if embed.description}
		<meta name="description" content={embed.description} />
		<meta property="og:description" content={embed.description} />
		<meta property="twitter:description" content={embed.description} />
	{/if}
	{#if embed.author}
		<meta property="site_name" content={embed.author} />
		<meta property="og:site_name" content={embed.author} />
		<meta property="twitter:site_name" content={embed.author} />
	{/if}

	<meta property="twitter:author" content={user_name} />
	<meta property="author" content={user_name} />
	<meta property="theme-color" content={embed.color} />
	<meta property="og:author" content={user_name} />

	<meta property="og:image" content={`https://ascella.wtf/v2/ascella/view/${url}`} />
	<meta property="og:type" content="website" />
	<meta property="twitter:card" content="summary_large_image" />
</svelte:head>

<div class="nav-div">
	<nav class="nav" style={`border-color: ${embed.color || '#00a41b'}`}>
		<ul class="flex-center">
			<li>
				{image_size}
			</li>
		</ul>

		<ul class="flex-center">
			<li>
				<h1 class="pl-8 lg:pl-0"><a href="https://www.ascella.host">Ascella</a></h1>
			</li>
		</ul>

		<ul class="flex-center">
			<li>
				{user_name}
			</li>
		</ul>
	</nav>
</div>

<div class="main">
	<a href={`https://ascella.wtf/v2/ascella/view/${url}`}>
		<img class="image" alt="" src={`https://ascella.wtf/v2/ascella/view/${url}`} />
	</a>
</div>
<footer style={`border-color: ${embed.color || '#00a41b'}`} class="footer">
	<a class="text-larger font-bold" href="https://www.ascella.host">Ascella.host</a>
</footer>

<style lang="postcss">
	.footer {
		@apply bg-gray-700 text-white border-t-2 fixed inset-x-0 bottom-0 h-8 pt-4;
		display: flex;
		align-items: center;
		justify-content: center;
	}
	.nav {
		@apply px-4 flex justify-between bg-gray-700 h-8 border-b-2;
	}
	.nav-div {
		@apply flex-1 flex flex-col;
	}
	.flex-center {
		@apply flex items-center;
	}
	.image {
		margin-top: auto;
		margin-bottom: auto;
		display: flex;
		padding: auto;
		padding-top: auto;
		max-width: 80rem;
		max-height: 60rem;
		width: auto;
		height: auto;
	}
	.main {
		padding-top: 4rem;
		margin-top: auto;
		margin-bottom: auto;
		display: flex;
		justify-content: center;
		align-items: center;
	}
</style>
