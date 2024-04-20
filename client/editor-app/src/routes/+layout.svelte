<script>
	let loggedIn = false;
</script>

<nav>
	<div class="buttonsContainer">
		<a href="/">Home</a>
		<a href="/edit">Editor</a>
		<a href="/documents">Documents</a>
	</div>
    
</nav>


<style>
	nav {
		background-color: rgb(70, 7, 129);
		display: flex; 
        justify-content: center;
		font-size: x-large;
		color: aliceblue;
	}

	.buttonsContainer {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		width: 30%;
	}
	a {
		text-decoration: solid;
		color: cornflowerblue;
		font-size: xx-large;
	}
	a:hover{
		color: black;
	}
	:global(body) {
		overflow: hidden;
		background-color: rgb(25, 25, 49);
	}
</style>

<slot />