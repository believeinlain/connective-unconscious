import * as cookie from 'cookie';
import jwt from 'jsonwebtoken';
import { env } from '$env/dynamic/private';

export async function handle({ event, resolve }) {
  const cookies = cookie.parse(event.request.headers.get('cookie') || '');
  const token = cookies.jwt;

  if (token) {
    jwt.verify(cookies.jwt, env.JWT_SECRET, function (err, decoded) {
      if (err) console.log(err);
      else {
        console.log('Serving page for logged in user:', decoded.username);
        event.locals.user = decoded.username;
      }
    });
  }

  return await resolve(event);
}

export function getSession({ locals }) {
  return {
    user: locals.user
  };
}
