<nav>
    <a href="/">home</a>
	<a href="/edit">editor</a>
	<a href="/documents">documents</a>
</nav>

<style>
	nav {
		background-color: rgb(31, 4, 56);
		display: flex; 
        justify-content: center;
        align-items: center;
		font-size: x-large;
		color: aliceblue;
		border-radius: 50;
		
	}

	a {
		margin-left: 5%;
	}

	:global(body) {
		overflow: hidden;
		background-color: rgb(45, 45, 87);
	}
</style>

<slot />