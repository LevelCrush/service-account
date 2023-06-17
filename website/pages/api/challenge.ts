import { AccountResponse } from '@website/core/api_responses';
import ENV from '@website/core/env';
import { sessionOptions } from '@website/core/session';
import { withIronSessionApiRoute } from 'iron-session/next';
import { NextApiRequest, NextApiResponse } from 'next';

async function loginChallenge(req: NextApiRequest, res: NextApiResponse) {
  try {
    const challenge = await req.body;
    req.session.challenge = challenge as string;
    await req.session.save();
  } catch (err) {
    console.log('Could not fetch or save the challenge');
  }
  res.status(200).send('200 OK');
}

export default withIronSessionApiRoute(loginChallenge, sessionOptions);
