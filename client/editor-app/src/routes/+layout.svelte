<script>
	import '../app.pcss';
	import { Navbar, NavBrand, NavLi, NavUl, NavHamburger } from 'flowbite-svelte';
	import { DarkMode } from 'flowbite-svelte';
	import Logo from "$lib/assets/logo.png"
	let loggedIn = true;
	let username = "Suru";

</script>

<nav>
	<Navbar class="border-b">
		<NavBrand href="/">
		  <img src="{Logo}" class="size-16 mr-4" alt="SyncDraft Logo" />
		  <span class="self-center whitespace-nowrap text-2xl font-bold dark:text-white">SyncDraft</span>
		</NavBrand>
		<NavHamburger/>
		<NavUl ulClass="flex items-center space-x-4 text-xl">
		  <NavLi href="/"> Home </NavLi>
		  <NavLi href="/edit"> Editor </NavLi>
		  <NavLi href="/documents"> Documents </NavLi>
		  {#if loggedIn}
			<p>Hello, {username}!</p>
		  {:else}
			<NavLi href="/signup"> Sign Up</NavLi>
			<NavLi href="/login"> Login </NavLi>
		  {/if}
		  <NavLi> <DarkMode/> </NavLi>
		</NavUl>
	</Navbar>
</nav>

<slot />

<style>
</style>
