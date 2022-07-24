// serve-static.js
import { port } from './build/index.js';
import { handler } from './build/handler.js';
import express from 'express';

const app = express();

app.use(express.static('static'))

// let SvelteKit handle everything else, including serving prerendered pages and static assets
app.use(handler);

const host = '127.0.0.1';

app.listen({ host, port }, () => {
	console.log('Serving static files on port ' + port);
});