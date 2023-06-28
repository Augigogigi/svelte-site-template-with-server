<script lang="ts">
	/* Imports */
	import { onMount } from "svelte";
	import { browser } from "$app/environment";

	/* Props */
	export let url: string;

	/* Types */
	type SocketState = {
		connected: boolean,
		code: number,
	}

	/* Variables */
	let socket = null;
	export let socketState: SocketState = {
		connected: false,
		code: -1,
	};

	/* Functions */
	export function connectToServer() {
		if (!browser) return;
		if (!url) return;
		if (socket) return;

		console.info("Attempting to connect to [" + url + "]!");

		let timeout = setTimeout(() => {
			socket = null;

			console.info("Timed out trying to connect to [" + url + "]! (5s)");
		}, 5000);

		socket = new WebSocket(url);

		socket.addEventListener("open", () => {
			clearTimeout(timeout);
			console.info("Successfully connected to [" + url + "]!");
		});

		socket.addEventListener("close", (event) => {
			socket = null;

			clearTimeout(timeout);
			console.info("Connection to [" + url + "] closed!\n[" + event.code + " " + (event.wasClean ? "Clean" : "Dirty") + "] Reason: " + (event.reason ? event.reason : "No Reason Provided :("));
		});

		socket.addEventListener("error", (event) => {
			socket = null;

			clearTimeout(timeout);
			console.error("Connection to [" + url + "] closed unexpectedly!", event);
		});

		socket.addEventListener("message", (event) => {
			console.log("Message from server:" + event);
		});
	}

	export function disconnectFromServer() {
		if (!browser) return;
		if (!socket) return;

		socket.close(1000, "User disconnected.");
	}

	export function sendData(data) {
		if (!browser) return;
		if (!socket) return;

		socket.send(data);
	}

	/* Run */
	onMount(() => {
		connectToServer();

		setInterval(() => {
			if (!socket) return;

			socketState = {
				connected: socket.readyState === 1,
				code: socket.readyState,
			};
		}, 200);
	});
</script>

<span></span>