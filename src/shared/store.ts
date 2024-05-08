import { writable } from 'svelte/store'

import { loadPersonaSites } from './sites'
import { HOME, type Site } from '../utils/constants'

export const tabs = writable<Site[]>([])
export const activeTab = writable<Site | null>(HOME)

export const loadSites = async () => {
    const sites = await loadPersonaSites()
    tabs.set(sites)
}
