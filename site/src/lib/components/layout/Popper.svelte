<script lang="ts">
	/* Imports */
	import { browser } from "$app/environment";

	import Padding from "$lib/components/layout/Padding.svelte";

	/* Props */
	export let target: HTMLElement;
	export let active: boolean = false;

	export let position: "left"|"right"|"top"|"bottom" = "bottom";
	export let alignment: "start"|"center"|"end" = "center";
	export let gutter: number = 0;

	/* Variables */
	let self: HTMLElement;

	let is_vertical: boolean = position === "top" || position === "bottom";

	let x: number = 0;
	let y: number = 0;

	/* Functions */
	function setSelf(node) {
		self = node;
	}

	export function refreshCallback() {
		if (!browser) return;
		if (!target) return;
		if (!self) return;

		is_vertical = position === "top" || position === "bottom";

		let pos_main: number;
		let pos_other: number;

		switch (position) {
			case "left":
				pos_main = target.offsetLeft - self.offsetWidth;
				break;
			case "right":
				pos_main = target.offsetLeft + target.offsetWidth;
				break;
			case "top":
				pos_main = target.offsetTop - self.offsetHeight;
				break;
			case "bottom":
				pos_main = target.offsetTop + target.offsetHeight;
				break;
		}

		switch (alignment) {
			case "start":
				pos_other = is_vertical ?
					target.offsetLeft :
					target.offsetTop;
				break;
			case "center":
				pos_other = is_vertical ?
					target.offsetLeft + (target.offsetWidth / 2) - (self.offsetWidth / 2) :
					target.offsetTop + (target.offsetHeight / 2) - (self.offsetHeight / 2);
				break;
			case "end":
				pos_other = is_vertical ?
					target.offsetLeft + target.offsetWidth - self.offsetWidth :
					target.offsetTop + target.offsetHeight - self.offsetHeight;
				break;
		}

		x = is_vertical ? pos_other : pos_main;
		y = is_vertical ? pos_main : pos_other;
	}

	/* Run */
	$: if (target && self && active) {
		refreshCallback();
	}
</script>

<style>
	.popper {
		position: absolute;
	}
</style>

{#if active}
	<div use:setSelf class="popper" style="left: {x}px; top: {y}px;">
		<Padding size={gutter} horizontal={is_vertical} vertical={!is_vertical}>
			<slot/>
		</Padding>
	</div>
{/if}