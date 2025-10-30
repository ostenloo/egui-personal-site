const cacheName = 'egui-template-pwa';
const filesToCache = [
  './',
  './index.html',
  './ps.js',
  './ps_bg.wasm',
];

const swUrl = new URL(self.location.href);
const isLocalDev =
  swUrl.hostname === '127.0.0.1' ||
  swUrl.hostname === 'localhost' ||
  swUrl.hostname.endsWith('.local');

self.addEventListener('install', event => {
  if (isLocalDev) {
    self.skipWaiting();
    return;
  }

  event.waitUntil(
    caches.open(cacheName).then(cache => cache.addAll(filesToCache))
  );
});

self.addEventListener('activate', event => {
  if (isLocalDev) {
    event.waitUntil(
      (async () => {
        const keys = await caches.keys();
        await Promise.all(keys.map(key => caches.delete(key)));
        await self.clients.claim();
        await self.registration.unregister();
      })()
    );
    return;
  }

  event.waitUntil(self.clients.claim());
});

if (!isLocalDev) {
  self.addEventListener('fetch', event => {
    event.respondWith(
      caches.match(event.request).then(response => response || fetch(event.request))
    );
  });
}
