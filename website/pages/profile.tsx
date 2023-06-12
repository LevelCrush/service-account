import Head from 'next/head';
import React, { useEffect, useState } from 'react';
import Hero from '../components/hero';
import { SiteHeader } from '../components/site_header';
import DiscordLink from '../components/discord_link';
import Container from '../components/elements/container';
import { H2, H3, H4 } from '../components/elements/headings';
import OffCanvas from '../components/offcanvas';
import Image from 'next/image';
import LoginGuard from '../components/login_guard';
import Hyperlink from '../components/elements/hyperlink';
import {
  AccountPlatformBungie,
  subscribeAccountEvent,
  unsubscribeAccountEvent,
} from '../components/account_observer';
import { AccountLinkedPlatformResultMap } from '../core/api_responses';
import Button, { HyperlinkButton } from '../components/elements/button';
import ENV from '../core/env';
import GoosePost from '../components/goose_post';

type LinkedPlatformMap = {
  discord: string | null;
  twitch: string | null;
  bungie: string | null;
  [platform: string]: string | null;
};

export const ProfilePage = () => {
  const [platformData, setPlatformData] = useState({
    discord: null,
    twitch: null,
    bungie: null,
  } as LinkedPlatformMap);

  const [bungieAccountUrl, setBungieAccountUrl] = useState(
    ENV.hosts.accounts + '/platform/bungie/login'
  );
  const [twitchAccountUrl, setTwitchAccountUrl] = useState(
    ENV.hosts.accounts + '/platform/twitch/login'
  );

  // setup event hooks when our component mounts
  useEffect(() => {
    const platformUpdatedHandle = subscribeAccountEvent(
      'account_platforms_updated',
      (ev) => {
        setPlatformData({
          discord:
            typeof ev.detail['discord'] !== 'undefined'
              ? ev.detail['discord'].display_name
              : null,
          twitch:
            typeof ev.detail['twitch'] !== 'undefined'
              ? ev.detail['twitch'].display_name
              : null,
          bungie:
            typeof ev.detail['bungie'] !== 'undefined'
              ? (ev.detail['bungie'] as AccountPlatformBungie).unique_name
              : null,
        });

        console.log(ev.detail['bungie']);

        const baseBungieUrl = ev.detail['discord']
          ? ENV.hosts.accounts + '/platform/bungie/unlink'
          : ENV.hosts.accounts + '/platform/bungie/login';

        setBungieAccountUrl(
          baseBungieUrl +
            '?redirect=' +
            encodeURIComponent(window.location.href)
        );

        const baseTwitchUrl = ev.detail['twitch']
          ? ENV.hosts.accounts + '/platform/twitch/unlink'
          : ENV.hosts.accounts + '/platform/twitch/login';
        setTwitchAccountUrl(
          baseTwitchUrl +
            '?redirect=' +
            encodeURIComponent(window.location.href)
        );
      }
    );
    // subscribeAccountEvent('account_logout', eventAccountLogout);

    return () => {
      unsubscribeAccountEvent(
        'account_platforms_updated',
        platformUpdatedHandle
      );
      //unsubscribeAccountEvent('account_logout', eventAccountLogout);
    };
  }, []);

  return (
    <OffCanvas>
      <Head>
        <title>Profile | Level Crush</title>
      </Head>
      <SiteHeader forceStickyStyle={true} />
      <main>
        <Container className="flex flex-wrap top-[4.5rem] relative">
          <LoginGuard>
            <div className="profile-section w-full">
              <H3 id="linkedAccounts">Linked Accounts</H3>

              <GoosePost>
                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque
                iaculis ullamcorper euismod. Suspendisse eu volutpat sem.
                Pellentesque aliquam, ante in semper aliquam, sapien urna tempor
                mauris, interdum lacinia risus urna a ex. Nullam bibendum erat
                quis mauris consequat, quis gravida justo interdum. Mauris eros
                neque, mattis vel consectetur nec, auctor sed urna. Aliquam urna
                dui, accumsan vel facilisis nec, bibendum nec est. Nam eget nunc
                turpis. Suspendisse potenti. Fusce in tellus commodo, hendrerit
                arcu eget, auctor sem.
              </GoosePost>

              <div className="flex flex-wrap gap-4 justify-between my-8">
                <div className="basis-full">
                  <H3>Discord</H3>
                  <hr />
                  <H4 className="my-4">
                    {platformData.discord || 'Not Linked'}
                  </H4>
                </div>
                <div className="basis-full md:basis-[40%] my-4 md:my-0">
                  <H3>Bungie</H3>
                  <hr />
                  <H4 className="my-4">
                    {platformData.bungie || 'Not Linked'}
                  </H4>
                  <HyperlinkButton
                    intention={platformData.bungie ? 'attention' : 'normal'}
                    href={bungieAccountUrl}
                  >
                    {platformData.bungie ? 'Unlink' : 'Link'}
                  </HyperlinkButton>
                </div>
                <div className="basis-full md:basis-[40%] my-4 md:my-0">
                  <H3>Twitch</H3>
                  <hr />
                  <H4 className="my-4">
                    {platformData.twitch || 'Not Linked'}
                  </H4>
                  <HyperlinkButton
                    intention={platformData.twitch ? 'attention' : 'normal'}
                    href={twitchAccountUrl}
                  >
                    {platformData.twitch ? 'Unlink' : 'Link'}
                  </HyperlinkButton>
                </div>
              </div>
            </div>
          </LoginGuard>
        </Container>
      </main>
    </OffCanvas>
  );
};

export default ProfilePage;
