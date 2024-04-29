<script lang="ts">
    import { writable } from 'svelte/store'
    import { invoke } from '@tauri-apps/api/core'

    type Site = { id: string, ico: string, url: string }

    let sites: Site[] = [
        {
            id: "home",
            url: "../../panel.html",
            ico: '&#x2b;'
        },
        {

            id: "google",
            url: "https://google.com",
            ico: 'https://icons.duckduckgo.com/ip3/google.com.ico'
        },
        {

            id: "github",
            url: "https://github.com",
            ico: 'https://icons.duckduckgo.com/ip3/github.com.ico'
        },
        {
            id: "spotify",
            url: "https://open.spotify.com",
            ico: 'https://icons.duckduckgo.com/ip3/spotify.com.ico'
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
