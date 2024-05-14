<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { backendUrl, peerJSServerUrl } from '../../config';
    import { Table, TableBody, TableBodyCell, TableBodyRow,
         TableHead, TableHeadCell, Pagination, 
		 Button, Select, Label, Modal
    } from 'flowbite-svelte';
    import axios from 'axios';
    import {TrashBinOutline, EditOutline, ArrowLeftOutline, ArrowRightOutline } from 'flowbite-svelte-icons';
    import { userDocuments, loggedUser, isAnyDocEdited, currentEditingDocument, usersList } from '../../stores';

    let selected = 10;
    let pageSizes = [
        { value: 10, name: '10' },
        { value: 25, name: '25' },
        { value: 50, name: '50' },
        { value: 100, name: '100' }
    ];

    let pageSize = selected;
    let currentDocsBodySize = 40;   // number of characters in body shown to the user..
    let helper = { start: 1, end: pageSize, total: 100 };
    const previous = () => {
        if (helper.start - pageSize >= 1)
        {
            helper.start -= pageSize;
            helper.end -= pageSize;
        }
        currentDocs = $userDocuments.slice(helper.start - 1, helper.end);
     };

    const next = () => {
        if (helper.start + pageSize <= helper.total)
        {
            helper.end += pageSize;
            helper.start += pageSize;
        }
        currentDocs = $userDocuments.slice(helper.start - 1, helper.end);
    }

    const trimDocsBody = () => {
        currentDocs.forEach((doc) => {
            doc.body = doc.body.slice(0, currentDocsBodySize);
        });
    }

    let currentDocs = $userDocuments;

    const handleSelectPageSize = async () => {
        helper.end = Math.min(selected, helper.total);;
        pageSize = selected;
        currentDocs = $userDocuments.slice(helper.start - 1, helper.end);
    }

    async function fetchDocuments() {
        try {
            const params = {
                doc_owner: $loggedUser.username,
            };
            const response = await axios.get(`${backendUrl}/doc/get`, {params});
            $userDocuments = response.data;
            currentDocs = $userDocuments.slice(helper.start - 1, helper.end);
            trimDocsBody();
            helper.total = $userDocuments.length;
            helper.end = Math.min(helper.total, helper.end);
        } catch (error) {
            console.error('Error fetching documents:', error);
        }
    }

    const updateUserList = async (users_list: Array<{"username": "", "webrtc_id": ""}>) => {
        $usersList = [$loggedUser.firstName];
        users_list.forEach((user) => {
            let user_name = user.username.split('@')[0];
            if (user_name)
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
                })
                .catch((err) => {
                    console.log(err);
                });

                peerModule.PeerConnection.onIncomingConnection(function(conn) {
                    console.log(`Hello there ${conn.peer}...!!`);
                    getUsersAndUpdate();
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

    async function editDocument(index: number) {
        // set current doc
        $isAnyDocEdited = true;
        $currentEditingDocument = $userDocuments[index];
        // close any existing webrtc session!!!
        // e.g. you join session, and you want to create yourself one,
        // then you should delete the connection to the one hosted by
        // somebody else
        await closeWEBRTCSession();
        // only after completing the currentEditingDocument!!!!
        await createWEBRTCSession();
        
        goto('/edit');
    }

    async function createNewDocument() {
        // close existing connections,
        // because there is only one access
        // to the editor at the time
        await closeWEBRTCSession();
        // set new doc
        $isAnyDocEdited = true;
        $currentEditingDocument = {
            "_id": "",
            "body": "",
            "doc_owner": $loggedUser.username,
            "doc_name": "",
        };
        goto('/edit');
    }

    async function deleteDocument(index: number) {
        showModal = true;
        doc_index = index;
    }

    let showModal = false;
    let doc_index: number = -1;
    async function confirmDelete() {
        await axios.post(`${backendUrl}/doc/delete`, {
            _id: $userDocuments[doc_index]._id,
        })
            .then((result) => {
                location.assign('/documents');
                console.log(result);
            })
            .catch((err) => {
                console.log(err);
            });
        
        showModal = false;
        doc_index = -1;
    }

    async function cancelDelete() {
        showModal = false;
    }

    // Call the fetch function when the component mounts
    onMount(() => {
        $userDocuments = [];
        fetchDocuments();
    });
</script>

<main class="bg-white dark:bg-gray-800">
        <Table divClass="w-3/4 flex flex-col m-auto" striped={true}>
            <TableHead>
                <TableHeadCell>Document Name</TableHeadCell>
                <TableHeadCell>Document Owner</TableHeadCell>
                <TableHeadCell>Body</TableHeadCell>
                <TableHeadCell>Edit</TableHeadCell>
                <TableHeadCell>Delete</TableHeadCell>
            </TableHead>
            <TableBody tableBodyClass="divide-y">
                {#each currentDocs as document, index}
                    <TableBodyRow>
                        <TableBodyCell>{document.doc_name}</TableBodyCell>
                        <TableBodyCell>{document.doc_owner}</TableBodyCell>
                        <TableBodyCell>{document.body}</TableBodyCell>
                        <TableBodyCell on:click={() => editDocument(index)}>
                            <EditOutline class="w-6 h-5 me-2 text-primary-500 hover:outline dark:text-primary-500" />
                        </TableBodyCell>
                        <TableBodyCell on:click={() => deleteDocument(index)}>
                            <TrashBinOutline class="w-6 h-5 me-2 text-primary-500 hover:outline dark:text-primary-500" />
                        </TableBodyCell>
                    </TableBodyRow>
                {/each}
            </TableBody>
            <div class="under-table">
                    <div class="flex flex-col items-center justify-center gap-2 mt-5">
                        <div class="text-sm text-gray-700 dark:text-gray-400">
                            Showing <span class="font-semibold text-gray-900 dark:text-white">{helper.start}</span>
                            to
                            <span class="font-semibold text-gray-900 dark:text-white">{helper.end}</span>
                            docs of
                            <span class="font-semibold text-gray-900 dark:text-white">{helper.total}</span>
                            total docs
                        </div>
                    
                        <Pagination large>
                            <button type="button" slot="prev" on:click={previous} class="flex items-center gap-2 dark:text-white text-gray-900 bg-white dark:bg-gray-800">
                                <ArrowLeftOutline class="w-3.5 h-3.5 me-2"/>
                                Prev
                            </button>
                            <button type="button" slot="next" on:click={next} class="flex items-center gap-2 dark:text-white text-gray-900 bg-white dark:bg-gray-800">
                                Next
                                <ArrowRightOutline class="w-6 h-6 me-2"/>
                            </button>
                        </Pagination>
                    </div>
                    <Label>
                        Select page size
                        <Select class="mt-2" items={pageSizes} bind:value={selected} on:change={handleSelectPageSize}/>
                    </Label>
                    <Button on:click={() => createNewDocument()}> Create a new document </Button>
            </div>
        </Table>

        <Modal bind:open={showModal}>
            <div slot="header" class="text-lg leading-6 font-medium text-gray-900">
              Delete Document
            </div>
            <div class="text-sm text-gray-500">
              Are you sure you want to delete the document?
            </div>
            <div slot="footer" class="flex justify-end">
              <button class="px-4 py-2 bg-red-600 text-white rounded-md mr-2" on:click={confirmDelete}>Delete</button>
              <button class="px-4 py-2 bg-gray-200 text-gray-700 rounded-md" on:click={cancelDelete}>Cancel</button>
            </div>
          </Modal>
</main>

<style>  
    .under-table {
        min-width: 100%;
        width: 155%;
        overflow: auto;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
    }
</style>