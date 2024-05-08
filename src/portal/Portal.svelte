<script lang="ts">
    import { onMount } from 'svelte'
    import { writable } from 'svelte/store'
    import { invoke } from '@tauri-apps/api/core'

    import Input from './components/Input.svelte'
    import SiteGrid from './components/SiteGrid.svelte'

    import { isValidAddress } from '../utils/validators'
    import { genSvg, HOME, type Site } from '../utils/constants'

    const search = writable<string>('')
    const tab = writable<Site | null>(null)
    const tabs = writable<Site[]>([])

    function filterList(event: CustomEvent) {
        const searchValue = event.detail.value
        search.set(searchValue)
        const filteredList = $tabs.filter(({ id }) => id.startsWith(searchValue))

        tabs.set(filteredList)
        updateSite(searchValue)
    }

    function updateSite(value: string) {
        if (!isValidAddress(value)) {
            return tab.set(null)
        }

        const possibleUrl = value.startsWith('http') ? value : `https://${value}`
        const url = new URL(possibleUrl)

        tab.set({
            id: url.host,
            url: url.href,
            ico: `https://icons.duckduckgo.com/ip3/${url.host}.ico`
        })
    }

    async function loadPanel() {
        const { sites } = await invoke('get_persona', { id: null })
        const list = [HOME, ...sites.map((site) => ({ ...site, ico: genSvg(site.id) }))]
        tabs.set(list)
    }

    onMount(() => {
        ;(async () => {
            await loadPanel()
        })()
    })
</script>

<main class="flex-col h-dvh justify-center mx-auto bg-gray-800 p-20">
    <div class="flex-col my-10 mx-auto w-full max-w-screen-md">
        <Input on:search={filterList} />

        {#if !!$tabs.length}
            <SiteGrid sites={$tabs} title="Favourites" />
        {:else if $tab}
            <SiteGrid sites={[$tab]} title="Found" />
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
