import '../styles.css'
import TitleBar from './TitleBar.svelte'

const app = new TitleBar({
    target: document.getElementById('title_bar'),
})

export default app
