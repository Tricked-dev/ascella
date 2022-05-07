<script>
	import '../css/app.scss';
	//Feel free to add yourself to this
	const contributors = [
		{
			name: 'Stef',
			discord: 'stef#6470',
			description: 'Rethemed the site',
			type: ['Website']
		},
		{
			name: 'Abd',
			discord: 'Abd#6933',
			description: 'Fixed the navbar and various other improvements',
			type: ['Website']
		},
		{
			name: 'Bumble',
			discord: 'bumble#0001',
			description: 'Fixed many typos in the site',
			type: ['Website']
		}
	];
</script>

<div class="max-w-[80rem] m-auto">
	<div class="text-center py-5 text-yellow-400 text-3xl">
		<h1>Contributors who helped make this site a better</h1>
	</div>
	<div class="justify-center flex">
		<div class="grid auto-rows-auto gap-4 auto-cols-auto w-96">
			{#each contributors as contributor}
				<div
					class="py-4 duration-200 border-4 border-yellow-500 bg-amber-700 w-50 justify-center items-center text-center m-auto contributor p-4 w-full"
				>
					<img
						class="w-40 h-40 m-auto border-yellow-500 border-2"
						alt={contributor.name}
						src={`/assets/contributors/${contributor.name.toLowerCase()}.png`}
					/>
					<div class="pt  -4">
						<h3 class="text-2xl">
							{contributor.name}
						</h3>
						<p class="">{contributor.description}</p>
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>
