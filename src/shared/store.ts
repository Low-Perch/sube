import { writable, get } from 'svelte/store'
import { invoke } from '@tauri-apps/api/core'
import { emitTo } from '@tauri-apps/api/event'
import { getCurrent } from '@tauri-apps/api/webview'

import { HOME, type Site, genSvg } from '../utils/constants'

// TODO: write types for Persona object
export const personas = writable()

export const tabs = writable<Site[]>([])
export const profile = writable<string>('')
export const profiles = writable<string[]>([])
export const activeTab = writable<Site | null>(HOME)

export const setActionTab = async (e: MouseEvent) => {
    const button = e.currentTarget as HTMLButtonElement
    const webview = getCurrent()
    const isPanel = webview.label == 'panel'

    if (isPanel) {
        const site = get(tabs).find(({ id }) => id == button.name)
        site && activeTab.set(site)
    } else {
        await emitTo('panel', 'switch_tab', { tab: button.name })
    }

    await invoke('set_webview_url', { url: button.value })
}

const addIconsToSites = (sites) => {
    return [HOME, ...sites.map((site) => ({ ...site, ico: genSvg(site.id) }))]
}

const loadInit = async () => {
    const { profile, profiles, personas } = await invoke('init')
    return { profile, profiles, personas: personas.personas }
}

export const loadData = async () => {
    const { profile: current, profiles: list, personas: config } = await loadInit()
    const sites = addIconsToSites(config[current].sites)

    profile.set(current)
    personas.set(config)
    profiles.set(list)
    tabs.set(sites)
}

export const updateProfile = (newProfile: string) => {
    profile.set(newProfile)
    const sites = addIconsToSites(get(personas)[newProfile].sites)
    tabs.set(sites)
}
