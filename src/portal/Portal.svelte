<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'

    type Site = { id: string, ico: string, url: string }

    import { sites, type Site } from '../utils/constants'

    const favourites = writable<Site[]>(sites);

    async function setActionTab(e: MouseEvent) {
        const button = e.currentTarget as HTMLButtonElement;
        await invoke('set_webview_url', { url: button.value })
    }
</script>

<main class="flex-col h-dvh justify-center mx-auto bg-gray-800 p-20">
    <div class="flex-col my-10 mx-auto w-full max-w-screen-md">
        <div class="relative">
            <input 
                placeholder="Enter keywords or a url"
                class="w-full mx-auto h-12 rounded-3xl px-12 py-2"
            />
            <img 
                class="absolute left-2 top-2 w-8 h-8"
                src="https://icons.duckduckgo.com/ip3/google.com.ico"
            />
        </div>

        <h2 class="text-slate-100 text-xl mt-10">Favourites</h2>

        <div class="flex flex-wrap gap-4 my-10 mx-auto">
            {#each sites as site (site.id)}
                <button 
                    name={site.id}
                    value={site.url}
                    on:click={setActionTab}
                    class="flex flex-col bg-slate-400 rounded-md place-content-center justify-center items-center gap-2 w-20 h-20"
                >
                    {#if site.ico.startsWith('https')}
                        <img class="inline-flex justify-center w-6 h-6 bg-transparent" src={site.ico} />
                    {:else}
                        <span class="flex relative -top-3 justify-center item-center w-5 h-5 text-3xl text-white">
                            {@html site.ico}
                        </span>
                    {/if}
                    <label for={site.id} class="text-sm text-slate-100">{site.id}</label>
                </button>
            {/each}
        </div>
    </div>
</main>
