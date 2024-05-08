export type Site = {
    id: string
    url: string
    ico: string
}

export const genSvg = (id: string) => `https://api.iconify.design/simple-icons:${id}.svg`

export const HOME: Site = {
    id: 'home',
    url: '../../panel.html',
    ico: '&#x2b;'
}

export const sites: Site[] = [
    {
        id: 'google',
        url: 'https://google.com',
        ico: genSvg('https://google.com')
    },
    {
        id: 'github',
        url: 'https://github.com',
        ico: genSvg('https://github.com')
    },
    {
        id: 'spotify',
        url: 'https://open.spotify.com',
        ico: genSvg('https://spotify.com')
    },
    {
        id: 'tauri',
        url: 'https://beta.tauri.app',
        ico: genSvg('https://tauri.app')
    },
    {
        id: 'gmail',
        url: 'https://mail.google.com',
        ico: genSvg('https://mail.google.com')
    },
    {
        id: 'youtube',
        url: 'https://youtube.com',
        ico: genSvg('https://youtube.com')
    }
]
