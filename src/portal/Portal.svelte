<script lang="ts">
    import { writable } from 'svelte/store'

    import Input from './components/Input.svelte'
    import SiteGrid from './components/SiteGrid.svelte'

    import { isValidAddress } from '../utils/validators'
    import { sites, type Site } from '../utils/constants'

    const search = writable<string>('')
    const site = writable<Site | null>(null)
    const favourites = writable<Site[]>(sites)

    function filterList(event: CustomEvent) {
        const searchValue = event.detail.value
        search.set(searchValue)
        const filteredList = sites.filter(({ id }) => id.startsWith(searchValue))

        favourites.set(filteredList)
        updateSite(searchValue)
    }

    function updateSite(value: string) {
        if (!isValidAddress(value)) {
            return site.set(null)
        }

        const possibleUrl = value.startsWith('http') ? value : `https://${value}`
        const url = new URL(possibleUrl)

        site.set({
            id: url.host,
            url: url.href,
            ico: `https://icons.duckduckgo.com/ip3/${url.host}.ico`
        })
    }
</script>

<main class="flex-col h-dvh justify-center mx-auto bg-gray-800 p-20">
    <div class="flex-col my-10 mx-auto w-full max-w-screen-md">
        <Input on:search={filterList} />

        {#if !!$favourites.length}
            <SiteGrid sites={$favourites} title="Favourites" />
        {:else if $site}
            <SiteGrid sites={[$site]} title="Found" />
        {:else}
            <div
                class="flex gap-2 text-2xl text-slate-400 justify-center items-center mt-10 h-72 bg-gray-700 rounded-xl"
            >
                <span class="relative top-1">{@html '&#128269;'}</span>
                <p>{$search}</p>
            </div>
        {/if}
    </div>
</main>
