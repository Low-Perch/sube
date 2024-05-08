export type Site = { id: string; url: string; ico: string }

export const genSvg = (id: string) => `https://api.iconify.design/simple-icons:${id}.svg`

export const HOME: Site = {
    id: 'home',
    url: '../../panel.html',
    ico: '&#x2b;'
}
