<script>
	/* Theme + Server Management */
	import ThemeManager from "$lib/components/meta/ThemeManager.svelte";
	import ServerManager from "$lib/components/meta/ServerManager.svelte";

	let themeManager = undefined;
	let currentStyle = {
		layout: "regular",
		theme: "light",
	};

	let serverManager = undefined;
	let socketState = {
		connected: false,
		code: -1,
	};

	/* Imports */
	import { onMount } from "svelte";

	import Box from "$lib/components/layout/Box.svelte";
	import Center from "$lib/components/layout/Center.svelte";
	import Column from "$lib/components/layout/Column.svelte";
	import Flex from "$lib/components/layout/Flex.svelte";
	import Menu from "$lib/components/core/Menu.svelte";
	import Padding from "$lib/components/layout/Padding.svelte";
	import Popper from "$lib/components/layout/Popper.svelte";
	import Row from "$lib/components/layout/Row.svelte";
	import MovementDetector from "$lib/components/meta/MovementDetector.svelte";
	import Button from "$lib/components/core/Button.svelte";
	import Text from "$lib/components/core/Text.svelte";

	onMount(() => {

	});
</script>

<style>
	/* Responsiveness */
	@media screen and (max-width: 1152px) {
		:root {
			font-size: 14px;
		}
	}

	@media screen and (max-width: 1000px) {
		:root {
			font-size: 12px;
		}
	}

	@media screen and (max-width: 750px) {
		:root {
			font-size: 10px;
		}
	}

	/* Main */
	.main {
		background-color: var(--block1);

		width: 100%;
		height: 100%;

		overflow: hidden;

		transition: background-color var(--color_transition);
	}
</style>

<ThemeManager bind:this={themeManager} bind:currentStyle={currentStyle} />
<!--
<ServerManager bind:this={serverManager} bind:socketState={socketState} url="ws://127.0.0.1/server" />
-->
<div class="main">
	<Column>
		<Box height=fit>
			<Padding size=1>
				<Row spacing=1>
					<Button on:click={() => {themeManager.changeStyle({theme: "light"})}}>
						<Text>Light</Text>
					</Button>
					<Button on:click={() => {themeManager.changeStyle({theme: "dark"})}}>
						<Text>Dark</Text>
					</Button>
					<Button on:click={() => {themeManager.changeStyle({layout: "regular"})}}>
						<Text>Regular</Text>
					</Button>
					<Button on:click={() => {themeManager.changeStyle({layout: "compact"})}}>
						<Text>Compact</Text>
					</Button>
				</Row>
			</Padding>
		</Box>
		<Center>
			<slot />
		</Center>
	</Column>
</div>