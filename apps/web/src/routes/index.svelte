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
			title: 'What are you waiting for?',
			description:
				"Start now by joining the Discord, asking for a code, and redeeming it! It's that simple.",
			image: '/redeem.png'
		}
	];
	import '../css/index.scss';
</script>

<div>
	<div class="m-auto">
		<div class="text-center">
			<p class="text-3xl text-white">Welcome to the Ascella uploader</p>
			<p class="text-xl font-bold text-white">
				Looking to score an invite? Join the <a
					class="text-red-600"
					href="https://discord.gg/mY8zTARu4g">Discord.</a
				>
			</p>
		</div>
		<div class="box-container">
			{#each features as feature}
				<div class="box">
					<div class="pb-2 max-w-full">
						<img alt="" src={feature.image} class="max-w-full object-cover w-full h-40" />
					</div>
					<div class="text-white w-full h-full max-w-full css-tips-with-abd">
						<div>
							<p class="text-xl title">{feature.title}</p>
						</div>
						<div>
							<p class="desc">{feature.description}</p>
						</div>
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>
