<script lang="ts">
	/* Props */
	export let size: number;
	export let horizontal: boolean = false;
	export let vertical: boolean = false;
	export let inline: boolean = false;

	/* Run */
	if (!horizontal && !vertical) {
		horizontal = true;
		vertical = true;
	}
</script>

<style>
	/* Main */
	.padding {
		width: 100%;
		height: 100%;

		transition: padding var(--movement_transition);
	}

	.horizontal {
		padding-top: var(--pad);
		padding-bottom: var(--pad);
	}

	.vertical {
		padding-left: var(--pad);
		padding-right: var(--pad);
	}

	.inline {
		display: inline;

		width: fit-content;
		height: fit-content;
	}
</style>

<div class="padding" class:horizontal={horizontal} class:vertical={vertical} class:inline={inline} style="--pad: calc(({size * size} * 0.05 + 0.4) * var(--size));">
	<slot />
</div>