<script>
    import { onMount } from 'svelte';
    import { backendUrl } from '../../config';
    import { Table, TableBody, TableBodyCell, TableBodyRow,
         TableHead, TableHeadCell, Pagination, 
		 Button, Select, Label 
    } from 'flowbite-svelte';
    import axios from 'axios';
    import {TrashBinOutline, EditOutline, ArrowLeftOutline, ArrowRightOutline } from 'flowbite-svelte-icons';
    import { userDocuments, loggedUser } from '../../stores';
    let selected = 10;
    let pageSizes = [
        { value: 10, name: '10' },
        { value: 25, name: '25' },
        { value: 50, name: '50' },
        { value: 100, name: '100' }
    ];

    let pageSize = selected;
    let currentDocsBodySize = 40;
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
                {#each currentDocs as document}
                    <TableBodyRow>
                        <TableBodyCell>{document.doc_name}</TableBodyCell>
                        <TableBodyCell>{document.doc_owner}</TableBodyCell>
                        <TableBodyCell>{document.body}</TableBodyCell>
                        <TableBodyCell>
                            <EditOutline class="w-6 h-5 me-2 text-primary-500 hover:outline dark:text-primary-500" />
                        </TableBodyCell>
                        <TableBodyCell>
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
                    <Button> Create a new document </Button>
            </div>
        </Table>
</main>

<style>  
    .under-table {
        min-width: 100%;
        width: 215%;
        overflow: auto;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
    }
</style>