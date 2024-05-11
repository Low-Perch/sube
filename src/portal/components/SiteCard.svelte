<script lang="ts">
    import { setActionTab } from '../../shared/store'
    import { type Site } from '../../utils/constants'

    export let site: Site
    export let portal: boolean = false

    const button =
        portal &&
        'flex flex-col bg-slate-400 rounded-md place-content-center justify-center items-center gap-2 min-w-20 min-h-20 aspect-video'
    const image = portal && 'inline-flex invert justify-center w-6 w-6'
    const icon = portal && 'flex relative -top-3 justify-center item-center w-5 h-5'
</script>

<button
    name={site.id}
    value={site.url}
    on:click={setActionTab}
    class="inline-flex ml-2 justify-center items-center {button ?? ''}"
>
    {#if site.ico.startsWith('https')}
        <img alt={site.id} src={site.ico} class="invert w-5 h-5 bg-transparent {image ?? ''}" />
    {:else}
        <span class="flex relative -top-2.5 item-center w-5 h-5 text-3xl text-white {icon ?? ''}">
            {@html site.ico}
        </span>
    {/if}

    <label class:hidden={!portal} for={site.id} class="text-sm text-slate-100">{site.id}</label>
</button>
