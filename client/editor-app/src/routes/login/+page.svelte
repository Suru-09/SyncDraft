<script>
    import {backendUrl} from '../../config.js';
    import axios from 'axios';
    import { loggedIn } from '../../stores.js';
	import { goto } from '$app/navigation';
    import { Button, Label, Input, ButtonGroup, InputAddon, ToolbarButton } from 'flowbite-svelte';
    import { EyeOutline, EyeSlashOutline } from 'flowbite-svelte-icons';
    import { EnvelopeSolid } from 'flowbite-svelte-icons';

    let isVisible = false;
    let usernameValue = "";
    let passwordValue = "";

    async function onLogin() {
        console.log(`username is ${usernameValue}`);
        console.log(`pw is ${passwordValue}`);

        try {
            axios.post(`${backendUrl}/user/login`, {
                username: usernameValue,
                password: passwordValue,
            }).then(response => {
                console.log(response);
                if (response) {
                    $loggedIn = true;
                    goto('/documents');
                }
            });
        } catch (e) {
            console.log(`error: ${e}`);
        }
    }

    /**
	 * @param {any} event
    */
    const handleUserEmail = (event) => {
        usernameValue = event.target.value;
    }

    /**
	 * @param {any} event
    */
    async function onPwInput(event) {
        passwordValue = event.target.value;
    }

</script>

<main>
    <div class="login-container">
    <Label class="mb-2">Email</Label>
    <Input type="email" placeholder="username@something.com" on:input={handleUserEmail}>
        <EnvelopeSolid slot="left" class="w-4 h-4" />
    </Input>
    <div>
        <Label for="show-password" class="mb-2">Your password</Label>
        <Input on:input={onPwInput} id="show-password" 
                type={isVisible ? 'text' : 'password'} placeholder="Your password here"
                class="mb-2"
            >
            <button slot="left" on:click={() => (isVisible = !isVisible)}
                 class="pointer-events-auto"
            >
            {#if isVisible}
                <EyeOutline class="w-4 h-4" />
            {:else}
                <EyeSlashOutline class="w-4 h-4" />
            {/if}
            </button>
        </Input>
    </div>

    <Button type="submit" class="w-1/2 align-center m-auto" on:click={onLogin}>Submit</Button>
    </div>
</main>

<style>
    .login-container {
        display: flex;
        flex-direction: column;
        width: 30%;
        align-self: center;
        justify-content: center;
        margin-right: auto;
        margin-left: auto;
        margin-top: 3em;
    }
</style>