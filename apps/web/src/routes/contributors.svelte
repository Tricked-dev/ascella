<script>
	//Feel free to add yourself to this
	const contributors = [
		{
			name: 'Stef',
			discord: 'stef#6470',
			description: 'Rethemed the site',
			type: ['Website']
		},
		{
			name: 'abd',
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
	<div class="text-lg text-center py-5">
		<h1>Contributors who helped make this site a better</h1>
	</div>
	<div class="grid auto-rows-auto gap-4 auto-cols-auto">
		{#each contributors as contributor}
			<div
				class="py-4 bg-gray-700 hover:bg-slate-600 duration-300 border-4 border-lime-500 w-50 justify-center items-center text-center m-auto contributor p-4"
			>
				<img
					class="border-2 border-lime-200 w-40 h-40 m-auto"
					alt={contributor.name}
					src={`/assets/contributors/${contributor.name.toLowerCase()}.png`}
				/>
				<div class="pt  -4">
					<h3 class="text-2xl">
						{contributor.name}
					</h3>
					<p class="text-teal-400">{contributor.description}</p>
				</div>
			</div>
		{/each}
	</div>
</div>
