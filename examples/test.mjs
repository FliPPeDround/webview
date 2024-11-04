import { Application, Theme } from '../index.js';

const app = new Application();
const window = app.createBrowserWindow()

window.createWebview({
  url: 'https://www.bilibili.com',
  y: 200,
  isChild: true,
})
window.createWebview({
  url: 'https://www.bilibili.com',
  x: 0,
  y: 0,
  height: 200,
  isChild: true,
})
app.run()
