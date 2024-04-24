<script>
    import axios from 'axios';
    import { backendUrl } from '../../config';
    import { Textarea, Toolbar, ToolbarGroup, ToolbarButton, Button, Input, Label } from 'flowbite-svelte';
    import { PaperClipOutline, MapPinAltSolid, ImageOutline, CodeOutline, FaceGrinOutline, PaperPlaneOutline } from 'flowbite-svelte-icons';
    import { Listgroup, ListgroupItem, Avatar } from 'flowbite-svelte';
    import { TrashBinSolid } from 'flowbite-svelte-icons';
    
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
            const res = await axios.post(`${backendUrl}/edit`, {
                dt: event.data,
                val: textareaValue,
                pos: cursorPosition
            });
        } catch (e) {
            console.log(`error: ${e}`);
        }
    }

    let isTextareaFocused = false

    let userNames = ['Suru', 'John Doe', 'Jane Smith', 'dada', 'dada', 'dada', 'wtf'];
    let colors = ['#FF6666', '#FF9933', '#0000CC', '#B2FF66', '#66FFFF', '#66B2FF', '#9933FF', '#FF99FF', '#C0C0C0', '#00994C'];

    let index = 0;
    const getNextColor = () => {
        index = index >= colors.length ? 0 : index;
        return colors[index++];
    }

    let cursors = [
        {"position": 0, "username": "Suru", "color": getNextColor()},
        {"position": 15, "username": "Dodel", "color": getNextColor()},
    ];

    function handleTextareaFocus() {
        isTextareaFocused = true;
    }

    function handleTextareaBlur() {
        isTextareaFocused = false;
    }

    /**
    * @param {any} event
    */
    function getCursor(event) {
        let x = event.clientX;
        let y = event.clientY;
        let _position = `Suru`;

        const editorElement = document.getElementById('editor');
        const infoElement = document.getElementById('info');
        if (infoElement && editorElement) {
            const editorRect = editorElement.getClientRects();
            const rect = editorRect.length == 1 ? editorRect[0] : null;

            if (rect) {
                console.log(pointInRect(x, y, rect));
            }
            
            if (rect && pointInRect(x, y, rect)) {
                infoElement.innerHTML = _position;
                infoElement.style.top = y + "px";
                infoElement.style.left = x + "px";
            }

            if (!isTextareaFocused) {
                infoElement.style.visibility = "hidden";
            }
            else {
                infoElement.style.visibility = "visible";
            }
        }
    }

    /**
    * @param {number} x
    * @param {number} y
    * @param {DOMRect} rect
    */
    const pointInRect = (x, y, rect) => {
        console.log("Sanity check");
        console.log(`x: ${x}, y: ${y} and rect: l: ${rect?.left} r: ${rect?.right} t: ${rect?.top} b: ${rect?.bottom}`);
        if (x > rect.right || x < rect.left) {
            return false;
        }
        
        if ( y > rect.bottom || y < rect.top) {
            return false;
        }

        return true;
    }

</script>

<main>
    <div class="text-container" on:mousemove={getCursor} id="demo" role="button" tabindex="0">
        <form class="w-3/5">
            <label for="editor" class="sr-only">Publish post</label>
            <div id="info"></div>
            <Textarea id="editor" rows="8" class="mb-4" placeholder="Write something" on:mouseover={handleTextareaFocus} on:mouseleave={handleTextareaBlur}>
              <Toolbar slot="header" embedded>
                <ToolbarGroup>
                    <Input type="text" id="doc_name" placeholder="Document name" required />
                </ToolbarGroup>
                <ToolbarGroup>
                    <Label for="website" class="mb-2"> Document Owner: Suru </Label>
                </ToolbarGroup>
              </Toolbar>
            </Textarea>
            <Button>Save document</Button>
        </form>

        <Listgroup active class="w-48">
            <h3 class="p-1 text-center text-xl font-medium text-gray-900 dark:text-white">User list</h3>
            {#each userNames as userName}
                <ListgroupItem class="text-base font-semibold gap-2">
                    <Avatar size="xs" />
                    <span style="color: {getNextColor()}">{userName}</span>
                </ListgroupItem>
            {/each}
        </Listgroup>
    </div>
</main>


<style>
    main {
        width: 100%;
        height: 100%;
    }

    .text-container {
        display: flex;
        justify-content: space-evenly;
        margin-top: 2rem;
        height: 100%;
    }

    #demo {
        height: 100%;
        width: 100%;
    }

    #info {
        padding-left: 10px;
        border-radius: 10px;
        position: absolute;
        user-select: none;
        font-size: 1em;
        text-align: center;
        width: 45px;
        color: #EEEEEE;
        background-color: #FD7013;
    }
</style>