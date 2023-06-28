<script lang="ts">
	/* Props */
	export let callback = undefined;

	/* Variables */
	let target: HTMLElement;
	let self: HTMLElement;

	/* Functions */
	function updateDetector(target, self) {
		if (!target || !self) return;

		let movementChecker = document.createElement("div");
		Object.assign(movementChecker.style, {
			position: "relative",
			pointerEvents: "none",
			width: target.offsetWidth + "px",
			height: target.offsetHeight + "px",
			top: -target.offsetHeight + "px",
		});
		self.appendChild(movementChecker);

		const recalculatePosition = () => {
			Object.assign(self.style, {
				width: target.offsetWidth + "px",
				height: target.offsetHeight + "px",
			});

			Object.assign(movementChecker.style, {
				width: target.offsetWidth + "px",
				height: target.offsetHeight + "px",
				top: -target.offsetHeight + "px",
			});
		};
		recalculatePosition();

		let intersection_observer = new IntersectionObserver(
			(entries) => {
				for (const entry of entries) {
					recalculatePosition();
					callback();
				}
			},
			{
				root: self,
				threshold: [0, 0.99, 0.999, 1],
			}
		)
		intersection_observer.observe(movementChecker);

		let resize_observer = new ResizeObserver(
			(entries) => {
				for (const entry of entries) {
					recalculatePosition();
					callback();
				}
			},
		)
		resize_observer.observe(target);
		resize_observer.observe(document.documentElement);
	}

	$: updateDetector(target, self);
</script>

<style>
	.target {
		width: fit-content;
		height: fit-content;
	}
</style>

<div bind:this={self}>
	<div class="target" bind:this={target}>
		<slot />
	</div>
</div>