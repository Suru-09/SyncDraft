<script>
    import Logo from "$lib/assets/logo-removebg-preview.png"
    let loggedIn = false;
</script>

<main>
    <h1>SyncDraft</h1>
    <img src={Logo} alt="logo" />
    {#if loggedIn}
        <h2>Welcome, </h2>
        <a href="/edit">
            <button>New Draft</button>
        </a>
    {:else}
        <div class="signAndLogin">
            <a href="/signup">
                <button>Sign Up</button>
            </a>
            <a href="/login">
                <button>Login</button>
            </a>
        </div> 
    {/if}
</main>


<style>
    main {
        display: block; /* or grid */
        justify-content: center;
        align-items: center;
    }
    img {
        display: block;
        margin-left: auto;
        margin-right: auto;
        max-width: 550px;
        max-height: 550px;
        width: 50%;
    }
    h1 {
        color:cornflowerblue;
        font-family: 'Courier New', Courier, monospace;
        font-size: xx-large;
        text-align: center;
    }
    h2 {
        color:cornflowerblue;
        font-family: 'Courier New', Courier, monospace;
        font-size: xx-large;
        display: block;
        margin-left: auto;
        margin-right: auto;
        width: 50%;
    }

    .signAndLogin {
        display: flex;
        flex-direction: row;
        align-items: center;
        max-width: fit-content;
        margin-left: auto;
        margin-right: auto;
    }

    button {
        width: 100px;
        margin:0 20px;
        align-items: center;
        appearance: none;
        background-image: radial-gradient(100% 100% at 100% 0, #5adaff 0, #5468ff 100%);
        border: 0;
        border-radius: 6px;
        box-shadow: rgba(45, 35, 66, .4) 0 2px 4px,rgba(45, 35, 66, .3) 0 7px 13px -3px,rgba(58, 65, 111, .5) 0 -3px 0 inset;
        box-sizing: border-box;
        color: #fff;
        cursor: pointer;
        display: inline-flex;
        font-family: "JetBrains Mono",monospace;
        height: 48px;
        justify-content: center;
        line-height: 1;
        list-style: none;
        overflow: hidden;
        padding-left: 16px;
        padding-right: 16px;
        position: relative;
        text-decoration: none;
        transition: box-shadow .15s,transform .15s;
        user-select: none;
        -webkit-user-select: none;
        touch-action: manipulation;
        white-space: nowrap;
        will-change: box-shadow,transform;
        font-size: 18px;
    }
    button:focus {
        box-shadow: #3c4fe0 0 0 0 1.5px inset, rgba(45, 35, 66, .4) 0 2px 4px, rgba(45, 35, 66, .3) 0 7px 13px -3px, #3c4fe0 0 -3px 0 inset;
    }
    button:hover {
        box-shadow: rgba(45, 35, 66, .4) 0 4px 8px, rgba(45, 35, 66, .3) 0 7px 13px -3px, #3c4fe0 0 -3px 0 inset;
        transform: translateY(-2px);
    }
    button:active {
        box-shadow: #3c4fe0 0 3px 7px inset;
        transform: translateY(2px);
    }
</style>