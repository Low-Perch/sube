<script lang="ts">
    import { setActionTab } from '../../shared/store'
    import { type Site } from '../../utils/constants'

    export let site: Site
</script>

<button
    name={site.id}
    value={site.url}
    on:click={setActionTab}
    class="flex flex-col bg-slate-400 rounded-md place-content-center justify-center items-center gap-2 min-w-20 min-h-20 aspect-video"
>
    {#if site.ico.startsWith('https')}
        <img alt={site.id} src={site.ico} class="inline-flex invert justify-center w-6 h-6" />
    {:else}
        <span class="flex relative -top-3 justify-center item-center w-5 h-5 text-3xl text-white">
            {@html site.ico}
        </span>
    {/if}

    <label for={site.id} class="text-sm text-slate-100">{site.id}</label>
</button>
