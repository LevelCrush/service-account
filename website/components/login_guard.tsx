import { faTriangleExclamation } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import React, { useEffect, useState } from 'react';
import Container from '@website/components/elements/container';
import { H2 } from '@website/components/elements/headings';
import AccountButton from '@website/components/account_button';
import {
  subscribeAccountEvent,
  unsubscribeAccountEvent,
} from '@website/components/account_observer';

export interface LoginGuardProps extends React.PropsWithChildren {
  admin?: boolean;
  hide?: boolean;
}

export const LoginGuard = (props: LoginGuardProps) => {
  const [loggedIn, setLoggedIn] = useState(false);
  const [isAdmin, setIsAdmin] = useState(false);
  // setup event hooks when our component mounts
  useEffect(() => {
    const loginEvent = subscribeAccountEvent('account_login', () =>
      setLoggedIn(true)
    );
    const logoutEvent = subscribeAccountEvent('account_logout', () =>
      setLoggedIn(false)
    );

    const adminUpdatedEvent = subscribeAccountEvent(
      'account_admin_updated',
      (ev) => setIsAdmin(ev.detail)
    );

    return () => {
      unsubscribeAccountEvent('account_login', loginEvent);
      unsubscribeAccountEvent('account_logout', logoutEvent);
      unsubscribeAccountEvent('account_admin_updated', adminUpdatedEvent);
    };
  }, []);

  if (
    (loggedIn && !props.admin) ||
    (loggedIn && props.admin === true && isAdmin === true)
  ) {
    return <>{props.children}</>;
  } else {
    return props.hide ? (
      <></>
    ) : (
      <Container
        minimalCSS={true}
        className="mx-auto my-8 flex items-center justify-center self-center min-h-full h-auto"
      >
        <div className="flex-initial w-2/4 h-auto text-center">
          <H2
            minimalCSS={true}
            className="text-6xl text-black dark:text-yellow-400 font-headline font-bold uppercase tracking-widest "
          >
            <FontAwesomeIcon icon={faTriangleExclamation}></FontAwesomeIcon>
            <br />
            <br />
            <span>Please Login</span>
          </H2>
          <p className="text-xl my-8">
            This area is restricted to logged in members only.
          </p>
          <div className="w-auto h-auto inline-block">
            <AccountButton />
          </div>
        </div>
      </Container>
    );
  }
};

export default LoginGuard;
