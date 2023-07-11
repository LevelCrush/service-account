import React, { useEffect, useState } from 'react';
import ENV from '@website/core/env';
import useDeepCompareEffect from 'use-deep-compare-effect';
import { AccountResponse } from '@website/core/api_responses';

/**
 * An interface that details the expected events to emit from this observer and the expected types to feed to those dispatched events
 */
export interface AccountEvent {
  account_login: null;
  account_logout: null;
  account_platforms_updated: AccountResponse['response']['platforms'];
  account_name_updated: AccountResponse['response']['display_name'];
  account_admin_updated: AccountResponse['response']['is_admin'];
}

/**
 * Event dispatcher for this observer
 * @param event
 * @param detail
 */
function dispatchAccountEvent<K extends keyof AccountEvent>(
  event: K,
  detail: AccountEvent[K]
) {
  document.dispatchEvent(
    new CustomEvent(event, {
      detail: detail,
    })
  );
}

const accountEventListeners = {} as {
  [event: string]: ((ev: CustomEvent) => void)[];
};

export function subscribeAccountEvent<K extends keyof AccountEvent>(
  event: K,
  cb: (ev: CustomEvent<AccountEvent[K]>) => void
) {
  if (typeof accountEventListeners[event] === 'undefined') {
    accountEventListeners[event] = [];
  }
  const length = accountEventListeners[event].push(cb);
  const index = length - 1;
  document.addEventListener(
    event,
    accountEventListeners[event][index] as unknown as EventListenerObject
  );
  return index;
}

export function unsubscribeAccountEvent<K extends keyof AccountEvent>(
  event: K,
  handle: number
) {
  document.removeEventListener(
    event,
    accountEventListeners[event][handle] as unknown as EventListenerObject
  );
}

/**
 * Intended to be run at the top of the react document, handles emitting any events and data related to the account logins
 * @constructor
 */
export const AccountObserver = () => {
  const [didExternalCheck, setDidExternalCheck] = useState(false);
  const [displayName, setDisplayName] = useState('');
  const [loggedIn, setLoggedIn] = useState(false);
  const [platformData, setPlatformData] = useState(
    {} as AccountResponse['response']['platforms']
  );
  const [accountTimerInterval, setAccountTimerInterval] = useState(0);
  const [isAdmin, setIsAdmin] = useState(false);

  const saveChallenge = async (challenge_token: string) => {
    const endpoint = ENV.hosts.frontend + '/api/challenge';
    await fetch(endpoint, {
      method: 'POST',
      credentials: 'include',
      mode: 'cors',
      cache: 'no-store',
      body: challenge_token,
    });
  };

  const sendLogout = async () => {
    const endpoint = ENV.hosts.frontend + '/api/logout';
    await fetch(endpoint, {
      method: 'GET',
      credentials: 'include',
      mode: 'cors',
      cache: 'no-store',
    });
  };

  // setup an account session check
  const accountLoginCheck = async () => {
    console.log('Set in Host', ENV.hosts.accounts);
    const endpoint = ENV.hosts.accounts + '/profile/json';
    const response = await fetch(endpoint, {
      method: 'GET',
      credentials: 'include',
      mode: 'cors',
      cache: 'no-store',
    });

    const data = response.ok
      ? ((await response.json()) as AccountResponse)
      : null;

    if (data !== null && data.success) {
      window.localStorage.setItem(
        'session_display_name',
        data.response.display_name
      );
      window.localStorage.setItem(
        'session_is_admin',
        data.response.is_admin ? 'yes' : 'no'
      );
      window.localStorage.setItem('session_logged_in', 'yes');

      setDisplayName(data.response.display_name);
      setLoggedIn(true);
      setPlatformData(data.response.platforms);
      setIsAdmin(data.response.is_admin);
      saveChallenge(data.response.challenge).finally(() =>
        console.log('Challenge delivered')
      );
      setDidExternalCheck(true);
    } else {
      setDisplayName('');
      setPlatformData({});
      setLoggedIn(false);
      setIsAdmin(false);
      setDidExternalCheck(true);
      window.localStorage.removeItem('session_logged_in');
      window.localStorage.removeItem('session_is_admin');
      window.localStorage.removeItem('session_display_name');
    }
  };

  // when we mount our component we will run this effect
  useEffect(() => {
    if (window.localStorage) {
      setDisplayName(window.localStorage.getItem('session_display_name') || '');
      setIsAdmin(window.localStorage.getItem('session_is_admin') === 'yes');
      setLoggedIn(window.localStorage.getItem('session_logged_in') === 'yes');
    }

    // run the initial login check, we don't care
    accountLoginCheck();

    // when we unmount our component run this for cleanup
    return () => {
      // component unmount, stop timer
      window.clearInterval(accountTimerInterval);
      setAccountTimerInterval(0);
    };
  }, []);

  // whenever our login state changes we are going to fire off a event in the browser
  useEffect(() => {
    if (loggedIn) {
      setAccountTimerInterval(
        window.setInterval(() => {
          accountLoginCheck();
        }, 1000 * 60)
      );
    } else {
      if (didExternalCheck) {
        // attempt logout
        sendLogout();
        // log out, stop running interval
        window.clearInterval(accountTimerInterval);
        setAccountTimerInterval(0);
      } else {
        console.log('Have not yet finished doing the first external check');
      }
    }
    dispatchAccountEvent(loggedIn ? 'account_login' : 'account_logout', null);

    return () => {
      // component unmount, stop timer
      window.clearInterval(accountTimerInterval);
      setAccountTimerInterval(0);
    };
  }, [loggedIn]);

  // this is double work and absolutely redudanent.
  // but in the event a user **manually** alters the react state, this will fire off
  // there are better ways to get the same effect without having to repeat the same call
  // but at the same time, the redudant request will just return a cache copy regardless, no database overhead
  useEffect(() => {
    if (didExternalCheck) {
      accountLoginCheck()
        .then(() => console.log('Login state changed. Double confirming state'))
        .finally(() => console.log('Login double check completed'));
    } else {
      console.log(
        'Have not done the first external check yet. Cannot run duplicate check'
      );
    }
  }, [loggedIn, isAdmin]);

  // anytime our platform data changes this effect should run and we should emit an event
  useDeepCompareEffect(() => {
    dispatchAccountEvent('account_platforms_updated', platformData);
  }, [platformData]);

  // whenever our display name changes send out the event
  useEffect(() => {
    dispatchAccountEvent('account_name_updated', displayName);
  }, [displayName]);

  // listen to whenever we receive an admin change event
  useEffect(() => {
    dispatchAccountEvent('account_admin_updated', isAdmin);
  }, [isAdmin]);

  //  // listen to our own events for some debugging potential (probably should be disabled before production)
  useEffect(() => {
    const loginEvent = subscribeAccountEvent('account_login', () =>
      console.log('Logged in!')
    );
    const logoutEvent = subscribeAccountEvent('account_logout', () =>
      console.log('Logged out!')
    );
    const nameUpdateEvent = subscribeAccountEvent(
      'account_name_updated',
      (ev) => console.log('Name change', ev.detail)
    );
    const platformUpdateEvent = subscribeAccountEvent(
      'account_platforms_updated',
      (ev) => console.log('Platform data changed', ev.detail)
    );
    const adminUpdateEvent = subscribeAccountEvent(
      'account_admin_updated',
      (ev) => console.log('Account admin status changed', ev.detail)
    );

    // cleanup when component unmounts
    return () => {
      unsubscribeAccountEvent('account_login', loginEvent);
      unsubscribeAccountEvent('account_logout', logoutEvent);
      unsubscribeAccountEvent('account_name_updated', nameUpdateEvent);
      unsubscribeAccountEvent('account_platforms_updated', platformUpdateEvent);
      unsubscribeAccountEvent('account_admin_updated', adminUpdateEvent);
    };
  }, []);

  // we will take up no rendering node here , this component just handles logic
  return <></>;
};

export default AccountObserver;
