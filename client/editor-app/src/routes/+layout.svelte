<script>
	let loggedIn = false;
</script>

<nav>
    <a href="/">Home</a>
	<a href="/edit">Editor</a>
	<a href="/documents">Documents</a>
</nav>


<style>
	nav {
		background-color: rgb(70, 7, 129);
		display: flex; 
        justify-content: center;
        align-items: center;
		font-size: x-large;
		color: aliceblue;
		border-radius: 50;
		
	}
	a {
		margin-left: 5%;
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