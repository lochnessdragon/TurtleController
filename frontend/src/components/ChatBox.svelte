<script>
	import { onMount } from 'svelte';
	let chat = [];
	let newMessage = "";
	let ws = null;
	
	function addMessage(user, msg) {
		chat.unshift(user + ": " + msg);

		// trigger re render
		chat = chat;

		document.getElementById("chat-box").innerHtml="test"
	}

	function onSubmit(event) {
		addMessage("You", newMessage)
		newMessage = "";
	}

	onMount(async () => {
		addMessage("Info", "Attempting to connect to server!");

		ws = new WebSocket("https://TurtleController.lochnessdragon.repl.co:1234");
	});
</script>

<style>
	.chat {
		overflow: scroll;
		width: 70vw;
		height: 75vh;
		background: whitesmoke;
		display: flex;
 		flex-direction: column-reverse;
		padding: 0;
		margin: 0;
	}
	
	.message-1 {
		background: whitesmoke;
		width: 100%;
	}
	
	.message-2 {
		background: lightgray;
		width: 100%;
	}

	@media (prefers-color-scheme: dark) {
		.chat {
			background: midnightblue;
			color: lime;
		}

		.message-1 {
			background: midnightblue;
		}

		.message-2 {
			background: black;
		}
	}

	.text-scale {
		font-size: 5 em;
	}
</style>

<div>
	<div id="chat-box" class="chat">
		{#each chat as message, i} 
			<p class={"message-" + ((i % 2) + 1)}>{message}</p> 
		{/each}
	</div>
	<form on:submit|preventDefault={onSubmit}>
		<input type="text" style="width: 70vw;" placeholder="Hello chat!" id="message" name="message" class="text-scale" bind:value={newMessage}/>
		<input type="submit" class="text-scale" value="Send">
	</form>
</div>