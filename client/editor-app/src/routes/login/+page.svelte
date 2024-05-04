<script>
    import {backendUrl} from '../../config.js';
    import axios from 'axios';
    import { loggedIn, loggedUser } from '../../stores.js';
	import { goto } from '$app/navigation';
    import { Button, Label, Input, Alert } from 'flowbite-svelte';
    import { EyeOutline, EyeSlashOutline, InfoCircleSolid } from 'flowbite-svelte-icons';
    import { EnvelopeSolid } from 'flowbite-svelte-icons';

    let isVisible = false;
    let wrongPasswordIntroduced = false;


    let usernameValue = "";
    let passwordValue = "";

    async function onLogin() {
        console.log(`username is ${usernameValue}`);
        console.log(`pw is ${passwordValue}`);

        axios.post(`${backendUrl}/user/login`, {
            username: usernameValue,
            password: passwordValue,
        }).then(response => {
            console.log(response);
            if (response.status == 200) {
                $loggedIn = true;
                $loggedUser.firstName = response.data.first_name;
                $loggedUser.lastName = response.data.last_name;
                $loggedUser.password = response.data.password;
                $loggedUser.username = response.data.username;
                
                // nice to see your documents once you have logged in.
                // note: cannot use goto, because it does not refresh
                // the page
                //goto('/documents');
                // reload is used because other pages could depend on
                // local storage values.
                location.assign('/documents');
            }
        }).catch((error) => {
            console.log(error);
            if (error.response.status == 404 || error.response.status == 401) {
                wrongPasswordIntroduced = true;
            }
        });
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
            {#if wrongPasswordIntroduced}
                <Alert color="red">
                    <InfoCircleSolid slot="icon" class="w-4 h-4" />
                    <span class="font-medium">Wrong password!</span>
                    Try to introduce the password again.
                </Alert>
            {/if}
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