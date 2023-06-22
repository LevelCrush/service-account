import Head from 'next/head';
import React from 'react';
import Hero from '@website/components/hero';
import { SiteHeader } from '@website/components/site_header';
import OffCanvas from '@website/components/offcanvas';
import { GetStaticProps } from 'next';
import ENV from '@website/core/env';
import { H3 } from '@website/components/elements/headings';
import { HyperlinkButton } from '@website/components/elements/button';
import Container from '@website/components/elements/container';
import { DestinyClanInformation } from '@website/core/api_responses';

export interface NetworkClanResponse {
  success: boolean;
  response: DestinyClanInformation[];
  errors: unknown[]; //  not possible for this response to have errors, so just ignore it for now
}

export interface NetworkClanDirectoryPageProps {
  clans: DestinyClanInformation[];
}

export const getStaticProps: GetStaticProps<
  NetworkClanDirectoryPageProps
> = async () => {
  //

  const destiny_api = ENV.hosts.destiny;
  const response = await fetch(destiny_api + '/network');

  const network_clan = response.ok
    ? ((await response.json()) as NetworkClanResponse)
    : null;

  return {
    props: {
      clans: network_clan ? network_clan.response || [] : [],
    },
  };
};

export const ClanDirectoryPage = (props: NetworkClanDirectoryPageProps) => (
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
        {props.clans.map((clan, clanIndex) => (
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
                    'https://www.bungie.net/en/ClanV2?groupid=' + clan.group_id
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

export default ClanDirectoryPage;
