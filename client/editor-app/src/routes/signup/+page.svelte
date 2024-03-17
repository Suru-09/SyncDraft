<script>
    let isVisible = false;
    
    $: type = isVisible ? "text" : "password";

    const toggleVisibility = () => {
        isVisible = !isVisible;
    };
</script>

<main>
    <h1>SyncDraft</h1>
    <div>
        <h2>Username:</h2>
        <input>
    </div>
    <div>
        <h2>Password:</h2>
        <input {type}>
    </div>
    <div class="show_pass_div">
        <input on:change={toggleVisibility} type=checkbox name="toggle" id="toggle">
        <label for="toggle">Show password</label>    
    </div>
    <button>Create account</button>
</main>

<style>
    main {
        display: block; /* or grid */
        justify-content: center;
        align-items: center;
    }
    h1 {
        color:cornflowerblue;
        font-family: 'Courier New', Courier, monospace;
        font-size: xx-large;
        margin-left: 45%;
    }
    div {
        margin-left: 45%;
    }
    .show_pass_div {
        margin-top: 1%;
    }
    label {
        color:cornflowerblue;
        font-family: 'Courier New', Courier, monospace;
        font-size: large; 
    }
    button {
        margin-top: 2%;
        margin-left: 47%;
    }
    h2 {
        color:cornflowerblue;
        font-family: 'Courier New', Courier, monospace;
    }
    button {
        margin-top: 1%;
        margin-left: 45%;
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
        text-align: left;
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