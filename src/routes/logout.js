export function POST() {
  console.log('User logged out');
  return {
    headers: {
      'set-cookie': 'jwt=deleted; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT'
    },
    body: {
      ok: true
    }
  };
}
