<script lang="ts">
	import { getReviews } from '$lib/api';
	import { media } from '$lib/media';
	import { onMount } from 'svelte';
	
	import '../css/app.scss';


	let Carousel;
	let carousel;

	const handleNextClick = () => {
		carousel.goToNext();
	};

	const features: any[] = [
		{
			title: 'Easily change your settings',
			description:
				"You can change everything via Discord,\nmaking it the perfect image uploader for people who don't want to visit a confusing or cluttered site to just change some settings.",
			image: '/embeds.png'
		},
		{
			title: 'Changing domains',
			description:
				'You can easily change your domain from inside Discord with a single slash command.',
			image: '/domains.png'
		},
		{
			title: 'Dashboard',
			description: 'A simple dashboard to change your domain and other settings.',
			image: '/dashboard.png'
		},
		{
			title: 'Open Source',
			description: "Ascella's code is fully opensource, and you can view the code on github.",
			image: '/github.png'
		},
		{
			title: 'Desktop App',
			description:
				'Ascella has a open source desktop app for both Linux and MacOS, allowing you to make screenshots on all major operating systems.',
			image: '/desktop.png'
		},
		{
			title: 'What are you waiting for?',
			description:
				"Start now by joining the Discord, asking for a code, and redeeming it! It's that simple.",
			image: '/redeem.png'
		}
	];
	import '../css/index.scss';

	$: reviews = [];
	onMount(async () => {
		const [module, revs] = await Promise.all([import('svelte-carousel'), getReviews()]);
		Carousel = module.default;
		reviews = revs;
	});
</script>

<div class="mx-auto">
	<div class="">
		<div class="text-center py-6 px-2 bg-gradient-to-tr from-zinc-900 to-gray-900 pb-10">
			<h1 class="text-7xl text-white p-1 font-extrabold">Ascella</h1>
			<h2 class="text-white p-1 pb-10">A <b>Fast</b> Image uploader made for <b>all</b> platforms</h2>
			<a href="https://docs.ascella.host/signup">
			<button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full">Get started</button>
			</a>
		</div>
		<div>
			{#each features as feature, index}
				<div
					class="duration-75 md md:flex-row flex-col flex bg-gradient-to-r from-cyan-500 to-blue-500 hover:from-cyan-400 hover:to-blue-400 md:duration-200 md:p-10 p-5 md:m-10 m-5 md:h-96"
				>
					<div class="w-2/3">
						<h1 class="md:text-6xl text-2xl underline decoration-red-500 text-slate-50 pb-6">
							{feature.title}
						</h1>
						<h3 class="md:text-3xl text-xl text-sky-200">{feature.description}</h3>
					</div>
					<div class="md:w-1/3 md:pl-6 md:h-auto h-52 flex justify-center overflow-hidden">
						<img
							alt={feature.description}
							src={feature.image}
							class="w-max object-cover object-left"
						/>
					</div>
				</div>
			{/each}
		</div>
		{#if reviews.length !== 0}
			<div class="p-4">
				<svelte:component
					this={Carousel}
					bind:this={carousel}
					class="w-auto"
					particlesToShow={$media.laptop ? 3 : $media.tablet ? 2 : 1}
					particlesToScroll={$media.laptop || $media.tablet ? 2 : 1}
					autoplay
					autoplayDuration={3500}
					autoplayProgressVisible
					pauseOnFocus
				>
					{#each reviews as review}
						<div>
							<div class="flex text-white text-lg gap-2 bg-[#36393F] p-2 h-full">
								<img class="rounded-[50%] p-2 w-20 h-20" src={`${review.avatar}`} alt={review.name} on:error={(event)=> {
									event.target.src = "https://cdn.discordapp.com/embed/avatars/5.png"
  event.onerror = null
								}} />
								<div>
									<p class="text-1xl font-bold text-amber-400">{review.name}</p>
									<p class="text-white text-sm	 text-">{review.comment}</p>
								</div>
							</div>
						</div>
					{/each}
				</svelte:component>
			</div>
		{/if}

	
	</div>
</div>
