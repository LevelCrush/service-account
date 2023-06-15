import Head from 'next/head';
import React, { useEffect, useState } from 'react';
import { SiteHeader } from '@website/components/site_header';
import Container from '@website/components/elements/container';
import { H3, H4, H5 } from '@website/components/elements/headings';
import OffCanvas from '@website/components/offcanvas';
import LoginGuard from '@website/components/login_guard';
import {
  subscribeAccountEvent,
  unsubscribeAccountEvent,
} from '@website/components/account_observer';
import { HyperlinkButton } from '@website/components/elements/button';
import ENV from '@website/core/env';
import GoosePost from '@website/components/goose_post';
import { AccountPlatformBungie } from '@website/core/api_responses';

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
        <Container className="top-[4.5rem] relative">
          <LoginGuard>
            <div className="profile-section w-full">
              <H3 id="linkedAccounts">Linked Accounts</H3>
              <p className="max-w-[45rem]">
                Level Crush will only request your platform user names and
                retain the minimal amount of information to link your accounts
                together. No sensitive information is stored in our systems.
                <br />
                <br />
                If you are worried about your privacy you are welcome to audit
                our code yourself.
                <HyperlinkButton
                  className="md:max-w-[10rem] mt-2"
                  href="https://github.com/LevelCrush/levelcrush"
                  target="_blank"
                  intention={'normal'}
                >
                  Visit Github
                </HyperlinkButton>
              </p>

              <div className="mt-8 flex flex-wrap gap-4 justify-start">
                <H5 className="flex-auto w-full">Bungie Privacy Links</H5>
                <HyperlinkButton
                  className="flex-auto md:max-w-[20rem] mt-2"
                  href="https://www.bungie.net/7/en/User/Account/Privacy"
                  target="_blank"
                  intention={'normal'}
                >
                  Bungie Profile Privacy Settings
                </HyperlinkButton>
                <HyperlinkButton
                  className="flex-auto md:max-w-[20rem] mt-2"
                  href="https://help.bungie.net/hc/en-us/articles/360048721292-Account-Security-Guide"
                  target="_blank"
                  intention={'normal'}
                >
                  Bungie Privacy Guide
                </HyperlinkButton>
              </div>

              <div className="mt-8">
                <H5>Twitch Privacy Links</H5>
                <HyperlinkButton
                  className="flex-auto w-full md:max-w-[20rem] mt-2"
                  href="https://www.twitch.tv/p/en/legal/privacy-choices/"
                  target="_blank"
                  intention={'normal'}
                >
                  Twitch Privacy Guide
                </HyperlinkButton>
              </div>

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
          <GoosePost>What are the odds of even seeing this message?</GoosePost>
        </Container>
      </main>
    </OffCanvas>
  );
};

export default ProfilePage;
