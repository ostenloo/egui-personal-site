const RUNTIME_CACHE = 'egui-runtime-cache-v1';

const swUrl = new URL(self.location.href);
const isLocalDev =
  swUrl.hostname === '127.0.0.1' ||
  swUrl.hostname === 'localhost' ||
  swUrl.hostname.endsWith('.local');

self.addEventListener('install', () => {
  self.skipWaiting();
});

self.addEventListener('activate', event => {
  if (isLocalDev) {
    event.waitUntil(
      (async () => {
        const keys = await caches.keys();
        await Promise.all(keys.map(key => caches.delete(key)));
        await self.registration.unregister();
        await self.clients.claim();
      })()
    );
    return;
  }

  event.waitUntil(
    (async () => {
      const keys = await caches.keys();
      await Promise.all(
        keys
          .filter(key => key !== RUNTIME_CACHE)
          .map(key => caches.delete(key))
      );
      await self.clients.claim();
    })()
  );
});

if (!isLocalDev) {
  self.addEventListener('fetch', event => {
    const { request } = event;

    // Always go to the network first for navigation so we don't serve stale HTML.
    if (request.mode === 'navigate') {
      event.respondWith(
        (async () => {
          try {
            const networkResponse = await fetch(request);
            const cache = await caches.open(RUNTIME_CACHE);
            cache.put(request, networkResponse.clone());
            return networkResponse;
          } catch (_) {
            const cache = await caches.open(RUNTIME_CACHE);
            const cachedResponse =
              (await cache.match(request)) ?? (await cache.match('/'));
            if (cachedResponse) {
              return cachedResponse;
            }
            return new Response('Offline', { status: 503, statusText: 'Offline' });
          }
        })()
      );
      return;
    }

    // For other requests, serve cached data when available and refresh in background.
    event.respondWith((async () => {
      const cache = await caches.open(RUNTIME_CACHE);
      try {
        const response = await fetch(request);
        if (response && response.ok) {
          cache.put(request, response.clone());
        }
        return response;
      } catch (_) {
        const fallback = await cache.match(request);
        if (fallback) {
          return fallback;
        }
        throw _;
      }
    })());
  });
}
