<script lang="ts">
	/* Props */
	export let justify: "center"|"left"|"right"|"space-between"|"space-around"|"space-evenly" = "center";
	export let align: "center"|"start"|"end"|"stretch" = "center";
	export let spacing: number = 0;
	export let column: boolean = false;
	export let inline: boolean = false;
</script>

<style>
	/* Main */
	.flex {
		width: 100%;
		height: 100%;

		display: flex;

		transition: gap var(--movement_transition);
	}

	.inline {
		width: fit-content;
		height: fit-content;

		display: inline-flex;
	}
</style>

<div class="flex" class:inline={inline} style="flex-direction: {column ? 'column' : 'row'}; justify-content: {justify}; align-items: {align}; gap: calc(({spacing * spacing} * 0.05 + 0.4) * var(--size));">
	<slot />
</div>