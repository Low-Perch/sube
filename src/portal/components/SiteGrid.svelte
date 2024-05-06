<script lang="ts">
    import SiteCard from './SiteCard.svelte'
    import { type Site } from '../../utils/constants'

    export let sites: Site[]
    export let title: string
</script>

<h2 class="text-slate-100 text-xl mt-10">{title}</h2>

<div class="flex flex-wrap gap-4 my-10 mx-auto">
    {#each sites as site (site.id)}
        <SiteCard {site} />
    {/each}
</div>
