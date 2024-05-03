<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { getCurrent } from '@tauri-apps/api/window'

    const appWindow = getCurrent()

    async function goBack() {
        await invoke('update_history', { state: 'Back' })
    }
    async function goForward() {
        await invoke('update_history', { state: 'Forward' })
    }
    async function reload() {
        await invoke('update_history', { state: 'Reload' })
    }

    async function updateHistory(event: Event) {
        const button = event.currentTarget as HTMLButtonElement
        const state = button.name
        await invoke('update_history', { state })
    }

    const close = () => appWindow.close()

    const minimize = () => appWindow.minimize()

    const toggleMaximize = () => appWindow.toggleMaximize()
</script>

<main data-tauri-drag-region class="flex h-9 w-full justify-between items-center">
    <div
        class="flex justify-center items-center relative left-8 w-28 gap-x-2 h-full place-items-center"
    >
        <button
            name="Back"
            on:click={updateHistory}
            class="select-none inline-flex h-7 w-7 justify-center items-center hover:rounded-md hover:bg-zinc-700"
        >
            <svg
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg"
                class="mr-1 stroke-slate-200 fill-slate-200 w-8 h-8"
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
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg"
                class="ml-1 stroke-slate-200 fill-slate-200 w-8 h-8"
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
                viewBox="0 -4 24 24"
                xmlns="http://www.w3.org/2000/svg"
                class="ml-2 stroke-slate-200 fill-slate-200 w-8 h-8"
            >
                <path
                    class="fill-slate-200"
                    d="M14 8.495v-.5h-1v.5zM7.5 2.999H8v-1h-.5zm1-.5l.353.353l.354-.353l-.354-.354zM13 8.495a5.499 5.499 0 0 1-5.5 5.496v1c3.589 0 6.5-2.909 6.5-6.496zM7.5 13.99A5.499 5.499 0 0 1 2 8.495H1a6.499 6.499 0 0 0 6.5 6.496zM2 8.495a5.499 5.499 0 0 1 5.5-5.496v-1A6.499 6.499 0 0 0 1 8.495zM6.147.854l2 1.998l.706-.707l-2-1.999zm2 1.291l-2 1.999l.706.707l2-1.999z"
                />
            </svg>
        </button>
    </div>

    <div class="flex-col justify-center h-9 w-1/2 lg:w-3/5 md:max-w-screen-md lg:max-w-screen-lg">
        <input
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
            <span class="text-2xl mb-1 text-slate-200">{@html '&#10064;'}</span>
        </button>

        <button
            on:click={close}
            class="select-none inline-flex h-7 w-7 justify-center items-center hover:rounded-md hover:bg-zinc-700"
        >
            <span class="ml-0.5 text-2xl text-slate-200">{@html '&#10005;'}</span>
        </button>
    </div>
</main>
