<script lang="ts">
    import { onMount } from 'svelte'
    import { writable } from 'svelte/store'
    import { invoke } from '@tauri-apps/api/core'

    type Site = { id: string, ico: string, url: string }

    let sites: Site[] = [
        {
            id: "twitter",
            url: "https://twitter.com",
            ico: 'https://icons.duckduckgo.com/ip3/twitter.com.ico'
        },
        {
            id: "tauri",
            url: "https://beta.tauri.app",
            ico: 'https://icons.duckduckgo.com/ip3/tauri.app.ico'
        },
        {
            id: "gmail",
            url: "https://mail.google.com",
            ico: 'https://icons.duckduckgo.com/ip3/mail.google.com.ico'
        },
        {

            id: "github",
            url: "https://github.com",
            ico: 'https://icons.duckduckgo.com/ip3/github.com.ico'
        },
        {

            id: "youtube",
            url: "https://youtube.com",
            ico: 'https://icons.duckduckgo.com/ip3/youtube.com.ico'
        },
    ]

    let activeTab = writable<Site>(sites.at(0))

    async function setActionTab(e: MouseEvent) {
        const button = e.currentTarget as HTMLButtonElement;
        await invoke('set_webview_url', { url: button.value })

        const site = sites.find(({ id }) => id == button.name)
        site && activeTab.set(site)
    }

    onMount(async () => {
        await invoke('set_webview_url', { url: $activeTab.url })
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
                    <img class="w-5 h-5 grayscale brightness-[100]" src={site.ico} />
                    <label for={site.id} class="text-xs hidden">{site.id}</label>
                </button>
            </li>
        {/each}
    </ul>
</main>
