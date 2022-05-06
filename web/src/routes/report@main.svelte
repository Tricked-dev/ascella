<script lang="ts">
	import { page } from '$app/stores';
	let val = parseInt($page.url.searchParams.get('image'));
	let image = isNaN(val) ? undefined : val;
	let r;
</script>

<form
	on:submit|preventDefault={async () => {
		let v = await fetch('https://ascella.wtf/v2/ascella/report', {
			method: 'POST',
			body: JSON.stringify({
				id: parseInt(image.toString())
			})
		});
		r = await v.text();
	}}
>
	<p>Report a image</p>
	<input placeholder="image" bind:value={image} />
	<button>Confirm</button>
	{#if r}
		<p>{r}</p>
	{/if}
</form>
