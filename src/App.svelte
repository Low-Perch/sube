<script lang="ts">
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

    let active = "github"
</script>

<main class="flex w-full justify-center h-screen bg-black">
    <ul class="relative grid grid-flow-row place-content-start gap-y-4 py-4">
        {#each list as item (item.id)}
            <li id={item.id} class="relative inline-flex items-center justify-center border-1 rounded border-sno rounded-4 w-6 h-6">
                <hr 
                    class:hidden={item.id != active}
                    class="absolute -left-3 bg-slate-300 h-0.5 w-6 border-none rotate-90"
                />
                <img class="w-4 h-4 invert" src={item.url} />
                <label for={item.id} class="text-xs hidden">{item.id}</label>
            </li>
        {/each}
    </ul>
</main>
