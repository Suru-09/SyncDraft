<script>
    import axios from 'axios';
    
    /**
	 * @type {any}
	 */
    let textareaValue;

    let cursorPosition;

    /**
	 * @param {any} event
	 */
    async function onInputHandler(event) {
        console.log(`data is ${event.data}`);
        console.log(`textarea value is ${textareaValue}`);

        cursorPosition = event.target.selectionStart;
        console.log(`position is ${cursorPosition}`);

        try {
            const res = await axios.post(`http://localhost:8005/`, {
                dt: event.data,
                val: textareaValue,
                pos: cursorPosition
            });
        } catch (e) {
            console.log(`error: ${e}`);
        }
    }
</script>

<main>
    <textarea on:input={onInputHandler} bind:value={textareaValue}/>
</main>


<style>
    main {
        width:100%;
        height:100%;
    }
    textarea {
        width: 400px;
        height: 400px;
        background-color: black;
        color: white;
        top: 50%;
        left: 50%;
        margin-left: 35%;
        margin-top: 5%;
    }
</style>