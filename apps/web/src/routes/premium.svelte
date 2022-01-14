<script>
	const payments = [
		{
			name: 'XMR',
			long: 'Monaro',
			addr: '48i3VBVmL2RQSeN5ezmqxXLv4CbYPhGcDVGQNRTQxy3v26xRxz2m5PbQpnFDXpuD25Dup6w1RHD1LTEEYREaRaWET6oV1fZ'
		},
		{
			name: 'BCH',
			long: 'BitcoinCash',
			addr: 'qp6r9xknvf2dh3q0lhc4h0qv843uegacdvdkp75nx4'
		},
		{
			name: 'ETH',
			long: 'Etherium',
			addr: '0xc31a1A5dCd1a4704e81fB7c9C3fa858b9A00C7fb'
		},
		{
			name: 'BTC',
			long: 'Bitcoin',
			addr: 'bc1qm7nh7ftfcuhk7kal24xat3gnel5mzzrs4kymsl'
		},
		{
			name: 'LTC',
			long: 'Litecoin',
			addr: 'ltc1qjg6mjgt0ys0grzhtvlm9sl68a670zka2y7xmw4'
		}
	];
	const perks = [
		{
			name: 'Invite Ascella',
			desc: 'Allows you to add the Ascella bot to one of your Discord servers',
			price: 5,
			perm: true
		}
		// {
		// 	name: 'Redirects',
		// 	desc: 'Gives you access to the redirects functionality from the bot',
		// 	price: 2,
		// 	perm: false
		// }
	];
	let active = false;
	let thing;
	function onClick(perk) {
		active = true;
		thing = perk;
	}
	function close() {
		active = !active;
	}
</script>

<div>
	{#if active}
		<div
			class="overflow-x-hidden overflow-y-auto fixed inset-0 z-50 outline-none focus:outline-none justify-center items-center flex "
		>
			<div class="relative w-2/4 my-6 mx-auto max-w-3xl">
				<div
					class="border-0 rounded-lg shadow-lg relative flex flex-col w-full bg-gray-700 outline-none focus:outline-none"
				>
					<div
						class="flex items-start justify-between p-5 border-b border-solid border-gray-200 rounded-t"
					>
						<h3 class="text-3xl text-sky-200 font-semibold">{thing.name}</h3>
					</div>
					<div class="p-6">
						<p>
							To get the reward please pay {thing.price} euro to in crypto to one of the following addresses
							and send a discord dm to tricked#3777 to claim your reward
						</p>
						<div class="grid auto-rows-max max-w-fit gap-2 py-4">
							{#each payments as payment}
								<div class=" p-4 bg-teal-800 border-4 border-teal-700 rounded-md max-w-2xl">
									<div>
										<p>
											{payment.name}
											<strong class="text-sm text-teal-500 font-normal">
												{payment.long}
											</strong>
										</p>
									</div>
									<div>
										<pre class="text-black break-words whitespace-pre-wrap">
                                        	{payment.addr}
                                    	</pre>
									</div>
								</div>
							{/each}
						</div>
					</div>
					<div
						class="flex items-center justify-end p-6 border-t border-solid border-blueGray-200 rounded-b"
					>
						<button label="Cancel" on:click={close}> Close </button>
					</div>
				</div>
			</div>
		</div>
		<div class="opacity-25 fixed inset-0 z-40 bg-black" />
	{/if}
	<div class="flex justify-center gap-4">
		{#each perks as perk}
			<div class="bg-sky-700 rounded-md border-sky-500 p-4 w-96 max-w-full flex flex-col">
				<div>
					<div class="pb-6">
						<h1 class="text-xl text-sky-100">{perk.name}</h1>
						<p>{perk.desc}</p>
					</div>
					<div class="mt-auto">
						<p>Price <strong class="font-bold">{perk.price} €</strong></p>
						<p>{perk.perm ? 'One time' : 'Monthly'}</p>
					</div>
				</div>

				<div class="mt-auto self-end">
					<button
						class="relative w-full inline-flex items-center justify-center p-0.5 mb-2 mr-2 overflow-hidden text-sm font-medium text-sky-300 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white text-white focus:ring-4 focus:ring-blue-300 focus:ring-blue-800"
						on:click={() => {
							onClick(perk);
						}}
					>
						<span
							class="w-full relative px-5 py-2.5 transition-all ease-in duration-75 bg-sky-700 rounded-md group-hover:bg-opacity-0"
						>
							Buy now
						</span></button
					>
				</div>
			</div>
		{/each}
	</div>
</div>
