import { withIronSessionApiRoute } from 'iron-session/next';
import { sessionOptions } from '@website/core/session';
import { NextApiRequest, NextApiResponse } from 'next';

export default withIronSessionApiRoute(logoutRoute, sessionOptions);

function logoutRoute(req: NextApiRequest, res: NextApiResponse) {
  req.session.destroy();
  res.json({ isLoggedIn: false, login: '', avatarUrl: '' });
}
