<script lang="ts">
	let _class = '';
	export { _class as class };
</script>

<button
	class={`bg-blue-500 hover:bg-blue-400 text-white font-bold py-2 px-4 border-b-4 border-blue-700 hover:border-blue-500rounded ${_class}`}
>
	<slot />
</button>
