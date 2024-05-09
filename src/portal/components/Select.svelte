<script lang="ts">
    import { writable } from 'svelte/store'
    import { invoke } from '@tauri-apps/api/core'

    import { clickOutside } from '../../utils/directives'

    export let start: string
    export let options: string[]

    const open = writable<boolean>(false)
    let selection = writable<string>(start)

    function toggle() {
        open.update((open) => !open)
    }

    async function setPersona(event: MouseEvent) {
        const element = event.target as HTMLLIElement
        const persona = element?.innerText ?? 'me'
        selection.set(persona)
        open.set(false)
        await invoke('update_persona', { persona })
    }

    function outsideClick() {
        open.set(false)
    }
</script>

<div class="flex absolute max-w-screen-md top-5 left-5 z-50 w-1/5">
    <div data-name="select" class="relative w-64" use:clickOutside on:click_outside={outsideClick}>
        <button
            on:click={toggle}
            class:ring-blue-600={$open}
            class="flex w-full items-center relative justify-between rounded bg-gray-800 p-0.5 ring-1 ring-gray-600"
        >
            <span class="pl-2 text-lg text-slate-200">{$selection ?? 'Set Persona'}</span>
            <svg viewBox="0 0 256 256" xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 pr-2">
                <path
                    class="stroke-slate-200"
                    stroke-width="1.5rem"
                    d="m128 182a5.98159 5.98159 0 0 1 -4.24268-1.75732l-80-80a6.00006 6.00006 0 0 1 8.48536-8.48536l75.75732 75.75733 75.75732-75.75733a6.00006 6.00006 0 0 1 8.48536 8.48536l-80 80a5.98159 5.98159 0 0 1 -4.24268 1.75732z"
                />
            </svg>
        </button>

        <div
            class="flex flex-col z-2 absolute mt-1 w-full rounded bg-gray-700 ring-1 ring-gray-700"
            class:hidden={!$open}
        >
            {#each options as option, i (i)}
                <button
                    on:click={setPersona}
                    class="cursor-pointer select-none text-left p-1 pl-3 text-md text-slate-200 hover:bg-gray-600"
                >
                    {option}
                </button>
            {/each}
        </div>
    </div>
</div>
