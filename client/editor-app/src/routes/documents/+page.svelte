<script>
    import { onMount } from 'svelte';
    import { backendUrl } from '../../config';
    import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from 'flowbite-svelte';
    import axios from 'axios';
    import {TrashBinOutline, EditOutline } from 'flowbite-svelte-icons';
    import { Pagination } from 'flowbite-svelte';
    import { ArrowLeftOutline, ArrowRightOutline } from 'flowbite-svelte-icons';
	import { inputClass, labelClass } from 'flowbite-svelte/Radio.svelte';

    let helper = { start: 1, end: 10, total: 100 };
    const previous = () => {
        //alert('Previous btn clicked. Make a call to your server to fetch data.');
     };

    const next = () => {
        //alert('Next btn clicked. Make a call to your server to fetch data.');
    }


    let documents = [{"doc_name": "", "doc_owner": "", "body": ""}];

    async function fetchDocuments() {
        try {
            const params = {
                doc_owner: "Suru",
            };
            const response = await axios.get(`${backendUrl}/doc/get`, {params});
            console.log(response);
            documents = response.data;
        } catch (error) {
            console.error('Error fetching documents:', error);
        }
    }

    // Call the fetch function when the component mounts
    onMount(() => {
        fetchDocuments();
    });
</script>

<main class="bg-white dark:bg-gray-800">
    <Table style="width: 95%; margin-top: 3rem; margin-left: auto; margin-right: auto;" striped={true}>
        <TableHead>
            <TableHeadCell>Document Name</TableHeadCell>
            <TableHeadCell>Document Owner</TableHeadCell>
            <TableHeadCell>Body</TableHeadCell>
            <TableHeadCell>Edit</TableHeadCell>
            <TableHeadCell>Delete</TableHeadCell>
        </TableHead>
        <TableBody tableBodyClass="divide-y">
            {#each documents as document}
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
    </Table>
    
    <div class="flex flex-col items-center justify-center gap-2 mt-5">
      <div class="text-sm text-gray-700 dark:text-gray-400">
        Showing <span class="font-semibold text-gray-900 dark:text-white">{helper.start}</span>
        to
        <span class="font-semibold text-gray-900 dark:text-white">{helper.end}</span>
        of
        <span class="font-semibold text-gray-900 dark:text-white">{helper.total}</span>
        Entries
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
</main>

<style>
</style>