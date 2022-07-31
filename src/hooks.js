import * as cookie from 'cookie';
import * as jwt from 'jsonwebtoken';
import { env } from '$env/dynamic/private';

export async function handle({ event, resolve }) {
  const cookies = cookie.parse(event.request.headers.get('cookie') || '');
  const token = cookies.jwt;

  if (token) {
    jwt.verify(cookies.jwt, env.JWT_SECRET, function (err, decoded) {
      console.log('secret', env.JWT_SECRET);
      if (err) console.log(err);
      else {
        console.log('jwt verified:', decoded);
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
