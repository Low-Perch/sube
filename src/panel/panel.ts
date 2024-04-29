import '../styles.css'
import Panel from './Panel.svelte'

const app = new Panel({
    target: document.getElementById('panel'),
})

export default app
