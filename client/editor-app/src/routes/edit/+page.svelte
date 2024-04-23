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

    let userNames = ['Jese Leos', 'John Doe', 'Jane Smith', 'dada', 'dada', 'dada', 'wtf'];
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

</script>

<main>
    <div class="text-container">
        <form class="w-3/5">
            <label for="editor" class="sr-only">Publish post</label>
            <Textarea id="editor" rows="8" class="mb-4" placeholder="Write something">
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

    {#if isTextareaFocused}
        {#each cursors as cursor}
            <div style="position: relative; top: {cursor.position}px;">
                <span style="position: absolute; left: 0; color: {cursor.color}; font-weight: bold;">{cursor.username}</span>
                <span style="position: absolute; left: 0; width: 2px; background-color: {cursor.color}; height: 100%;"></span>
            </div>
        {/each}
    {/if}
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
</style>