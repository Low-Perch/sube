import { invoke } from '@tauri-apps/api/core'

import { genSvg, HOME } from '../utils/constants'

// TODO: get types form rust Persona & Site structs
export const loadPersonaSites = async () => {
    const persona = await invoke('get_persona', { id: null })
    return [HOME, ...persona.sites.map((site) => ({ ...site, ico: genSvg(site.id) }))]
}
