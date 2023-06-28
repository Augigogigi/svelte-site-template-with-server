<script lang="ts">
	/* Props */
	export let override = {};

	export let width: "fit"|"fill"|number = "fill";
	export let height: "fit"|"fill"|number = "fill";
	export let inline: boolean = false;

	/* Variables */
	let element;

	/* Functions */
	function propToSize(size: "fit"|"fill"|number) {
		switch (size) {
			case "fit":
				return "fit-content";
			case "fill":
				return "100%";
			default:
				return "calc((" + (size * size) +" * 0.05 + 0.4) * var(--size))";
		}
	}

	/* Run */
	$: {
		if (element) {
			Object.assign(element.style, override);
		}
	}
</script>

<style>
	/* Main */
	.box {
		transition: width var(--movement_transition), height var(--movement_transition);
	}
</style>

{#if inline}
	<span bind:this={element} class="box" style="width: {propToSize(width)}; height: {propToSize(height)};">
		<slot />
	</span>
{:else}
	<div bind:this={element} class="box" style="width: {propToSize(width)}; height: {propToSize(height)};">
		<slot />
	</div>
{/if}