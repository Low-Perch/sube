<script lang="ts">
    import { writable } from 'svelte/store'
    import { invoke } from '@tauri-apps/api/core'
    import { createEventDispatcher } from 'svelte'

    const dispatch = createEventDispatcher()
    const GOOGLE_SEARCH = 'https://google.com/search?q='

    let search = writable<string>('')
    function onInput(e: Event) {
        const input = e.currentTarget as HTMLInputElement
        const value = input?.value

        search.set(value)
        dispatch('search', { value })
    }

    async function searchQueryInGoogle() {
        const searchUrl = `${GOOGLE_SEARCH}${$search}`
        await invoke('set_webview_url', { url: searchUrl })
    }

    async function onKeyDown(event: KeyboardEvent) {
        if (event.key !== 'Enter') return

        await searchQueryInGoogle()
    }
</script>

<div class="relative">
    <input
        autocorrect="off"
        autocapitalize="off"
        on:input={onInput}
        on:keydown={onKeyDown}
        placeholder="Enter keywords or a url"
        class="w-full mx-auto h-12 rounded-3xl px-12 py-2"
    />
    <img
        class="absolute left-2 top-2 w-8 h-8"
        alt="google search"
        src="https://icons.duckduckgo.com/ip3/google.com.ico"
    />
</div>
