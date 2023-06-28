<script lang="ts">
	/* Imports */
	import { onMount } from "svelte";

	/* Types */
	type Style = {
		layout: string,
		theme: string,
	};

	/* Variables */
	const defaultStyle: Style = {
		layout: "regular",
		theme: "light"
	};
	export let currentStyle: Style = defaultStyle;

	const LayoutList = ["regular", "compact"] as const;
	const ThemeList = ["dark", "light"] as const;

	/* Functions */
	export function changeStyle(options: Partial<Style>) {
		if (options.layout) {
			if (LayoutList.indexOf(currentStyle.layout) > -1) {
				currentStyle.layout = options.layout;
			}

			document.documentElement.setAttribute("layout", currentStyle.layout);
			localStorage.setItem("layout", currentStyle.layout);
		}

		if (options.theme) {
			if (ThemeList.indexOf(currentStyle.theme) > -1) {
				currentStyle.theme = options.theme;
			}

			document.documentElement.setAttribute("theme", currentStyle.theme);
			localStorage.setItem('theme', currentStyle.theme);
		}
	}

	/* Run */
	onMount(() => {
		let storedLayout: string = localStorage.getItem('layout') ?? defaultStyle.layout;
		if (LayoutList.indexOf(storedLayout) > -1) {
			currentStyle.layout = storedLayout;
		}

		let storedTheme: string = localStorage.getItem('theme') ?? defaultStyle.theme;
		if (ThemeList.indexOf(storedTheme) > -1) {
			currentStyle.theme = storedTheme;
		}

		changeStyle({
			layout: currentStyle.layout,
			theme: currentStyle.theme,
		});

		setTimeout(() => {
			document.body.classList.remove("preload");
		}, 32);
	});
</script>

<style>
	/* Default Styling */
	:global(*) {
		padding: 0;
		margin: 0;

		box-sizing: border-box;

		--color_transition: 0.2s;
		--movement_transition: 0.1s;
	}

	:global(html), :global(body) {
		width: 100%;
		height: 100%;
	}

	:global(.preload *) {
		transition: none !important;
	}

	/* Layouts */
	:global(html[layout="regular"]) {
		--size: 1rem;
		--unit: 0.3rem;
	}

	:global(html[layout="compact"]) {
		--size: 0.6rem;
		--unit: 0.3rem;
	}

	/* Themes */
	:global(html[theme="light"]) {
		--block1: #F3F3F8;
		--block2: #EEEEF3;
		--block3: #E9E9EE;
		--block4: #E4E4E9;
		--block5: #DFDFE4;

		--line1: #7E7F88;
		--line2: #AEAFB6;

		--contrast: #272A31;

		--loading: #54428E;
		--info: #0C93F9;
		--success: #05B761;
		--warning: #FCBC46;
		--danger: #E02C35;
	}

	:global(html[theme="dark"]) {
		--block1: #212429;
		--block2: #24272D;
		--block3: #272A31;
		--block4: #2A2D35;
		--block5: #2D3039;

		--line1: #AEAFB6;
		--line2: #7E7F88;

		--contrast: #E9E9EE;

		--loading: #54428E;
		--info: #0C93F9;
		--success: #05B761;
		--warning: #FCBC46;
		--danger: #E02C35;
	}
</style>

<span></span>