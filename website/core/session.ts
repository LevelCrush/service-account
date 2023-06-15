// this file is a wrapper with defaults to be used in both API routes and `getServerSideProps` functions
import type { IronSessionOptions } from 'iron-session';

export const sessionOptions: IronSessionOptions = {
  password: process.env['IRON_SESSION_SECRET'] as string,
  cookieName: process.env['NEXT_PUBLIC_IRON_SESSION_COOKIE_NAME'] as string,
  cookieOptions: {
    secure: process.env.NODE_ENV === 'production',
  },
};

// This is where we specify the typings of req.session.*
declare module 'iron-session' {
  interface IronSessionData {
    challenge?: string;
  }
}
