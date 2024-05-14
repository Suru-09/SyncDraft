<script lang="ts">
    import axios from 'axios';
    import { backendUrl, peerJSServerUrl } from '../../config';
    import { Textarea, Toolbar, ToolbarGroup, Button, Input, Label, ButtonGroup } from 'flowbite-svelte';
    import { Listgroup, ListgroupItem, Avatar } from 'flowbite-svelte';
	import { onMount, createEventDispatcher } from 'svelte';
    import { currentEditingDocument, isAnyDocEdited, loggedUser, loggedIn, usersList } from '../../stores';

    export let value: string = '';
    export let currentDocumentName: string = '';

    let colors = ['#FF6666', '#FF9933', '#0000CC', '#B2FF66', '#66FFFF', '#66B2FF', '#9933FF', '#FF99FF', '#C0C0C0', '#00994C'];

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
        console.log($usersList);

        // make sure when client leaves we delete the doc he was last editing.
        if (window && window !== undefined)
        {
            window.addEventListener('beforeunload', function (event) {
                
            });
        }
    });

    const connectToWebRTCUserList = async (users_list: Array<{"username": "", "webrtc_id": ""}>) => {
        // start peer session first
        import('$lib/utils/peer').then(async (peerModule) => {
            peerModule.PeerConnection.startPeerSession().then(async (id) => {
                $usersList = [$loggedUser.firstName];
                users_list.forEach((user) => {
                    $usersList.push(user.username);
                    $usersList = $usersList;
                    console.log($usersList);

                    peerModule.PeerConnection.connectPeer(user.webrtc_id).then(() => {
                            $currentEditingDocument._id = userInputSessionID;
                            console.log(`Connecting to user: ${user.username} with WEBRTC_ID: ${user.webrtc_id}`);
                    }); 
                });

                await axios.post(`${peerJSServerUrl}/append-user-to-session`, {
                    _id: userInputSessionID,
                    doc_owner: $loggedUser.username,
                    webrtc_id: id
                });
            });
        });
    };

    let userInputSessionID = '';
    const connectToSession = async () => {
        await axios.get(`${peerJSServerUrl}/get-users-list`, {
            params: {
                _id: userInputSessionID
            }
        })
        .then(async (result) => {
            console.log(result);
            connectToWebRTCUserList(result.data.users_list);
        })
        .catch((err) => {
            console.log(err);
        });
    };
    

    let isTextareaFocused = false;

    let index = 0;
    const getNextColor = () => {
        index = index >= colors.length ? 0 : index;
        return colors[index++];
    }

    let cursors = [
        {"position": 0, "username": $loggedUser.firstName, "color": getNextColor()},
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
        let _position = $loggedUser.firstName;

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

            
            // if (rect) {
            //     console.log(pointInRect(x, y, rect));

            // }
            // else{
            //     console.log("Editor rectangle is null");
            // }
            
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
        <div class="pt-8">
            <Label for="input-addon" class="mb-2">Connect to a shared session</Label>
            <ButtonGroup class="w-full">
              <Input id="input-addon" type="text" 
                placeholder="session id" bind:value={userInputSessionID}
              />
              <Button color="primary"
                on:click={() => connectToSession()}
                
              >Connect</Button>
            </ButtonGroup>
          </div>

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
                    <Label for="website" class="mb-2"> Document Owner: {$currentEditingDocument.doc_owner} </Label>
                </ToolbarGroup>

                <ToolbarGroup>
                    <Label for="website" class="mb-2"> Session id: 
                        {$currentEditingDocument._id !== "" ? $currentEditingDocument._id : 'Not a session'} 
                    </Label>
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
            {#each $usersList as userName}
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
        padding-left: 5px;
        padding-right: 5px;
        border-radius: 10px;
        position: absolute;
        user-select: none;
        font-size: 1em;
        text-align: center;
        width: 75px;
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