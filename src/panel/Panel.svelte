<script lang="ts">
    import { onMount } from 'svelte'
    import { listen } from '@tauri-apps/api/event'

    import SiteCard from '../shared/components/SiteCard.svelte'
    import { loadData, updateProfile, tabs, activeTab } from '../shared/store'

    onMount(() => {
        ;(async () => {
            await loadData()

            const unlistenTabSwitch = await listen<{ tab: string }>('switch_tab', (event) => {
                const tab = event?.payload?.tab
                if (!tab) return

                const site = $tabs.find(({ id }) => id == tab)
                site && activeTab.set(site)
            })

            const unlistenPersonSwitch = await listen('new_persona', (event) => {
                const newProfile = event?.payload ?? 'me'
                updateProfile(newProfile)
            })

            return () => {
                unlistenTabSwitch()
                unlistenPersonSwitch()
            }
        })()
    })
</script>

<main class="fixed h-full w-full justify-center items-center py-2">
    <ul class="relative grid grid-flow-row place-content-start gap-y-2">
        {#each $tabs as site (site.id)}
            <li id={site.id} class="relative inline-flex items-center mx-auto w-8 h-8">
                <hr
                    class:hidden={site.id != $activeTab.id}
                    class="absolute -left-3 bg-slate-100 h-0.5 w-7 border-none rotate-90"
                />
                <SiteCard {site} />
            </li>
        {/each}
    </ul>
</main>
