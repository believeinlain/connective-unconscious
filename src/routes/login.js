
import * as bcrypt from 'bcrypt';
import { env } from '$env/dynamic/private';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function POST({ request }) {
    const { username, password } = await request.json();

    let correct = false;
    if (username == env.WEBMASTER_UNAME) {
        console.log("User:", username);
        correct = await bcrypt.compare(password, env.WEBMASTER_PWD_HASH);
        console.log(correct ? "Password match." : "Incorrect password.");
    } else {
        console.log("Incorrect username.");
    }

    return {
        status: correct ? 200 : 401,
        headers: {
            'access-control-allow-origin': '*',
            'accept': 'application/json',
        },
        body: {
            user: correct ? username : undefined,
            message: correct ? 'Successfully signed in.' :
                'Incorrect username or password.',
        },
    };
}
