// serve-static.js
import { handler } from './build/handler.js';
import express from 'express';

const app = express();

app.use(express.static('static'));

// let SvelteKit handle everything else, including serving prerendered pages and static assets
app.use(handler);

// This port will be the main entry point for the app
// Make sure the host is configured properly to point to :8080
const port = 8080;

app.listen(port, () => {
  console.log('Serving static files on port ' + port);
});
