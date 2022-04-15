<script>
	let chat = ["yes", "Is this from the server?"]
	let newMessage = ""
	
	function addMessage(msg) {
		chat.unshift(msg);

		// trigger re render
		chat = chat;

		document.getElementById("chat-box").innerHtml="test"
	}

	function onSubmit(event) {
		addMessage(newMessage)
		newMessage = "";
	}
</script>

<style>
	.chat {
		overflow: scroll;
		width: 70vw;
		height: 75vh;
		background: whitesmoke;
		display: flex;
 		flex-direction: column-reverse;
	}
	
	.message-1 {
		background: whitesmoke;
	}
	
	.message-2 {
		background: lightgray;
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