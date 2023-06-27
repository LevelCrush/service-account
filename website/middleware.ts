import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';
import { getIronSession } from 'iron-session/edge';
import { sessionOptions } from '@website/core/session';
import { AccountResponse } from '@website/core/api_responses';
import ENV from '@website/core/env';

export const middleware = async (req: NextRequest) => {
  const res = NextResponse.next();
  const session = await getIronSession(req, res, sessionOptions);

  // do anything with session here:
  const { challenge } = session;
  const timestamp = Date.now() / 1000;
  let allowInAdmin = false;
  if (challenge) {
    console.log('Performing session challenge...', challenge, timestamp);
    const challenge_request = await fetch(
      ENV.hosts.accounts + '/profile/challenge',
      {
        method: 'POST',
        cache: 'no-store',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          challenge: challenge,
        }),
      }
    );

    if (challenge_request.ok) {
      const challenge_response =
        (await challenge_request.json()) as AccountResponse;
      if (
        challenge_response &&
        challenge_response.response &&
        challenge_response.response.is_admin === true
      ) {
        console.log('Challenge succeeded! Is Admin: ', challenge, timestamp);
        allowInAdmin = true;
      } else {
        console.warn('Challenge failed. Not admin: ', challenge, timestamp);
      }
    } else {
      console.warn('Challenge could not be reached!', timestamp);
    }
  }

  // demo:
  if (!allowInAdmin) {
    return new NextResponse(null, { status: 403 }); // unauthorized to see pages inside admin/
  }

  return res;
};

export const config = {
  matcher: ['/admin/:path*'],
};
