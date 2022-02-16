<script>
	export let disabled = false;
	export let placeholder = '';
	export let value = '';
</script>

{#if disabled}
	<input
		class="bg-indigo-900 cursor-not-allowed p-1 text-gray-500"
		disabled={true}
		{placeholder}
		bind:value
	/>
{:else}
	<input class="bg-indigo-700 p-1" {placeholder} bind:value />
{/if}
