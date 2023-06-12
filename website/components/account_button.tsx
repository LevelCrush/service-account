import React, { useEffect, useState } from 'react';
import Button from './elements/button';
import {
  AccountEvent,
  subscribeAccountEvent,
  unsubscribeAccountEvent,
} from './account_observer';
import ENV from '../core/env';

export const AccountButton = () => {
  const [displayName, setDisplayName] = useState('');
  const [loggedIn, setLoggedIn] = useState(false);

  // run this effect when the component mounts
  useEffect(() => {
    // subscribe to our events from our account observer
    const loginEvent = subscribeAccountEvent('account_login', () =>
      setLoggedIn(true)
    );
    const logoutEvent = subscribeAccountEvent('account_logout', () =>
      setLoggedIn(false)
    );
    const nameUpdatedEvent = subscribeAccountEvent(
      'account_name_updated',
      (ev) => setDisplayName(ev.detail)
    );

    // cleanup when the component unmounts
    return () => {
      unsubscribeAccountEvent('account_login', loginEvent);
      unsubscribeAccountEvent('account_logout', logoutEvent);
      unsubscribeAccountEvent('account_name_updated', nameUpdatedEvent);
    };
  }, []);

  const sendToLogin = () => {
    window.location.href =
      ENV.hosts.accounts +
      '/login?redirect=' +
      encodeURIComponent(window.location.href);
  };
  const sendToProfile = () => {
    window.location.href = '/profile';
  };

  if (loggedIn) {
    return (
      <Button intention="normal" onClick={sendToProfile}>
        {displayName}
      </Button>
    );
  } else {
    return (
      <Button intention="normal" onClick={sendToLogin}>
        Login With Discord
      </Button>
    );
  }
};

export default AccountButton;
