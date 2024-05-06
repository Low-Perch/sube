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

<main class="select-none flex h-9 w-full items-center">
    <div class="flex select-none justify-center items-center h-full w-9">
        <img
            alt="logo"
            draggable="false"
            class="select-none w-6 h-6"
            src="../../src/assets/icon.ico"
        />
    </div>

    <div data-tauri-drag-region class="flex justify-between items-center h-8 w-full">
        <div class="flex items-center ml-6 w-28 gap-x-2 h-full">
            <button
                name="Back"
                on:click={updateHistory}
                class="select-none inline-flex h-7 w-7 justify-center items-center hover:rounded-md hover:bg-zinc-700"
            >
                <svg
                    viewBox="1 0 24 24"
                    xmlns="http://www.w3.org/2000/svg"
                    class="stroke-slate-200 fill-slate-200 w-7 h-7"
                >
                    <path
                        fill="currentColor"
                        d="M14.41 18.16L8.75 12.5l5.66-5.66l.7.71l-4.95 4.95l4.95 4.95z"
                    />
                </svg>
            </button>

            <button
                name="Forward"
                on:click={updateHistory}
                class="select-none inline-flex h-7 w-7 justify-center items-center hover:rounded-md hover:bg-zinc-700"
            >
                <svg
                    viewBox="-2 0 24 24"
                    xmlns="http://www.w3.org/2000/svg"
                    class="stroke-slate-200 fill-slate-200 w-7 h-7"
                >
                    <path
                        fill="currentColor"
                        d="m8.59 18.16l5.66-5.66l-5.66-5.66l-.7.71l4.95 4.95l-4.95 4.95z"
                    />
                </svg>
            </button>

            <button
                name="Reload"
                on:click={updateHistory}
                class="select-none inline-flex h-7 w-7 justify-center items-center hover:rounded-md hover:bg-zinc-700"
            >
                <svg
                    viewBox="-7 -6 28 28"
                    xmlns="http://www.w3.org/2000/svg"
                    class="stroke-slate-200 fill-slate-200 w-7 h-7"
                >
                    <path
                        class="fill-slate-200"
                        d="M14 8.495v-.5h-1v.5zM7.5 2.999H8v-1h-.5zm1-.5l.353.353l.354-.353l-.354-.354zM13 8.495a5.499 5.499 0 0 1-5.5 5.496v1c3.589 0 6.5-2.909 6.5-6.496zM7.5 13.99A5.499 5.499 0 0 1 2 8.495H1a6.499 6.499 0 0 0 6.5 6.496zM2 8.495a5.499 5.499 0 0 1 5.5-5.496v-1A6.499 6.499 0 0 0 1 8.495zM6.147.854l2 1.998l.706-.707l-2-1.999zm2 1.291l-2 1.999l.706.707l2-1.999z"
                    />
                </svg>
            </button>
        </div>

        <div
            class="flex-col justify-center h-9 w-1/2 lg:w-3/5 md:max-w-screen-md lg:max-w-screen-lg"
        >
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
                        viewBox="-5 -8 30 30"
                        xmlns="http://www.w3.org/2000/svg"
                        class="ml-1 stroke-slate-200 fill-slate-200 w-7 h-7"
                    >
                        <path
                            d="M5.5 0a.5.5 0 0 1 .5.5v4A1.5 1.5 0 0 1 4.5 6h-4a.5.5 0 0 1 0-1h4a.5.5 0 0 0 .5-.5v-4a.5.5 0 0 1 .5-.5m5 0a.5.5 0 0 1 .5.5v4a.5.5 0 0 0 .5.5h4a.5.5 0 0 1 0 1h-4A1.5 1.5 0 0 1 10 4.5v-4a.5.5 0 0 1 .5-.5M0 10.5a.5.5 0 0 1 .5-.5h4A1.5 1.5 0 0 1 6 11.5v4a.5.5 0 0 1-1 0v-4a.5.5 0 0 0-.5-.5h-4a.5.5 0 0 1-.5-.5m10 1a1.5 1.5 0 0 1 1.5-1.5h4a.5.5 0 0 1 0 1h-4a.5.5 0 0 0-.5.5v4a.5.5 0 0 1-1 0z"
                        />
                    </svg>
                {:else}
                    <span class="ml-0.5 text-2xl mb-1 text-slate-200">{@html '&#10064;'}</span>
                {/if}
            </button>

            <button
                on:click={close}
                class="select-none inline-flex h-7 w-7 justify-center items-center hover:rounded-md hover:bg-zinc-700"
            >
                <span class="ml-0.5 text-2xl text-slate-200">{@html '&#10005;'}</span>
            </button>
        </div>
    </div>
</main>
