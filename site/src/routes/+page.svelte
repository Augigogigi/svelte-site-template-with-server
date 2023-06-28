<script>
	import { slide } from 'svelte/transition';
	import Menu from "$lib/components/core/Menu.svelte";
	import Box from "$lib/components/layout/Box.svelte";
	import Padding from "$lib/components/layout/Padding.svelte";
	import Button from "$lib/components/core/Button.svelte";
	import Text from "$lib/components/core/Text.svelte";
</script>

<Menu popperPosition="bottom" popperAlignment="center" popperGutter=1>
	<Button slot="button">
		<Text>Menu</Text>
	</Button>
	<Box override={{backgroundColor: "#AAA"}}>
		<Padding size=3>
			<Text>Hello</Text>
		</Padding>
	</Box>
</Menu>