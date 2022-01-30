<script lang="ts">
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
</script>

<div class="mx-auto md:p-4 p-2">
	<div class="">
		<div class="text-center">
			<h1 class="text-4xl underline text-sky-600">Welcome to the Ascella uploader</h1>
			<h2 class="text-slate-100 text-xl">
				Looking to score an invite? Join the <a
					class="underline text-green-500"
					href="https://discord.gg/mY8zTARu4g">Discord.</a
				>
			</h2>
		</div>
		<div
			class="grid xl:grid-cols-3 md:grid-cols-2 grid-cols-1 gap-2 rounded-xl justify-center justify-items-center"
		>
			{#each features as feature, index}
				<div
					class={`md:flex w-full p-2 ${
						index % 2 ? 'md:ml-auto md:mr-0' : 'md:mr-auto md:ml-0 md:flex-row-reverse'
					}`}
				>
					<div class="m-auto p-1">
						<h3 class="text-7xl text-sky-700">{feature.name}</h3>
						<p class="text-2xl dark:text-sky-500 text-blue-800">{feature.description}</p>
					</div>
					<div class="m-auto p-1">
						<img class="h-96 object-cover" alt={feature.name} src={`${feature.image}`} />
					</div>
				</div>
				<!-- <div
					class="p-4 bg-slate-600 shadow-lg mx-auto hover:shadow-2xl hover:shadow-green-500/50 w-full"
				>
					<div class="">
						<img alt="" src={feature.image} class="w-full h-40 object-cover" />
					</div>
					<div class="">
						<div>
							<p class="text-xl text-cyan-400">{feature.title}</p>
						</div>
						<div>
							<p class="text-gray-400">{feature.description}</p>
						</div>
					</div>
				</div> -->
			{/each}
		</div>
	</div>
</div>
