import { writable, get } from 'svelte/store'
import { invoke } from '@tauri-apps/api/core'

import { HOME, type Site, genSvg } from '../utils/constants'

// TODO: write types for Persona object
export const personas = writable()

export const tabs = writable<Site[]>([])
export const profile = writable<string>('')
export const profiles = writable<string[]>([])
export const activeTab = writable<Site | null>(HOME)

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
