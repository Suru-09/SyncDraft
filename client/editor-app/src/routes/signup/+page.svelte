<script>
    import axios from 'axios';
    import {backendUrl} from '../../config.js';
    import { Input, Label, Helper, Button, Checkbox, A } from 'flowbite-svelte';
	import { loggedUser } from '../../stores.js';

  let currentUser = {
    "username": "",
    "password": "",
    "confirmPassword": "",
    "firstName": "",
    "lastName": ""
  };

  let confirmPWNotMatching = "The confirm password does not match the initial one.";
 
    async function onSignUp() {
      if (passwordsMatching == false || currentUser.firstName == ""
          || currentUser.lastName == "" || currentUser.password == ""
          || currentUser.username == "" || currentUser.username.search("@") == 0
      ) 
      {
        return;
      }

      console.log(`username is ${currentUser.username}`);
      console.log(`pw is ${currentUser.password}`);

      try {
        console.log(currentUser);
        axios.post(`${backendUrl}/user/create`, {
            username: currentUser.username,
            password: currentUser.password,
            first_name: currentUser.firstName,
            last_name: currentUser.lastName
        }).then(response => {
            console.log(response);
            $loggedUser = response.data;
        });
      } catch (e) {
          console.log(`error: ${e}`);
      }
    }

    let passwordsMatching = true;
    const checkConfirmPassword = () => {
      if (currentUser.confirmPassword == currentUser.password) {
        passwordsMatching = true;
      }
      else {
        passwordsMatching = false;
      }
    }

    /**
	 * @param {any} event
    */
    const onPasswordInput = (event) => {
        currentUser.password = event.target.value;
        checkConfirmPassword();
    }
    /**
	 * @param {any} event
    */
    const onConfirmPasswordInput = (event) => {
        currentUser.confirmPassword = event.target.value;
        checkConfirmPassword();
    }
    /**
	 * @param {any} event
    */
    const onUsernameInput = (event) => {
        currentUser.username = event.target.value;
    }
    /**
	 * @param {any} event
    */
    const onFirstNameInput = (event) => {
        currentUser.firstName = event.target.value;
    }
    /**
	 * @param {any} event
    */
    const onLastNameInput = (event) => {
        currentUser.lastName = event.target.value;
    }
</script>

<main>
    <form class="flex flex-col w-1/2 self-center ml-auto mr-auto mt-6 mb-6">
        <div class="grid gap-6 mb-6 md:grid-cols-2">
          <div>
            <Label for="first_name" class="mb-2">First name</Label>
            <Input on:input={onFirstNameInput} type="text" id="first_name" placeholder="John" required />
          </div>
          <div>
            <Label for="last_name" class="mb-2">Last name</Label>
            <Input on:input={onLastNameInput} pe="text" id="last_name" placeholder="Doe" required />
          </div>
        </div>
        <div class="mb-6">
          <Label for="email" class="mb-2">Email address</Label>
          <Input on:input={onUsernameInput} type="email" id="email" placeholder="john.doe@company.com" required />
        </div>
        <div class="mb-6">
          <Label for="password" class="mb-2">Password</Label>
          <Input on:input={onPasswordInput} pe="password" id="password" placeholder="•••••••••" required />
        </div>
        <div class="mb-6">
          <Label for="confirm_password" class="mb-2">Confirm password</Label>
          <Input on:input={onConfirmPasswordInput} 
            color={passwordsMatching == false ? "red" : "base"} type="password" id="confirm_password" placeholder="•••••••••" required
          />
          {#if passwordsMatching == false}
          <Helper class="mt-2" color="red">
              {confirmPWNotMatching}
          </Helper>
          {/if}
        </div>
        <Button on:click={onSignUp} type="submit">Submit</Button>
      </form>
</main>

<style>
</style>