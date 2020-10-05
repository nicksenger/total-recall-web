importScripts('https://storage.googleapis.com/workbox-cdn/releases/5.1.2/workbox-sw.js');

const core = workbox.core;
const precaching = workbox.precaching;

core.skipWaiting();
core.clientsClaim();

precaching.precacheAndRoute(
  [
    { "revision": "0", "url": "/index.html" },
    { "revision": "0", "url": "/pkg/package_bg.wasm" }, 
    { "revision": "0", "url": "/pkg/package.js" },
    { "revision": "0", "url": "/icon.png" },
    { "revision": "0", "url": "/banner.jpeg" },
    { "revision": "0", "url": "/pkg/@spectrum-css/site.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/icon/index.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/vars/dist/spectrum-global.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/vars/dist/spectrum-medium.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/vars/dist/spectrum-light.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/vars/dist/spectrum-dark.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/page/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/button/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/divider/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/switch/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/typography/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/fieldlabel/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/textfield/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/link/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/table/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/icon/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/picker/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/menu/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/popover/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/inputgroup/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/buttongroup/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/underlay/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/dialog/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/modal/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/checkbox/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/actionbar/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/pagination/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/card/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/progressbar/dist/index-vars.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/icon/icons.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/icon/workflow-icons.css" },
    { "revision": "0", "url": "/pkg/@spectrum-css/icon/ui-icons.css" },
  ]
);
