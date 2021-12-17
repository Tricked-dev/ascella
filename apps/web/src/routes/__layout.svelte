<script context="module" lang="ts">
</script>

<script lang="ts">
	import '../css/app.scss';
	import '../css/roboto.scss';
	const bottomLinks = [
		[
			{
				href: '/docs/faq',
				a: 'Faq'
			},
			{
				href: '/docs/rules',
				a: 'Rules'
			},
			{
				href: '/docs/privacy',
				a: 'Privacy'
			}
		],
		[
			{
				href: 'https://github.com/Tricked-dev/ascella',
				a: 'Github'
			}
		],
		[
			{
				href: 'https://dash.ascella.host',
				a: 'Dashboard'
			}
		],
		[
			{
				href: 'https://discord.gg/mY8zTARu4g',
				a: 'Discord'
			},
			{
				a: 'Build by tricked#3777'
			}
		]
	];
	const otherLinks = [
		{
			href: '/docs/domain',
			a: 'Adding your own domain'
		},
		{
			href: '/docs/rules',
			a: 'Rules'
		},
		{
			href: '/docs/privacy',
			a: 'Privacy'
		},
		{
			href: '/docs/',
			a: 'Docs Index'
		},
		{
			href: '/docs/installing',
			a: 'Installing'
		},
		{
			href: '/domains',
			a: 'Domains'
		}
	];

	import { onMount } from 'svelte';
	import { fly } from 'svelte/transition';
	import { goto } from '$app/navigation';

	let show = false; // menu state
	let menu = null; // menu wrapper DOM reference
	onMount(() => {
		show = false;
		const handleOutsideClick = (event) => {
			if (menu && show && !menu.contains(event.target)) {
				show = false;
			}
		};

		const handleEscape = (event) => {
			if (show && event.key === 'Escape') {
				show = false;
			}
		};

		// add events when element is added to the DOM
		document.addEventListener('click', handleOutsideClick, false);
		document.addEventListener('keyup', handleEscape, false);

		// remove events when element is removed from the DOM
		return () => {
			document.removeEventListener('click', handleOutsideClick, false);
			document.removeEventListener('keyup', handleEscape, false);
		};
	});
	let meta: Record<string, string> = {
		type: 'website',
		description:
			'Ascella Uploader is an extremely fast image uploader built with the newest technologies.  ' +
			'Ascella is OpenSource you can find the source code at https://github.com/Tricked-dev/ascella',
		title: 'Ascella Uploader - The fastest image uploader',
		keywords: 'javascript typescript programming discord matrix',
		viewport: 'width=device-width,initial-scale=1,minimum-scale=1,maximum-scale=1,user-scalable=no',
		site_name: 'tricked#3777',
		locale: 'en_US'
	};
</script>

<svelte:head>
	<title>Ascella Image Uploader</title>
	{#each ['type', 'site_name', 'description', 'title', 'locale'] as item}
		<meta property={`og:${item}`} content={meta[item]} />
		<meta property={`twitter:${item}`} content={meta[item]} />
		<meta property={`${item}`} content={meta[item]} />
	{/each}

	{#if meta.image}
		<meta property="og:image" content={meta.image} />
		<meta name="twitter:card" content="summary_large_image" />
	{/if}

	<meta property="theme-color" content="#1b8aeb" />
	<meta property="robots" content="index,follow" />
	<meta property="googlebot" content="index,follow" />
	<meta property="viewport" content="width=device-width" />
</svelte:head>

<template bind:this={menu} class="flex-1 flex flex-col">
	<nav
		class="flex items-center justify-between flex-wrap bg-slate-700 py-4 lg:px-12 shadow border-solid border-t-2 border-lime-500"
	>
		<div
			class="flex justify-between lg:w-auto w-full lg:border-b-0 pl-6 pr-2 border-solid border-b-2 border-gray-300 pb-5 lg:pb-0"
		>
			<div class="flex items-center flex-shrink-0 text-slate-100 mr-16">
				<span
					class="font-semibold text-xl tracking-tight hover:text-lime-500 duration-200 hover:border-b-2 border-indigo-700"
					><a href="https://ascella.host">Ascella.host</a></span
				>
			</div>
		</div>

		<div class="menu w-full flex-grow lg:flex lg:items-center lg:w-auto lg:px-3 px-8">
			<div class="lg:text-lg text-md font-bold text-green-500  lg:flex-grow">
				<a href="https://dash.ascella.host" class="a-btn"> Dashboard </a>
				<a href="/" class="a-btn"> Home </a>
				<a href="https://discord.gg/mY8zTARu4g" class="a-btn"> Discord </a>

				<div class=" relative inline-block text-left dropdown">
					<span class="rounded-md shadow-sm"
						><button
							class="inline-flex justify-center w-full px-4 py-2 leading-5 bg-slate-500 text-white hover:bg-slate-700 transition duration-150 ease-in-out border border-gray-300 hover:text-gray-100 focus:outline-none focus:border-blue-300 focus:shadow-outline-blue active:text-gray-800"
							type="button"
							aria-haspopup="true"
							aria-expanded="true"
							aria-controls="headlessui-menu-items-117"
						>
							<span>Other</span>
							<svg class="w-5 h-5 ml-2 -mr-1" viewBox="0 0 20 20" fill="currentColor"
								><path
									fill-rule="evenodd"
									d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
									clip-rule="evenodd"
								/></svg
							>
						</button></span
					>
					<div
						class="opacity-0 invisible dropdown-menu transition-all duration-300 transform origin-top-right -translate-y-2 scale-95"
					>
						<div
							class="absolute left-0 w-56 mt-2 origin-top-left  bg-slate-600 border border-gray-500 divide-y divide-gray-100 rounded-md shadow-lg outline-none"
							aria-labelledby="headlessui-menu-button-1"
							id="headlessui-menu-items-117"
							role="menu"
						>
							<div class="py-1">
								{#each otherLinks as link}
									<a
										href={link.href}
										tabindex="0"
										class="text-gray-200 flex justify-between w-full px-4 py-2 text-sm leading-5 text-left"
										role="menuitem">{link.a}</a
									>
								{/each}
							</div>
						</div>
					</div>
				</div>
			</div>

			<div class="flex">
				<a
					href="/docs/signup"
					class="a-btn duration-500 border-b-4 border-blue-300 hover:border-orange-600 lg:text-lg text-md block px-4 py-2 rounded text-green-700 ml-2 font-bold hover:text-white mt-4 hover:bg-blue-700 lg:mt-0"
					>Create an account</a
				>
			</div>
		</div>
	</nav>

	<div class="pb-4" />
	<slot />

	<div class="pb-7" />
	<div
		class="transform no-underline text-gray-500 hover:text-white min-h-full border-t-2 border-gray-600 hover:border-white pt-6 footer"
		transition:fly={{ x: 1500, y: 0, duration: 800 }}
	>
		<div class="flex-auto flex list-none">
			{#each bottomLinks as links}
				<div>
					<u>
						{#each links as link}
							<li>
								<a href={link.href} class="px-4 py-2 not-sr-only hover:text-blue-500 translate-x-"
									>{link.a}</a
								>
							</li>
						{/each}
					</u>
				</div>
			{/each}
		</div>
	</div>
</template>

<style lang="postcss">
	.a-btn {
		@apply duration-500 block border-b-4 border-blue-300 hover:border-teal-500 mt-4 lg:inline-block lg:mt-0 hover:text-white px-4 py-2 rounded hover:bg-teal-800 mr-2;
	}
	.dropdown:focus-within .dropdown-menu {
		opacity: 1;
		transform: translate(0) scale(1);
		visibility: visible;
	}
</style>
