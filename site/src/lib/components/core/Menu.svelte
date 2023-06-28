<script lang="ts">
	/* Imports */
	import Popper from "$lib/components/layout/Popper.svelte";
	import MovementDetector from "$lib/components/meta/MovementDetector.svelte";

	/* Props */
	export let popperPosition: "left"|"right"|"top"|"bottom" = "bottom";
	export let popperAlignment: "start"|"center"|"end" = "center";
	export let popperGutter: number = 0;

	/* Variables */
	let popperBtn;
	let popperOpen = false;

	let popperCallback;

	/* Functions */
	function setPopperBtn(btn) {
		popperBtn = btn;
	}

	function movementCallback() {
		if (popperCallback) {
			popperCallback();
		}
	}
</script>

<style>
	.btn {
		width: fit-content;
		height: fit-content;
	}
</style>

<div>
	<MovementDetector callback={movementCallback}>
		<div class="btn" use:setPopperBtn on:click={() => {popperOpen = !popperOpen}}>
			<slot name="button" />
		</div>
	</MovementDetector>
	<Popper bind:refreshCallback={popperCallback} target={popperBtn} active={popperOpen} position={popperPosition} alignment={popperAlignment} gutter={popperGutter}> <!-- TODO: btnwidth, btnheight -->
		<slot />
	</Popper>
</div>