<script lang="ts">
    import { onMount } from 'svelte'
    import { writable, derived } from 'svelte/store'

    import Input from './components/Input.svelte'
    import SiteGrid from './components/SiteGrid.svelte'

    import { isValidAddress } from '../utils/validators'
    import { loadSites, activeTab, tabs } from '../shared/store'

    const search = writable<string>('')

    const filteredTabs = derived([tabs, search], ([$tabs, $search]) => {
        const searchValue = $search.toLowerCase().trim()
        return $tabs.filter(({ id }) => id.toLowerCase().startsWith(searchValue))
    })

    function filterList(event: CustomEvent) {
        const searchValue = event.detail.value
        search.set(searchValue)
        updateSite(searchValue)
    }

    function updateSite(value: string) {
        if (!isValidAddress(value)) {
            return activeTab.set(null)
        }

        const possibleUrl = value.startsWith('http') ? value : `https://${value}`
        const url = new URL(possibleUrl)

        const hostParts = url.host.split('.')
        const index = hostParts.length == 3 ? 1 : 0
        const id = hostParts.at(index)

        activeTab.set({
            id: url.host,
            url: url.href,
            ico: `https://api.iconify.design/simple-icons:${id}.svg`
        })
    }

    onMount(() => {
        ;(async () => {
            await loadSites()
        })()
    })
</script>

<main class="flex-col h-dvh justify-center mx-auto bg-gray-800 p-20">
    <div class="flex-col my-10 mx-auto w-full max-w-screen-md">
        <Input on:search={filterList} />

        {#if !!$filteredTabs.length}
            <SiteGrid sites={$filteredTabs} title="Favourites" />
        {:else if $activeTab}
            <SiteGrid sites={[$activeTab]} title="Found" />
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
