<script lang="ts">
	import { getReviews, getStats } from '$lib/api';
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
	$: stats = {};
	onMount(async () => {
		const [module, revs, stat] = await Promise.all([
			import('svelte-carousel'),
			getReviews(),
			getStats()
		]);
		Carousel = module.default;
		reviews = revs;
		stats = stat;
	});
</script>

<div class="mx-auto">
	<div class="">
		<div class="text-center py-24 px-2 bg-gradient-to-tr from-zinc-900 to-gray-900 pb-20">
			<h1 class="text-7xl text-white p-1 font-extrabold">Ascella</h1>
			<h2 class="text-white p-1 pb-10">
				A <b>fast</b> image uploader made for <b>all</b> platforms.
			</h2>
			<a href="https://docs.ascella.host/signup">
				<button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full duration-150"
					>Get started</button
				>
			</a>
		</div>
		{#if stats.total_users}
			<div class="flex md:flex-row flex-col justify-center gap-4 md:p-10 p-4">
				<div
					class="p-8 bg-slate-800 rounded-lg text-white text-center lg:px-40 md:px-16
 px-10"
				>
					<p><b class="cursor-default">Users</b></p>
					<p class="p-1 cursor-default">{stats.total_users}</p>
				</div>
				<div
					class="p-8 bg-slate-800 rounded-lg text-white text-center lg:px-40 md:px-16
 px-10"
				>
					<p><b class="cursor-default">Uploads</b></p>
					<p class="p-1 cursor-default">{stats.total_uploads}</p>
				</div>
				<div class="p-8 bg-slate-800 rounded-lg text-white text-center lg:px-40 md:px-16 px-10">
					<p><b class="cursor-default">Domains</b></p>
					<p class="p-1 cursor-default">{stats.total_domains}</p>
				</div>
			</div>
		{/if}
		<div>
			{#each features as feature, index}
				<div
					class="md md:flex-row flex-col flex text-zinc-200 bg-gradient-to-br from-slate-500 to-slate-600 hover:text-slate-200 md:duration-200 md:p-10 p-5 md:m-10 m-5 md:h-96 rounded-lg duration-150"
				>
					<div class="w-2/3">
						<h1 class="md:text-6xl text-2xl decoration-slate-400 text-slate-50 pb-6 cursor-default">
							<b>{feature.title}</b>
						</h1>
						<h3 class="md:text-3xl text-xl cursor-default pb-8">
							{feature.description}
						</h3>
					</div>
					<div class="md:w-1/3 md:pl-6 md:h-auto h-52 flex justify-center overflow-hidden">
						<img
							alt={feature.description}
							src={feature.image}
							class="w-max object-cover object-left rounded-lg"
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
							<div class="flex text-white text-lg gap-2 bg-[#36393F] p-2 h-full rounded-lg">
								<img
									class="rounded-[50%] p-2 w-20 h-20"
									src={`${review.avatar}`}
									alt={review.name}
									on:error={(event) => {
										event.target.src = 'https://cdn.discordapp.com/embed/avatars/5.png';
										event.onerror = null;
									}}
								/>
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
