
import { env } from '$env/dynamic/private';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function GET() {
    console.log("Server:", env.CU_BLOG_WRITER_PWD);
    return {
        status: 200,
        headers: {
            'access-control-allow-origin': '*'
        },
        body: {
            pwd: env.CU_BLOG_WRITER_PWD
        }
    };
}

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function PUT({ request }) {
    console.log("Server:", request);
    await new Promise(resolve => setTimeout(resolve, 1000));
    return {
        status: 201,
        headers: {
            'access-control-allow-origin': '*'
        },
        body: {
            result: "Success"
        }
    };
}
