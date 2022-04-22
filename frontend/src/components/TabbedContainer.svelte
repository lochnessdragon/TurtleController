<script>
	let tabs = [{name:"Tab 1", content:"Hello World!"}, {name:"Mother", content:"The mothership"}, {name:"Tabby", content:"Hi! I'm new on this block. uwu!"}];

	function addTab() {
		tabs.push({name:"Tabby", content:"Hi! I'm new on this block. uwu!"});

		tabs = tabs;
	}
</script>

<style>
	.tab-container {
		display: flex;
		flex-direction: row;
		overflow: auto;
	}

	.tab-button {
		background: gray;
		color: white;
		border-style:solid;
		border: 1px;
		border-color: red;
		border-radius: 4px 4px 0px 0px;
		padding: 4px;
		margin-left: 1px;
		margin-right: 1px;
		flex: 1 0 auto
	}

	.tab-button:hover {
		background: black;
	}
</style>

<div style="border-color:black; border:3px; border-style:solid;max-width:100vw;min-width:100vw;">
	<div class="tab-container">
		{#each tabs as tab}
			<span class="tab-button">{tab.name}</span>
		{/each}
		<span class="tab-button" on:click={addTab()}>New Connection</span>
	</div>
</div>