import React, { useEffect, useState } from 'react';
import ENV from '@website/core/env';
import useDeepCompareEffect from 'use-deep-compare-effect';

/**
 * Expected data from a platform if it is discord
 */
export interface AccountPlatformDiscord {
  display_name: string;
  discord_id: string;
}

/**
 * Expected data from a platform if it is twitch
 */
export interface AccountPlatformTwitch {
  profile_image_url: string;
  twitch_id: string;
  display_name: string;
  login: string;
  offline_image_url: string;
}

/**
 * Expected data from our AccountResponse if our platform is bungie related
 */
export interface AccountPlatformBungie {
  display_name: string;
  unique_name: string;
  memberships: string;
  raid_report: string;
  primary_membership_id: string;
  primary_platform: string;
  [membership_key: string]: string;
}

/**
 * The expected response from our server when querying for profile information
 */
export interface AccountResponse {
  success: boolean;
  response: {
    display_name: string;
    is_admin: boolean;
    platforms: {
      [platform: string]:
        | AccountPlatformDiscord
        | AccountPlatformBungie
        | AccountPlatformTwitch;
    };
  };
  errors: unknown[]; // this response will not return a route, so if there are any, it's unknown and we don't know how to handle it
}

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
  const [displayName, setDisplayName] = useState('');
  const [loggedIn, setLoggedIn] = useState(false);
  const [platformData, setPlatformData] = useState(
    {} as AccountResponse['response']['platforms']
  );
  const [accountTimerInterval, setAccountTimerInterval] = useState(0);

  const [isAdmin, setIsAdmin] = useState(false);
  const [storageIsAdmin, setStorageIsAdmin] = useState(
    window.localStorage.getItem('sess_admin') === 'yes'
  );

  const [storageDisplayName, setStorageDisplayName] = useState(
    window.localStorage.getItem('sess_display_name') === 'yes'
  );

  console.log('StorageLogge');

  // setup an account session check
  const accountLoginCheck = async () => {
    console.log('Set in Host', ENV.hosts.accounts);
    const endpoint = ENV.hosts.accounts + '/profile/json';
    const response = await fetch(endpoint, {
      method: 'GET',
      credentials: 'include',
      mode: 'cors',
      cache: 'no-store',
      next: {
        revalidate: 0,
      },
    });

    const data = response.ok
      ? ((await response.json()) as AccountResponse)
      : null;

    if (data !== null && data.success) {
      setDisplayName(data.response.display_name);
      setLoggedIn(true);
      setPlatformData(data.response.platforms);
      setIsAdmin(data.response.is_admin);
    } else {
      setDisplayName('');
      setPlatformData({});
      setLoggedIn(false);
      setIsAdmin(false);
    }
  };

  // when we mount our component we will run this effect
  useEffect(() => {
    // run the initial login check, we don't care
    accountLoginCheck().finally(() =>
      console.log('Initial login check completed')
    );

    // when we unmount our component run this for cleanup
    return () => {
      //
    };
  }, []);

  // whenever our login state changes we are going to fire off a event in the browser
  useEffect(() => {
    if (loggedIn) {
      setAccountTimerInterval(
        window.setInterval(() => {
          accountLoginCheck().finally(() =>
            console.log('Checking if logged in')
          );
        }, 1000 * 60)
      );
    } else {
      // log out, stop running interval
      clearInterval(accountTimerInterval);
      setAccountTimerInterval(0);
    }
    dispatchAccountEvent(loggedIn ? 'account_login' : 'account_logout', null);

    return () => {
      // component unmount, stop timer
      clearInterval(accountTimerInterval);
      setAccountTimerInterval(0);
    };
  }, [loggedIn]);

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
