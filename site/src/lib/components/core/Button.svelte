<script lang="ts">
	import { createEventDispatcher } from "svelte/internal";
	import Padding from "$lib/components/layout/Padding.svelte";

	export let disabled: boolean = false;

	const dispatcher = createEventDispatcher();

	function dispatchAll(e) {
		for (const key in e) {
			if (!key.startsWith("on")) continue;
			const _key = key.slice(2);
			e[key] = (event) => {
				if (disabled) return;
				dispatcher(_key, event);
			};
		}
	}
</script>

<style>
	.button {
		border: solid 1px var(--line1);
		border-radius: var(--unit);

		cursor: pointer;
		user-select: none;

		transition: background-color var(--color_transition), border-color var(--color_transition);
	}
</style>

<div use:dispatchAll class="button">
	<Padding size=2 vertical>
		<Padding size=1 horizontal>
			<slot />
		</Padding>
	</Padding>
</div>