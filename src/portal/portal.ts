import '../styles.css'
import Portal from './Portal.svelte'

const app = new Portal({
    target: document.getElementById('portal')
})

export default app
