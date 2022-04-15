<script>
	let chat = ["Hello World!", "Is this from the server?"]
	
	function addMessage(msg) {
		chat.push(msg);

		// trigger re render
		chat = chat;
	}

	function onSubmit(event) {
		formData = new FormData(event.target);

		addMessage("help")
	}
</script>

<style>
	.chat {
		overflow: scroll;
		width: 70vw;
		height: 75vh;
		background: whitesmoke;
	}
	
	.message-1 {
		background: whitesmoke;
	}
	
	.message-2 {
		background: lightgray;
	}
</style>

<div>
	<div id="chat-box" class="chat">
		{#each chat as message} <p>{message}</p> {/each}
	</div>
	<form on:submit|preventDefault={onSubmit}>
		<input type="text" style="width: 70vw;" placeholder="Hello chat!" id="message" name="message"/>
		<input type="submit" value="Send">
	</form>
</div>