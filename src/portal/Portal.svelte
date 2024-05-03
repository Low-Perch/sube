<script lang="ts">
    import { writable } from 'svelte/store'

    import Input from './components/Input.svelte'
    import SiteGrid from './components/SiteGrid.svelte'
    import { sites, type Site } from '../utils/constants'

    const favourites = writable<Site[]>(sites)

    function filterList(event: CustomEvent) {
        const searchValue = event.detail.value
        let filteredList = sites.filter(({ id }) => id.startsWith(searchValue))
        favourites.set(filteredList)
    }
</script>

<main class="flex-col h-dvh justify-center mx-auto bg-gray-800 p-20">
    <div class="flex-col my-10 mx-auto w-full max-w-screen-md">
        <Input on:search={filterList} />

        <SiteGrid sites={$favourites} title="Favourites" />
    </div>
</main>
