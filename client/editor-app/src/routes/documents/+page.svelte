<script>
    import { onMount } from 'svelte';
    import { backendUrl } from '../../config';
    import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from 'flowbite-svelte';
    import axios from 'axios';

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

<main>
    <Table style="width: 95%; margin: auto;" stripped="true" color="purple">
        <TableHead>
            <TableHeadCell>Document Name</TableHeadCell>
            <TableHeadCell>Document Owner</TableHeadCell>
            <TableHeadCell>Body</TableHeadCell>
            <TableHeadCell>
                <span class="sr-only">Edit</span>
            </TableHeadCell>
        </TableHead>
        <TableBody tableBodyClass="divide-y">
            {#each documents as document}
                <TableBodyRow>
                    <TableBodyCell>{document.doc_name}</TableBodyCell>
                    <TableBodyCell>{document.doc_owner}</TableBodyCell>
                    <TableBodyCell>{document.body}</TableBodyCell>
                    <TableBodyCell>
                        <a href="/tables" class="font-medium text-primary-600 hover:underline dark:text-primary-500">Edit</a>
                    </TableBodyCell>
                </TableBodyRow>
            {/each}
        </TableBody>
    </Table>
</main>

<style>
</style>