<script lang="ts">
    import { writable } from 'svelte/store'

    let activeTab = writable("gmail")

    type Site = { id: string, url: string }
    let list: Site[] = [
        {
            id: "gmail",
            url: 'https://icons.duckduckgo.com/ip3/mail.google.com.ico'
        },
        {

            id: "github",
            url: 'https://icons.duckduckgo.com/ip3/github.com.ico'
        },
    ]

    function setActionTab(e: MouseEvent) {
        let button = e.currentTarget as HTMLButtonElement;
        let tab = button.name;
        activeTab.set(tab)
    }

</script>

<main class="fixed h-full w-full justify-center items-center py-2">
    <ul class="relative grid grid-flow-row place-content-start gap-y-2">
        {#each list as item (item.id)}
            <li id={item.id} class="relative inline-flex items-center mx-auto w-8 h-8">
                <hr 
                    class:hidden={item.id != $activeTab}
                    class="absolute -left-3 bg-slate-100 h-0.5 w-7 border-none rotate-90"
                />
                <button 
                    name={item.id}
                    on:click={setActionTab}
                    class="inline-flex ml-2 justify-center items-center"
                >
                    <img class="w-5 h-5 grayscale brightness-[100]" src={item.url} />
                    <label for={item.id} class="text-xs hidden">{item.id}</label>
                </button>
            </li>
        {/each}
    </ul>
</main>
