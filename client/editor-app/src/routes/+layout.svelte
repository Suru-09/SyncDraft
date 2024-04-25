<script>
	import '../app.pcss';
	import { page } from '$app/stores';
	import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Button } from 'flowbite-svelte';
	import { DarkMode } from 'flowbite-svelte';
	import { loggedIn } from '../stores.js';
	import { ArrowLeftToBracketOutline } from 'flowbite-svelte-icons';
	import Logo from "$lib/assets/logo.png"
	let username = "Suru";

	const logout = () => {
		$loggedIn = false;
	}

	
	$: activeUrl = $page.url.pathname;
</script>

<nav>
	<Navbar class="border-b">
		<NavBrand href="/">
		  <img src="{Logo}" class="size-16 mr-4" alt="SyncDraft Logo" />
		  <span class="self-center whitespace-nowrap text-2xl font-bold dark:text-white">SyncDraft</span>
		</NavBrand>
		<NavHamburger/>
		<NavUl {activeUrl} ulClass="flex items-center space-x-4 text-xl">
		  <NavLi href="/edit"> Editor </NavLi>
		  <NavLi href="/documents"> Documents </NavLi>
		  {#if $loggedIn}
			<p>Hello, {username}!</p>
			<NavLi>
				<Button color="alternative" class="outline-none border-hidden border-transparent focus:border-transparent focus:ring-0" on:click={logout}>
					<span class="self-center font-normal text-xl whitespace-nowrap mr-3"> Log out </span>
					<ArrowLeftToBracketOutline/>
				</Button>
			</NavLi>
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
