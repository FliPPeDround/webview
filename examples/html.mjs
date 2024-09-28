import { Application } from '../index.js';

const app = new Application();
const window = app.createBrowserWindow({
  html: `<!DOCTYPE html>
    <html>
        <head>
            <title>Webview</title>
        </head>
        <body>
            <h1>Hello world!</h1>
        </body>
    </html>
    `,
});

window.setTitle('WebviewJS + Node');

app.run();
