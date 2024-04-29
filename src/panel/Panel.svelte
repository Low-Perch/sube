<script lang="ts">
    import { onMount } from 'svelte'
    import { writable } from 'svelte/store'
    import { invoke } from '@tauri-apps/api/core'
    import { listen } from '@tauri-apps/api/event'

    import { sites, type Site } from '../utils/constants'

    let activeTab = writable<Site>(sites.at(0))

    async function setActionTab(e: MouseEvent) {
        const button = e.currentTarget as HTMLButtonElement;
        const site = sites.find(({ id }) => id == button.name)
        if (!site) return

        activeTab.set(site)
        await invoke('set_webview_url', { url: site.url })
    }

    onMount(() => {
        (async () => {
            listen
            const unlisten = await listen<{ tab: string }>('switch_tab', event => {
                const tab = event?.payload?.tab
                if (!tab) return

                const site = sites.find(({ id }) => id == tab)
                site && activeTab.set(site)
            })

            return () => unlisten()
        })()
    })
</script>

<main class="fixed h-full w-full justify-center items-center py-2">
    <ul class="relative grid grid-flow-row place-content-start gap-y-2">
        {#each sites as site (site.id)}
            <li id={site.id} class="relative inline-flex items-center mx-auto w-8 h-8">
                <hr 
                    class:hidden={site.id != $activeTab.id}
                    class="absolute -left-3 bg-slate-100 h-0.5 w-7 border-none rotate-90"
                />
                <button 
                    name={site.id}
                    value={site.url}
                    on:click={setActionTab}
                    class="inline-flex ml-2 justify-center items-center"
                >
                    {#if site.ico.startsWith('https')}
                        <img class="w-5 h-5 bg-transparent" src={site.ico} />
                    {:else}
                        <span class="flex relative -top-2.5 item-center w-5 h-5 text-3xl text-white">{@html site.ico}</span>
                    {/if}
                    <label for={site.id} class="text-xs hidden">{site.id}</label>
                </button>
            </li>
        {/each}
    </ul>
</main>
