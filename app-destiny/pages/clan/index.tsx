import Head from 'next/head';
import React, { useEffect, useState } from 'react';
import Hero from '@website/components/hero';
import { SiteHeader } from '@website/components/site_header';
import OffCanvas from '@website/components/offcanvas';
import { H3 } from '@website/components/elements/headings';
import { HyperlinkButton } from '@website/components/elements/button';
import Container from '@website/components/elements/container';
import { ClanInformation } from '@ipc/bindings';
import { getNetworkClans } from '@ipc/service-destiny';

export const ClanDirectoryPage = () => {
  const [clans, setClans] = useState([] as ClanInformation[]);

  useEffect(() => {
    getNetworkClans().then((clans) => {
      setClans(clans);
    });
  });

  return (
    <OffCanvas>
      <Head>
        <title>Network Clans | Level Crush</title>
      </Head>
      <SiteHeader />
      <main>
        <Hero
          className="min-h-[40rem] overflow-hidden top-0 relative"
          youtubeID="hgCfi27VmNQ"
        ></Hero>
        <Container className="md:flex md:justify-between md:flex-wrap">
          {clans.map((clan, clanIndex) => (
            <div
              className="network-clan w-full md:w-[40%] mt-0 mb-12"
              key={'network_clan_' + clanIndex}
            >
              <H3 className="text-yellow-400">
                {clan.name}{' '}
                <span className="text-sm text-white">({clan.motto})</span>
              </H3>
              <p>{clan.about}</p>
              <div className="w-full md:flex md:justify-between">
                <div className="w-full md:w-[45%] my-4">
                  <HyperlinkButton
                    href={'/clan/' + clan.slug + '/roster'}
                    intention={'normal'}
                  >
                    View Roster
                  </HyperlinkButton>
                </div>
                <div className="w-full md:w-[45%] my-4">
                  <HyperlinkButton
                    href={
                      'https://www.bungie.net/en/ClanV2?groupid=' +
                      clan.group_id
                    }
                    intention={'normal'}
                  >
                    Bungie Page
                  </HyperlinkButton>
                </div>
              </div>
            </div>
          ))}
        </Container>
      </main>
    </OffCanvas>
  );
};

export default ClanDirectoryPage;
