<script>
	import '../app.pcss';
	import { page } from '$app/stores';
	import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Button } from 'flowbite-svelte';
    import { FacebookSolid, GithubSolid, DiscordSolid, TwitterSolid } from 'flowbite-svelte-icons';
	import { DarkMode } from 'flowbite-svelte';
	import { loggedIn, userDocuments, loggedUser,isAnyDocEdited, currentEditingDocument, usersList } from '../stores.js';
	import { ArrowLeftToBracketOutline } from 'flowbite-svelte-icons';
	import Logo from "$lib/assets/logo.png"

	let username = $loggedUser.firstName;

	const logout = () => {
		$loggedIn = false;
		$userDocuments = [];
		$loggedUser = {firstName: "", lastName: "", username: "", password: ""};
        $isAnyDocEdited = false;
        $currentEditingDocument = {
            "_id": "",
            "doc_name": "",
            "doc_owner": "",
            "body": "",
        };
        $usersList = [];
		location.reload();
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


<footer class="fixed bottom-0 right-0 left-0 w-full p-4 bg-white border-t border-gray-200 shadow md:flex md:items-center md:justify-between md:p-6 dark:bg-gray-800 dark:border-gray-600">
    <div class="mx-auto w-full p-10 py-6 lg:py-1">
        <div class="md:flex md:justify-between">
          <div class="mb-6 md:mb-0">
              <a href="https://flowbite.com/" class="flex items-center">
                  <img src="{Logo}" class="size-16 mr-4"  alt="SyncDraft Logo" />
                  <span class="self-center text-3xl font-semibold whitespace-nowrap dark:text-white">SyncDraft</span>
              </a>
          </div>
          <div class="grid grid-cols-2 gap-8 sm:gap-6 sm:grid-cols-3">
              <div>
                  <h2 class="mb-6 text-sm font-semibold text-gray-900 uppercase dark:text-white">Resources</h2>
                  <ul class="text-gray-500 dark:text-gray-400 font-medium">
                      <li class="mb-4">
                          <a href="https://flowbite.com/" class="hover:underline">Flowbite</a>
                      </li>
                      <li>
                          <a href="https://tailwindcss.com/" class="hover:underline">Tailwind CSS</a>
                      </li>
                  </ul>
              </div>
              <div>
                  <h2 class="mb-6 text-sm font-semibold text-gray-900 uppercase dark:text-white">Follow us</h2>
                  <ul class="text-gray-500 dark:text-gray-400 font-medium">
                      <li class="mb-4">
                          <a href="https://github.com/Suru-09" target="_blank" rel="noopener noreferrer" class="hover:underline ">Github</a>
                      </li>
                      <li>
                          <a href="https://discord.com/download" target="_blank" rel="noopener noreferrer" class="hover:underline">Discord</a>
                      </li>
                  </ul>
              </div>
              <div>
                  <h2 class="mb-6 text-sm font-semibold text-gray-900 uppercase dark:text-white">Legal</h2>
                  <ul class="text-gray-500 dark:text-gray-400 font-medium">
                      <li class="mb-4">
                          <a href="https://www.termsfeed.com/live/4d0d282b-22ab-419c-9013-c673c90586b6" target="_blank" rel="noopener noreferrer" class="hover:underline">Privacy Policy</a>
                      </li>
                      <li>
                          <a href="https://www.termsfeed.com/live/8143e1ad-d5ee-40b4-a0bc-14dd7cc1b8da" target="_blank" rel="noopener noreferrer" class="hover:underline">Terms &amp; Conditions</a>
                      </li>
                  </ul>
              </div>
          </div>
      </div>
      <hr class="my-6 border-gray-200 sm:mx-auto dark:border-gray-700 lg:my-5" />
      <div class="sm:flex sm:items-center sm:justify-between ">
          <span class="text-sm text-gray-500 sm:text-center dark:text-gray-400">©2024 <a href="https://flowbite.com/" class="hover:underline">SyncDraft</a>. All Rights Reserved.
          </span>
          <div class="flex mt-4 sm:justify-center sm:mt-0">
              <a href="https://www.facebook.com/" target="_blank" rel="noopener noreferrer" class="text-gray-500 hover:text-gray-900 dark:hover:text-white">
                <FacebookSolid size="xl"/>
              </a>
              <a href="https://discord.com/download" target="_blank" rel="noopener noreferrer" class="text-gray-500 hover:text-gray-900 dark:hover:text-white ms-5">
                <DiscordSolid size="xl"/>
              </a>
              <a href="https://github.com/Suru-09" target="_blank" rel="noopener noreferrer" class="text-gray-500 hover:text-gray-900 dark:hover:text-white ms-5">
                <GithubSolid size="xl"/>
              </a>
              <a href="https://twitter.com/i/flow/login" target="_blank" rel="noopener noreferrer" class="text-gray-500 hover:text-gray-900 dark:hover:text-white ms-5">
                <TwitterSolid size="xl"/>
              </a>
          </div>
      </div>
    </div>
</footer>


<style>
</style>
