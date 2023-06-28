<script lang="ts">
	/* Imports */
	import Flex from "$lib/components/layout/Flex.svelte";

	/* Props */
	export let spacing: number = 0;
	export let inline: boolean = false;
</script>

<Flex {spacing} {inline} column>
	<slot />
</Flex>