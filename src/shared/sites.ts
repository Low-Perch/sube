import { invoke } from '@tauri-apps/api/core'

import { genSvg, HOME } from '../utils/constants'

// TODO: get types form rust Persona & Site structs
export const loadPersonaSites = async () => {
    const sites = await invoke('get_sites')
    return [HOME, ...sites.map((site) => ({ ...site, ico: genSvg(site.id) }))]
}
