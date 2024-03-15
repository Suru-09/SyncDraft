<script>
    import Logo from "$lib/assets/logo-removebg-preview.png"
    let loggedIn = false;
</script>

<main>
    <h1>SyncDraft</h1>
    <img src={Logo} alt="logo"/>
    {#if loggedIn}
        <h2>Wellcome, </h2>
        <a href="/edit">
            <button>New Draft</button>
        </a>
    {:else}
        <a href="/signup">
            <button>Sign Up</button>
        </a>
        <a href="/login">
            <button>Login</button>
        </a>
    {/if}
</main>


<style>
    main {
        display: block; /* or grid */
        justify-content: center;
        align-items: center;
    }
    img {
        margin-left: 37%;
        width: 410px;
        height: 410px;
    }
    h1 {
        color:cornflowerblue;
        font-family: 'Courier New', Courier, monospace;
        font-size: xx-large;
        margin-left: 45%;
    }
    button {
        margin-top: 1%;
        margin-left: 48%;
    }
</style>