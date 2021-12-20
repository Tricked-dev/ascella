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
  <nav>
    <div class="all">
      <a class="ascella" href="https://ascella.host">Ascella.host</a>
      <div class="btns">
        <ul>
          <li><a class="btn" href="https://dash.ascella.host" > Dashboard </a></li>
          <li><a class="btn" href="/"> Home </a></li>
          <li><a class="btn" href="https://discord.gg/mY8zTARu4g" > Discord </a></li>
        </ul>
      </div>
        <div class="other">
          <input id="input" type="checkbox" checked>
          <label for="input">Other
            <svg class="w-5 h-5 -mr-1" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          </label>
          <div class="list">
            <ul>
				    {#each otherLinks as link}
              <li>
                <div>
                  <slide><a
				  	  	    href={link.href}
				  	    	  tabindex="0"
				  	  	    role="menuitem">{link.a}</a
				  	      ></slide>
                  <a
				  	  	    href={link.href}
				  	    	  tabindex="0"
				  	  	    role="menuitem">{link.a}</a
				  	      >
                </div>
              </li>
				    {/each}
				    </ul>      
          </div>
        </div>
      <a class="btn" href="/docs/signup">Create an account</a>
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
	/* .a-btn {
		@apply duration-500 block border-b-4 border-blue-300 hover:border-teal-500 mt-4 lg:inline-block lg:mt-0 hover:text-white px-4 py-2 rounded hover:bg-teal-800 mr-2;
	}
	.dropdown:focus-within .dropdown-menu {
		opacity: 1;
		transform: translate(0) scale(1);
		visibility: visible;
	} */
</style>
