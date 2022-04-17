<script context="module" lang="ts">
	import { getImage } from '$lib/api';
	import type { Load, Handle } from '@sveltejs/kit';
	import { onMount } from 'svelte';
	export const prerender = true;
	export const hydrate = false;
	export const router = true;

	export const load: Load = async ({ params }) => {
		const { image } = params;
		let imgParams = image?.split('/');

		if (imgParams && imgParams[imgParams?.length - 1].split('.')?.[1]) {
			return {
				status: 302,
				redirect: 'https://ascella.host'
			};
		}
		let data = await getImage(imgParams[imgParams?.length - 1]);
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
			props: { ...data, url: imgParams[imgParams?.length - 1] }
		};
	};
	onMount(() => {
		//@ts-ignore -
		if (umami) umami.trackView('/image');
	});
</script>

<script lang="ts">
	export let user_name: string;
	export let image_size: string = '10kb';
	export let embed: Record<string, string> = {};
	export let url: string;
	export let views: string;
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

	<meta property="og:image" content={`https://ascella.wtf/v2/ascella/view/${url}.png`} />
	<meta property="og:type" content="website" />
	<meta property="twitter:card" content="summary_large_image" />

	<script
		async
		defer
		data-website-id="9d7a10ea-0ef7-4e4a-959f-bfe22fc26cfd"
		data-auto-track="false"
		src="https://analytics.tricked.pro/umami.js"></script>
</svelte:head>

<div class="flex-1 flex flex-col">
	<nav
		class=" px-4 flex justify-between bg-gray-700 h-8 border-b-2 text-white"
		style={`border-color: ${embed.color || '#00a41b'}`}
	>
		<ul class="flex items-center">
			<li>
				{image_size}
			</li>
		</ul>

		<ul class="flex items-center">
			<li>
				<h1 class="pl-8 lg:pl-0"><a href="https://www.ascella.host">Ascella</a></h1>
			</li>
		</ul>

		<ul class="flex items-center">
			<li>
				{user_name}
			</li>
		</ul>
	</nav>
</div>

<div class="main">
	<div class="box mx-auto">
		<a href={`https://ascella.wtf/v2/ascella/view/${url}.png`} target="_blank">
			<img class="image" alt="" src={`https://ascella.wtf/v2/ascella/view/${url}.png`} />
		</a>
		<p>Views {views}</p>
	</div>
</div>
<footer
	style={`border-color: ${embed.color || '#00a41b'}`}
	class="footer bg-gray-700 text-white border-t-2 fixed inset-x-0 bottom-0 h-8 pt-4 pb-2"
>
	<a class="text-larger font-bold" href="https://www.ascella.host">Ascella.host</a>
</footer>

<style lang="postcss">
	.footer {
		display: flex;
		align-items: center;
		justify-content: center;
	}
	.image {
		object-fit: contain;
		height: 100%;
		width: 100%;
	}
	.box {
		width: 40rem;
		height: 55rem;
		justify-content: center;
	}
	.main {
		padding-top: 4rem;
		display: grid;
		justify-content: center;
	}
</style>
