import Head from 'next/head';
import React, { useEffect, useState } from 'react';
import Hero from '@website/components/hero';
import { SiteHeader } from '@website/components/site_header';
import OffCanvas from '@website/components/offcanvas';
import { H2, H3 } from '@website/components/elements/headings';
import { HyperlinkButton } from '@website/components/elements/button';
import Container from '@website/components/elements/container';
import { ClanInformation } from '@ipc/bindings';
import { getClanInfo } from '@ipc/service-destiny';
import { useRouter } from 'next/router';

export const ClanPage = () => {
  const [clan, setClan] = useState(null as ClanInformation | null);
  const router = useRouter();

  useEffect(() => {
    console.log('Router', router);
    const slug = (router.query.slug as string) || '';
    getClanInfo(slug as string)
      .then((clan_response) => {
        if (clan_response.response !== null) {
          setClan(clan_response.response);
        }
      })
      .catch((err) => {
        console.error(err);
      });
  });

  return (
    <OffCanvas>
      <Head>
        <title>{(clan ? clan.name : 'Clan') + ' | Level Crush'}</title>
      </Head>
      <SiteHeader />
      <main>
        <Hero className="min-h-[40rem] overflow-hidden top-0 relative">
          <Container>
            <H2 className="drop-shadow text-center">
              {clan ? clan.name : 'Clan'}
            </H2>
          </Container>
        </Hero>
        <Container className="md:flex md:justify-between md:flex-wrap">
          {clan !== null ? (
            <div className="network-clan w-full md:w-[40%] mt-0 mb-12">
              <H3 className="text-yellow-400">
                {clan.name}
                <span className="text-sm text-white">(clan.motto)</span>
              </H3>
              <p>clan.about</p>
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
          ) : (
            <></>
          )}
        </Container>
      </main>
    </OffCanvas>
  );
};

export default ClanPage;
