import Head from 'next/head';
import React from 'react';
import Hero from '@website/components/hero';
import { SiteHeader } from '@website/components/site_header';
import OffCanvas from '@website/components/offcanvas';
import { GetServerSideProps } from 'next';
import ENV from '@website/core/env';
import { H2, H3 } from '@website/components/elements/headings';
import { HyperlinkButton } from '@website/components/elements/button';
import Container from '@website/components/elements/container';
import {
  DestinyClanInformation,
  DestinyClanResponse,
} from '@website/core/api_responses';

export interface ClanPageProps {
  clan: DestinyClanInformation;
}

export const getServerSideProps: GetServerSideProps<ClanPageProps> = async (
  context
) => {
  //

  const slug = context.query.slug;

  const destiny_api = ENV.hosts.destiny;
  const response = await fetch(destiny_api + '/clan/' + slug, {
    next: {
      revalidate: 3600, // one hour cache time
    },
  });

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
      <Hero className="min-h-[40rem] overflow-hidden top-0 relative">
        <Container>
          <H2 className="drop-shadow text-center">{props.clan.name}</H2>
        </Container>
      </Hero>
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
