<script lang="ts">
    import axios from 'axios';
    import { backendUrl, peerJSServerUrl } from '../../config';
    import { Textarea, Toolbar, ToolbarGroup, Button, Input, Label, ButtonGroup, Alert  } from 'flowbite-svelte';
    import { Listgroup, ListgroupItem, Avatar } from 'flowbite-svelte';
	import { onMount, createEventDispatcher } from 'svelte';
    import { currentEditingDocument, isAnyDocEdited, loggedUser, loggedIn, usersList, isSessionStarted, connectedToSession } from '../../stores';
    import { ChevronSortOutline, ClipboardSolid, InfoCircleSolid } from 'flowbite-svelte-icons';
    import { LogootDocument, generateSiteId, InsertOperation, DeleteOperation } from '$lib/utils/logoot';
    

    export let value: string = '';
    export let currentDocumentName: string = '';
    let logootDocument = new LogootDocument();
    let siteId = generateSiteId();

    let colors = ['#FF6666', '#FF9933', '#0000CC', '#B2FF66', '#66FFFF', '#66B2FF', '#9933FF', '#FF99FF', '#C0C0C0', '#00994C'];

    function handleChange(event: Event) {
        const target = event.target as HTMLTextAreaElement;
        value = target.value;
        $currentEditingDocument.body = value;
        onInputHandler(event);
    }

    function handleDocNameChange(event: Event) {
        const target = event.target as HTMLTextAreaElement;
        currentDocumentName = target.value;
        $currentEditingDocument.doc_name = currentDocumentName;
    }

    function updateTextAreaFromLogoot() {
        let newValue = "";
        logootDocument.lines.forEach((line) => {
            if(line.atom != null) {
                newValue = newValue.concat(line.atom);
            }
        });

        value = newValue;
        $currentEditingDocument.body = newValue;
    }

    const updateUserList = async (users_list: Array<{"username": "", "webrtc_id": ""}>) => {
        let logged_username = $loggedUser.username.split('@')[0];
        if (logged_username)
        {
            $usersList = [logged_username];
        }
        users_list.forEach((user) => {
            let user_name = user.username.split('@')[0];
            if (user_name && !$usersList.find((user) => user === user_name))
            {
                $usersList.push(user_name);
                $usersList = $usersList;
            }
        });
        console.log($usersList);
    };

    const getUsersAndUpdate = async () => {
        await axios.get(`${peerJSServerUrl}/get-users-list`, {
            params: {
                _id: $currentEditingDocument._id
            }
        })
        .then((result) => {
            console.log(result);
            updateUserList(result.data.users_list);
            return result;
        })
        .catch((err) => {
            console.log(err);
        });
    };

    const createWEBRTCSession = async () => {
        if ($isSessionStarted || $connectedToSession)
        {
            return;
        }

        // start the session\
        import('$lib/utils/peer').then((peerModule) => {
            peerModule.PeerConnection.startPeerSession().then(async () => {
                const webrtcID = peerModule.PeerConnection.getPeer()?.id;
                await axios.post(`${peerJSServerUrl}/create-session`, {
                    "_id": $currentEditingDocument._id,
                    "doc_owner": $currentEditingDocument.doc_owner,
                    "webrtc_id": webrtcID
                })
                .then((result) => {
                    console.log(result);
                    $isSessionStarted = true;
                })
                .catch((err) => {
                    console.log(err);
                });

                peerModule.PeerConnection.onIncomingConnection(function(conn) {
                    console.log(`Hello there ${conn.peer}...!!`);
                    peerModule.PeerConnection.onConnectionReceiveData(conn.peer, onReceiveCallback);

                    let newPeerPayload = {
                        type: "initial_payload",
                        current_document_info: $currentEditingDocument,
                        logoot_document: logootDocument
                    };

                    conn.on('open', () => {
                        broadCastNewConnections([conn.peer]);
                        getUsersAndUpdate();
                        console.log("User list updated");
                        try {
                            const serializedPayload = JSON.stringify(newPeerPayload);
                            conn.send(serializedPayload);
                            console.log("Payload sent successfully");
                            console.log(serializedPayload);
                        } catch (error) {
                            console.error("Serialization Error: ", error);
                        }
                    });
                });

                let id = '';
                const peer = peerModule.PeerConnection.getPeer();
                if (peer !== undefined && peer.id !== undefined)
                {
                    id = peer.id;
                }
                peerModule.PeerConnection.onConnectionDisconnected(id, function() {
                    console.log(`Bye mr: ${id}...!!`);
                    getUsersAndUpdate();
                });
                
            });
        });
    }

    const closeWEBRTCSession = async () => {
        // start the session\
        import('$lib/utils/peer').then((peerModule) => {
            peerModule.PeerConnection.closePeerSession().then(async () => {
                console.log("WEBRTC session closed");
            });
        });
        
    }

    let onIncomingConnectionCallback = async (conn: any) => {
        // set the callback for receiving data for this peer
        const peerModule = await import('$lib/utils/peer');
        let peerConnection = peerModule.PeerConnection;
        peerConnection.onConnectionReceiveData(conn.peer, onReceiveCallback);

        conn.on('open', () => {
            try {
                console.log("Incoming connection for not master : ");
                console.log(conn.peer);
            } catch (error) {
                console.error("Serialization Error: ", error);
        }
    });
    };

    let onReceiveCallback = async (data: string) => {
        let received = JSON.parse(data);
        console.log("Received on callback: ");
        console.log(received);

        if (received["type"] != undefined && received["type"] === "initial_payload") {
            // update document info
            let docOwner = received["current_document_info"]["doc_owner"];
            let docName = received["current_document_info"]["doc_name"];
            $currentEditingDocument.doc_owner = docOwner;
            $currentEditingDocument.body = received["current_document_info"]["body"];
            currentDocumentName = docName;
            // update logoot document
            logootDocument.fromJSON(data);
            // update actual text
            value = received["current_document_info"]["body"];
        } else if (received["type"] === "insert") {
            let insertOp = InsertOperation.fromJSON(data);
            logootDocument.insert(insertOp.posId, insertOp.atom);
            updateTextAreaFromLogoot();
        } else if (received["type"] === "delete") {
            let deleteOp = DeleteOperation.fromJSON(data);
            logootDocument.delete(deleteOp.posId);
            updateTextAreaFromLogoot()
        } else if (received["type"] === "newConnections") {
            const peers: Array<string> = received["peers"];
            console.log(`Broad cast received with peers: ${peers}`);
            await connectToNewUsers(peers);
        }
    }

    onMount(async () => {
        value = $isAnyDocEdited ? $currentEditingDocument.body : '';
        currentDocumentName = $isAnyDocEdited ? $currentEditingDocument.doc_name : '';
        console.log($usersList);

        let idx = 1;
        [...$currentEditingDocument.body].forEach(character => {
            logootDocument.insertAtIndex(siteId, character, idx);
            idx++;
        });

        // make sure when client leaves we delete the doc he was last editing.
        if (window && window !== undefined)
        {
            window.addEventListener('beforeunload', function (event) {
                
            });
        }
    });

    const broadcastData = async (data: string) => {
        import('$lib/utils/peer').then(async (peerModule) => {
            let map = peerModule.getConnectionMap();

            map.forEach((_, peerId) => {
                peerModule.PeerConnection.sendConnection(peerId, data);
            }); 
        }
    )};

    export const broadCastNewConnections = async (users_list: Array<string>) => {
        import('$lib/utils/peer').then(async (peerModule) => {
            let map = peerModule.getConnectionMap();
            let json_stringy = JSON.stringify({
                type: new String('newConnections'),
                peers: users_list
            });

            console.log("Connection map: ");
            console.log(map);

            map.forEach(async (_, peerId) => {
                await peerModule.PeerConnection.sendConnection(peerId, json_stringy);
            }); 
        })
    }

    const connectToNewUsers = async (peerConnections: Array<string>) => {
        console.log(`Connecting to new users: ${peerConnections}`);
        import('$lib/utils/peer').then(async (peerModule) => {
                peerConnections.forEach(async (peerID) => {
                    let currentPeerID = '';
                    const peer = peerModule.PeerConnection.getPeer();
                    if (peer !== undefined)
                    {
                        currentPeerID = peer.id;
                    }
                    //peerModule.PeerConnection.onIncomingConnection(onIncomingConnectionCallback);
                    if (!peerModule.getConnectionMap().get(peerID) &&  peerID !== currentPeerID)
                    {
                        peerModule.PeerConnection.connectPeer(peerID).then(async () => {
                            console.log(`Connecting to user: IDK with WEBRTC_ID: ${peerID}`);
                            peerModule.PeerConnection.onConnectionReceiveData(peerID, onReceiveCallback);
                        });
                    }
                });

                // update users_list
                let logged_username = $loggedUser.username.split('@')[0];
                if (logged_username)
                {
                    $usersList = [logged_username];
                }
                else {
                    $usersList = [];
                }

                await axios.get(`${peerJSServerUrl}/get-users-list`, {
                params: {
                    _id: userInputSessionID
                }
                }).then((result)=> {
                    let users_list: Array<{"username": "", "webrtc_id": ""}> = result.data.users_list;
                    users_list.forEach((user) => {
                        let user_name = user.username.split('@')[0];
                        if (user_name && !$usersList.find((user) => user === user_name))
                        {
                            $usersList.push(user_name);
                            $usersList = $usersList;
                        }
                        console.log($usersList);                   
                    });
                });
        });
    };

    const connectToWebRTCUserList = async (users_list: Array<{"username": "", "webrtc_id": ""}>) => {
        // start peer session first
        return import('$lib/utils/peer').then(async (peerModule) => {
            return peerModule.PeerConnection.startPeerSession().then(async (id) => {
                let logged_username = $loggedUser.username.split('@')[0];
                if (logged_username)
                {
                    $usersList = [logged_username];
                }
                else {
                    $usersList = [];
                }
                users_list.forEach(async (user) => {
                    let user_name = user.username.split('@')[0];
                    if (user_name && !$usersList.find((user) => user === user_name))
                    {
                        $usersList.push(user_name);
                        $usersList = $usersList;
                    }
                    console.log($usersList);
                    
                    let currentPeerID = '';
                    const peer = peerModule.PeerConnection.getPeer();
                    if (peer !== undefined)
                    {
                        currentPeerID = peer.id;
                    }
                    peerModule.PeerConnection.onIncomingConnection(onIncomingConnectionCallback);

                    if(!peerModule.getConnectionMap().get(user.webrtc_id) && currentPeerID !== user.webrtc_id)
                    {
                        await peerModule.PeerConnection.connectPeer(user.webrtc_id).then(async () => {
                            $currentEditingDocument._id = userInputSessionID;
                            console.log(`Connecting to user: ${user.username} with WEBRTC_ID: ${user.webrtc_id}`);
                            peerModule.PeerConnection.onConnectionReceiveData(user.webrtc_id, onReceiveCallback);
                        });
                    }
                    
                });

                await axios.post(`${peerJSServerUrl}/append-user-to-session`, {
                    _id: userInputSessionID,
                    doc_owner: $loggedUser.username,
                    webrtc_id: id
                });

                return id;
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
            const peerID: string = await connectToWebRTCUserList(result.data.users_list);
            // let the session owner know about the new connection.
            $connectedToSession = true;
        })
        .catch((err) => {
            console.log(err);
        });
        
    };

    let cursorPosition;

    async function onInputHandler(event: any) {
        let char = event.data;
        console.log(`event is`);
        console.log(event)
        console.log(`data is ${char}`);
        console.log(`textarea value is ${value}`);
        console.log(`user session id is ${userInputSessionID}`)

        cursorPosition = event.target.selectionStart;
        console.log(`position is ${cursorPosition}`);
        if (char != null) {
            let insertOperation = logootDocument.insertAtIndex(siteId, char, cursorPosition);
            console.log(insertOperation);
            console.log(insertOperation.getJson())
            await broadcastData(insertOperation.getJson());
        } else if (event.inputType === 'deleteContentBackward') {
            let deleteOperation = logootDocument.deleteAtIndex(siteId, cursorPosition);
            console.log(deleteOperation);
            console.log(deleteOperation.getJson())
            await broadcastData(deleteOperation.getJson());
        }

        console.log(logootDocument);

        // try {
        //     const res = await axios.post(`${backendUrl}/edit`, {
        //         dt: event.data,
        //         val: textareaValue,
        //         pos: cursorPosition
        //     });
        // } catch (e) {
        //     console.log(`error: ${e}`);
        // }
    }

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

    const copyToClipboard = async ()=>
    {
        let id = ($currentEditingDocument._id !== "" ? $currentEditingDocument._id : 'Not a session');

        await navigator.clipboard.writeText(id)

    }
</script>

<main>
    <div class="text-container"id="demo" role="button" tabindex="0">
        <div class="w-2/4">
            <div class="pt-8" style="margin-bottom: 1rem;">
                <Label for="input-addon" class="mb-2">Connect to a shared session</Label>
                <ButtonGroup class="w-1/3">
                <Input id="input-addon" type="text"
                    placeholder="session id" bind:value={userInputSessionID}
                />
                <Button color="primary"
                    on:click={() => connectToSession()}

                >Connect</Button>
                </ButtonGroup>
                <Button on:click={createWEBRTCSession}>
                    Start session
                </Button>
                {#if $isSessionStarted}
                    <Alert dismissable>
                        <InfoCircleSolid slot="icon" class="w-5 h-5" />
                        You have started a new session.
                    </Alert>
		        {/if}
                
            </div>

            <form class="w-full">
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
                            {$currentEditingDocument._id !== "" && ($isSessionStarted || $connectedToSession) ? $currentEditingDocument._id : 'Not a session'}
                        </Label>
                        <button on:click={copyToClipboard}>
                            <ClipboardSolid size="md" style="margin-left: 5px; margin-bottom: 10px;"/>
                        </button>
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
        </div>

        <Listgroup active class="w-64">
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