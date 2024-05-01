const shortcuts = {
    'ArrowUp': () => scrollTo(0, 0),
    'ArrowDown': () => scrollTo(0, document.body.scrollHeight),
    '[': () => window.history.back(),
    ']': () => window.history.forward(),
    'r': () => window.location.reload(),
    '-': () => zoomOut(),
    '=': () => zoomIn(),
    '+': () => zoomIn(),
    '0': () => setZoom('100%'),
};

function setZoom(zoom) {
    const html = document.getElementsByTagName('html')[0];
    html.style.zoom = zoom
    window.localStorage.setItem('htmlZoom', zoom)
}

function zoomCommon(zoomChange) {
    const currentZoom = window.localStorage.getItem('htmlZoom') || '100%'
    setZoom(zoomChange(currentZoom))
}

function zoomIn() {
    zoomCommon((currentZoom) => `${Math.min(parseInt(currentZoom) + 10, 200)}%`)
}

function zoomOut() {
    zoomCommon((currentZoom) => `${Math.max(parseInt(currentZoom) - 10, 30)}%`)
}

function handleShortcut(event) {
    if (shortcuts[event.key]) {
        event.preventDefault()
        shortcuts[event.key]()
    }
}

document.addEventListener('keyup', (event) => {
    if (/windows|linux/i.test(navigator.userAgent) && event.ctrlKey) { handleShortcut(event) }
    if (/macintosh|mac os x/i.test(navigator.userAgent) && event.metaKey) { handleShortcut(event) }
})
