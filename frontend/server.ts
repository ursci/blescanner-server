const express = require('express');
const next = require('next');
const { createProxyMiddleware } = require('http-proxy-middleware');

const port = 3000
const dev = process.env.NODE_ENV !== 'production'
const API_URL = 'http://blescannerbackend:8080/api/v1/device_logs'

const app = next({ dev })
const handle = app.getRequestHandler()

app.prepare().then(() => {
  const server = express();

  server.use(
    '/device_logs',
    createProxyMiddleware({
      target: API_URL,
      pathRewrite: { [`^/device_logs`]: '' },
      changeOrigin: true
    })
  );

  server.all('*', (req, res) => {
    return handle(req, res)
  });

  server.listen(port, err => {
    if (err) throw err
    console.log(`> Ready on http://localhost:${port}`)
  });
});
