import Head from 'next/head';
import React, { cache } from 'react';
import Hero from '../../../components/hero';
import { SiteHeader } from '../../../components/site_header';
import OffCanvas from '../../../components/offcanvas';
import { GetServerSideProps } from 'next';
import ENV from '../../../core/env';
import { H3 } from '../../../components/elements/headings';
import Button, { HyperlinkButton } from '../../../components/elements/button';
import Container from '../../../components/elements/container';
import { useRouter } from 'next/router';
import {
  DestinyClanInformation,
  DestinyClanResponse,
  DestinyMemberInformation,
  AccountLinkedPlatformResultMap,
} from '../../../core/api_responses';

export interface ClanPageProps {
  clan: DestinyClanInformation;

}

export const getServerSideProps: GetServerSideProps<ClanPageProps> = async (
  context
) => {
  //

  const slug = context.query.slug;

  const destiny_api = ENV.hosts.destiny;
  const response = await fetch(destiny_api + '/clan/' + slug);

  const clan_response = response.ok
    ? ((await response.json()) as DestinyClanResponse)
    : null;

  if (clan_response === null || clan_response.response === null) {
    return {
      notFound: true,
    };
  } else {
    return {
      props: {
        clan: clan_response.response,
      },
    };
  }
};

export const ClanPage = (props: ClanPageProps) => (
  <OffCanvas>
    <Head>
      <title>{props.clan.name + ' | Level Crush'}</title>
    </Head>
    <SiteHeader />
    <main>
      <Hero className="min-h-[40rem] overflow-hidden top-0 relative"></Hero>
      <Container className="md:flex md:justify-between md:flex-wrap">
        {props.clan !== null ? (
          <div className="network-clan w-full md:w-[40%] mt-0 mb-12">
            <H3 className="text-yellow-400">
              {props.clan.name}
              <span className="text-sm text-white">({props.clan.motto})</span>
            </H3>
            <p>{props.clan.about}</p>
            <div className="w-full md:flex md:justify-between">
              <div className="w-full md:w-[45%] my-4">
                <HyperlinkButton
                  href={'/clan/' + props.clan.slug + '/roster'}
                  intention={'normal'}
                >
                  View Roster
                </HyperlinkButton>
              </div>
              <div className="w-full md:w-[45%] my-4">
                <HyperlinkButton
                  href={
                    'https://www.bungie.net/en/ClanV2?groupid=' +
                    props.clan.group_id
                  }
                  intention={'normal'}
                >
                  Bungie Page
                </HyperlinkButton>
              </div>
            </div>
          </div>
        ) : (
          <></>
        )}
      </Container>
    </main>
  </OffCanvas>
);

export default ClanPage;