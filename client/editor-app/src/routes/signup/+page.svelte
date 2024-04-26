<script>
    import axios from 'axios';
    import {backendUrl} from '../../config.js';
    import { Input, Label, Helper, Button, Checkbox, A } from 'flowbite-svelte';

    let isVisible = false;
    let usernameValue = "";
    let passwordValue = "";
    
    $: type = isVisible ? "text" : "password";

    const toggleVisibility = () => {
        isVisible = !isVisible;
    };

    async function onSignUp() {
        console.log(`username is ${usernameValue}`);
        console.log(`pw is ${passwordValue}`);

        try {
            axios.post(`${backendUrl}/user/create`, {
                username: usernameValue,
                password: passwordValue,
            }).then(response => {
                console.log(response);
            });
        } catch (e) {
            console.log(`error: ${e}`);
        }
    }

    /**
	 * @param {any} event
    */
    const onPasswordInput = (event) => {
        passwordValue = event.target.value;
    }
</script>

<main>
    <form class="flex flex-col w-1/2 self-center ml-auto mr-auto mt-6 mb-6">
        <div class="grid gap-6 mb-6 md:grid-cols-2">
          <div>
            <Label for="first_name" class="mb-2">First name</Label>
            <Input type="text" id="first_name" placeholder="John" required />
          </div>
          <div>
            <Label for="last_name" class="mb-2">Last name</Label>
            <Input type="text" id="last_name" placeholder="Doe" required />
          </div>
        </div>
        <div class="mb-6">
          <Label for="email" class="mb-2">Email address</Label>
          <Input type="email" id="email" placeholder="john.doe@company.com" required />
        </div>
        <div class="mb-6">
          <Label for="password" class="mb-2">Password</Label>
          <Input type="password" id="password" placeholder="•••••••••" required />
        </div>
        <div class="mb-6">
          <Label for="confirm_password" class="mb-2">Confirm password</Label>
          <Input type="password" id="confirm_password" placeholder="•••••••••" required />
        </div>
        <Checkbox class="mb-6 space-x-1 rtl:space-x-reverse" required>
          I agree with the <A href="/" class="text-primary-700 dark:text-primary-600 hover:underline">terms and conditions</A>.
        </Checkbox>
        <Button type="submit">Submit</Button>
      </form>
</main>

<style>
</style>