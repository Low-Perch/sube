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
</script>

<main>
    <ul>
        {#each list as item (item.id)}
            <li>
                <span>{item.id}</span>
                <img src={item.url} />
            </li>
        {/each}
    </ul>
</main>
