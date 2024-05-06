<script lang="ts">
    import { onMount } from 'svelte'
    import { writable } from 'svelte/store'
    import { invoke } from '@tauri-apps/api/core'
    import { listen } from '@tauri-apps/api/event'
    import { getCurrent } from '@tauri-apps/api/window'

    const appWindow = getCurrent()

    const url = writable<string>('')
    const isFullScreen = writable<boolean>(false)

    async function updateHistory(event: Event) {
        const button = event.currentTarget as HTMLButtonElement
        const state = button.name
        await invoke('update_history', { state })
    }

    const close = async () => await appWindow.close()

    const minimize = async () => await appWindow.minimize()

    const toggleMaximize = async () => {
        await appWindow.toggleMaximize()
        isFullScreen.update((fullScreen) => !fullScreen)
    }

    onMount(() => {
        ;(async () => {
            const fullScreen = await appWindow.isFullscreen()
            isFullScreen.set(fullScreen)

            const unlisten = await listen<string>('url_update', (event) => {
                const updated_url = event?.payload
                url.set(updated_url)
            })

            return () => unlisten()
        })()
    })
</script>

<main data-tauri-drag-region class="flex h-9 w-full items-center">
    <div class="flex justify-center items-center h-full w-9 border-2 border-gray-600">
        <img src="../../src/assets/icon.ico" alt="logo" class="w-6 h-6" />
    </div>

    <div class="flex-col justify-center h-9 w-1/2 lg:w-3/5 md:max-w-screen-md lg:max-w-screen-lg">
        <input
            readonly
            value={$url}
            class="w-full bg-black border-gray-500 border-2 text-gray-300 h-8 my-0.5 px-2 rounded-md"
        />
    </div>

    <div class="flex justify-center items-center w-24 h-9 gap-x-2 mr-2">
        <button
            on:click={minimize}
            class="select-none inline-flex h-7 w-7 justify-center items-center hover:rounded-md hover:bg-zinc-700"
        >
            <span class="text-xl mb-1.5 text-slate-200">{@html '&UnderBar;'}</span>
        </button>

        <button
            on:click={toggleMaximize}
            class="select-none inline-flex h-7 w-7 justify-center items-center hover:rounded-md hover:bg-zinc-700"
        >
            {#if $isFullScreen}
                <svg
                    viewBox="-4 -6 28 28"
                    xmlns="http://www.w3.org/2000/svg"
                    class="ml-1 stroke-slate-200 fill-slate-200 w-8 h-8"
                >
                    <path
                        d="M5.5 0a.5.5 0 0 1 .5.5v4A1.5 1.5 0 0 1 4.5 6h-4a.5.5 0 0 1 0-1h4a.5.5 0 0 0 .5-.5v-4a.5.5 0 0 1 .5-.5m5 0a.5.5 0 0 1 .5.5v4a.5.5 0 0 0 .5.5h4a.5.5 0 0 1 0 1h-4A1.5 1.5 0 0 1 10 4.5v-4a.5.5 0 0 1 .5-.5M0 10.5a.5.5 0 0 1 .5-.5h4A1.5 1.5 0 0 1 6 11.5v4a.5.5 0 0 1-1 0v-4a.5.5 0 0 0-.5-.5h-4a.5.5 0 0 1-.5-.5m10 1a1.5 1.5 0 0 1 1.5-1.5h4a.5.5 0 0 1 0 1h-4a.5.5 0 0 0-.5.5v4a.5.5 0 0 1-1 0z"
                    />
                </svg>
            {:else}
                <span class="text-2xl mb-1 text-slate-200">{@html '&#10064;'}</span>
            {/if}
        </button>

        <button
            on:click={close}
            class="select-none inline-flex h-7 w-7 justify-center items-center hover:rounded-md hover:bg-zinc-700"
        >
            <span class="ml-0.5 text-2xl text-slate-200">{@html '&#10005;'}</span>
        </button>
    </div>
</main>
