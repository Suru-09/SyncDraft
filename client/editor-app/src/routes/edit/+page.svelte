<script lang="ts">
    import axios from 'axios';
    import { backendUrl } from '../../config';
    import { Textarea, Toolbar, ToolbarGroup, Button, Input, Label } from 'flowbite-svelte';
    import { Listgroup, ListgroupItem, Avatar } from 'flowbite-svelte';
	import { onMount, createEventDispatcher } from 'svelte';
    import { currentEditingDocument, isAnyDocEdited } from '../../stores';

    export let value: string = '';
    export let currentDocumentName: string = '';

    function handleChange(event: Event) {
        const target = event.target as HTMLTextAreaElement;
        value = target.value;
        $currentEditingDocument.body = value;
    }

    function handleDocNameChange(event: Event) {
        const target = event.target as HTMLTextAreaElement;
        currentDocumentName = target.value;
        $currentEditingDocument.doc_name = currentDocumentName;
    }
  

    onMount(() => {
        value = $isAnyDocEdited ? $currentEditingDocument.body : '';
        currentDocumentName = $isAnyDocEdited ? $currentEditingDocument.doc_name : '';

        // make sure when client leaves we delete the doc he was last editing.
        if (window && window !== undefined)
        {
            window.addEventListener('beforeunload', function (event) {
                $isAnyDocEdited = false;
                $currentEditingDocument = {
                    "_id": "",
                    "body": "",
                    "doc_owner": "",
                    "doc_name": "",
                };
            });
        }
    });
    
    // /**
	//  * @type {any}
	//  */
    // let textareaValue;

    // let cursorPosition;

    // /**
	//  * @param {any} event
    // */
    // async function onInputHandler(event) {
    //     console.log(`data is ${event.data}`);
    //     console.log(`textarea value is ${textareaValue}`);

    //     cursorPosition = event.target.selectionStart;
    //     console.log(`position is ${cursorPosition}`);

    //     try {
    //         const res = await axios.post(`${backendUrl}/edit`, {
    //             dt: event.data,
    //             val: textareaValue,
    //             pos: cursorPosition
    //         });
    //     } catch (e) {
    //         console.log(`error: ${e}`);
    //     }
    // }

    let isTextareaFocused = false;

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
    function getCursor(event: any) {
        let x = event.clientX;
        let y = event.clientY;
        let _position = `Suru`;

        let index_x = event.target.selectionEnd;

        const lineHeight = parseFloat(getComputedStyle(event.target).lineHeight);
        const linesBeforeCursor = event.target.value.substr(0, index_x).split('\n').length - 1;
        const lastLine = event.target.value.substr(0, index_x).split('\n')[linesBeforeCursor];

        const font = getComputedStyle(event.target).font;
        const canvas = document.createElement('canvas');
        const context = canvas.getContext('2d');
        if (context)
        {
            context.font = font;
            index_x = context.measureText(lastLine).width;
        }


        let index_y = linesBeforeCursor * lineHeight;


        const editorElement = document.getElementById('editor');
        const infoElement = document.getElementById('info');
        if (infoElement && editorElement) {
            const editorRect = editorElement.getClientRects();
            const rect = editorRect.length == 1 ? editorRect[0] : null;

            
            if (rect) {
                console.log(pointInRect(x, y, rect));

            }
            else{
                console.log("Editor rectangle is null");
            }
            
            let padding_left = infoElement.clientWidth * 3/4;
            let padding_up = infoElement.clientHeight / 2;

            if (rect && pointInRect(x, y, rect)) { // set label new position
                infoElement.innerHTML = _position;
                infoElement.style.top = rect.top + index_y + padding_up + "px";
                infoElement.style.marginLeft = index_x + padding_left  + "px";
            }

            if (rect && ((rect.top + index_y + padding_up) >= rect.bottom)) // make label stop at end of textarea
            {
                infoElement.style.top = rect.bottom - lineHeight + "px";
            }

            if ( event.key === 'Enter' || event.key === '\r') // make label stop at end of textarea
            {
                infoElement.style.marginLeft = padding_left + "px";
                getLine(event);
            }
            if ( event.keyCode === 8)
            {
                getLine(event);
            }

            if (!isTextareaFocused) { // toggle label visibility when textarea is in focuse
                infoElement.style.visibility = "hidden";
            }
            else {
                infoElement.style.visibility = "visible";
            }
        }

        getColumn(event);
    }

    const pointInRect = (x: any, y: any, rect: any) => {
        //console.log("Sanity check");
        //console.log(`x: ${x}, y: ${y} and rect: l: ${rect?.left} r: ${rect?.right} t: ${rect?.top} b: ${rect?.bottom}`);
        if (x > rect.right || x < rect.left) {
            return false;
        }
        
        if ( y > rect.bottom || y < rect.top) {
            return false;
        }

        return true;
    }


    let line = 0 ;
    function getLine(event: any)
    {
        let textarea = event.target;
        line = textarea.value.substr(0, textarea.selectStart).split('\n').length;
    }

    let column = 0 ;
    /**
    * @param {any} event
    */
    function getColumn(event: any)
    {
        let textarea = event.target;
        const linesBeforeCursor = textarea.value.substr(0, textarea.selectionStart).split('\n').length - 1;
        const lastLine = textarea.value.substr(0, textarea.selectionStart).split('\n')[linesBeforeCursor];
        column = lastLine.length + 1;

    }

    const saveDocument = async () => {
        if ($currentEditingDocument._id !== "")
        {
            console.log(`Updating an already existing doc...`);
            console.log($currentEditingDocument);
            await axios.post(`${backendUrl}/doc/update`, $currentEditingDocument)
                .then((result) => {
                    console.log(result);
                })
                .catch((err) => {
                    console.log(err);
                });
        }
        else
        {
            console.log(`Creating a new document...`);
            console.log($currentEditingDocument);
            await axios.post(`${backendUrl}/doc/create`, $currentEditingDocument)
                .then((result) => {
                    console.log(result);
                    $currentEditingDocument._id = result.data._id;
                })
                .catch((err) => {
                    console.log(err);
                });
        }
    };

</script>

<main>
    <div class="text-container"id="demo" role="button" tabindex="0">
        <form class="w-3/5">
            <label for="editor" class="sr-only">Publish post</label>
            <div id="info"></div>
            <Textarea id="editor" rows="8" class="mb-4" placeholder="Write something" style="font-size: 16px"
                on:mouseover={handleTextareaFocus} on:mouseleave={handleTextareaBlur}  on:keypress={getCursor} on:click={getCursor}
                bind:value={value} on:input={handleChange}
                >
              <Toolbar slot="header" embedded>
                <ToolbarGroup>
                    <Input type="text" id="doc_name" placeholder="Document name" 
                        bind:value={currentDocumentName} on:input={handleDocNameChange} required 
                    />
                </ToolbarGroup>
                <ToolbarGroup>
                    <Label for="website" class="mb-2"> Document Owner: Suru </Label>
                </ToolbarGroup>
              </Toolbar>
            </Textarea>
            <div>
                <p class="numerics">Line: {line} | Column: {column}</p>
                <Button on:click={() => saveDocument()}
                    >
                    Save document
                </Button>
            </div>
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

    .numerics {
        color: #EEEEEE;
        font-size: 1em;
        position: relative;
        float: right;
    }
</style>