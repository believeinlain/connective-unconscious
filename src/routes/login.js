import * as bcrypt from 'bcrypt';
import jwt from 'jsonwebtoken';
import { env } from '$env/dynamic/private';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function POST({ request }) {
  const { username, password } = await request.json();

  let correct = false;
  if (username == env.WEBMASTER_UNAME) {
    console.log('User:', username);
    console.log('WEBMASTER_PWD_HASH', env.WEBMASTER_PWD_HASH);
    correct = await bcrypt.compare(password, env.WEBMASTER_PWD_HASH);
    console.log(correct ? 'Password match.' : 'Incorrect password.');
  } else {
    console.log('Incorrect username.');
  }

  if (!correct) {
    return {
      status: 401,
      headers: {
        'access-control-allow-origin': '*',
        'content-type': 'application/json'
      },
      body: {
        message: 'Incorrect username or password.'
      }
    };
  }

  const token = jwt.sign({ username }, env.JWT_SECRET);

  return {
    status: 200,
    headers: {
      'access-control-allow-origin': '*',
      'set-cookie': `jwt=${token}; Path=/; HttpOnly`,
      'content-type': 'application/json'
    },
    body: {
      user: username,
      message: 'Successfully signed in.'
    }
  };
}
