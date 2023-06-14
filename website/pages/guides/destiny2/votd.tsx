import React from 'react';
import Hero from '@website/components/hero';
import SiteHeader from '@website/components/site_header';
import {
  TableOfContents,
  TableOfContentsNavigationItem,
} from '@website/components/table_of_contents';
import Head from 'next/head';
import Container from '@website/components/elements/container';
import { H2 } from '@website/components/elements/headings';
import { GoogleDoc, GoogleDocAssetMap } from '@website/core/google_doc';
import { docs_v1 } from 'googleapis';
import GoogleDocDisplay from '@website/components/google_doc_display';
import OffCanvas from '@website/components/offcanvas';
import ENV from '@website/core/env';
import { GetStaticProps } from 'next';

export interface GuideVOTDPageProps {
  navTree: TableOfContentsNavigationItem[];
  doc: docs_v1.Schema$Document;
  assetMap: GoogleDocAssetMap;
}

export const getStaticProps: GetStaticProps<GuideVOTDPageProps> = async () => {
  console.log('Grabbing Google Doc', Date.now() / 1000);
  const google_doc = await GoogleDoc('raidguide_votd');

  console.log('Pulling google doc', Date.now() / 1000);
  await google_doc.pull();

  const docSchema = google_doc.data();
  if (docSchema === null) {
    return {
      notFound: true,
    };
  }

  console.log('Generating outline', Date.now() / 1000);
  const googleDocOutline = await google_doc.outline();

  // build a 2 level deep nav tree
  console.log('Building Navigation Tree');
  const navTree: TableOfContentsNavigationItem[] = [];
  for (let i = 0; i < googleDocOutline.entries.length; i++) {
    const level1Entry = googleDocOutline.entries[i];
    const subnavigation: TableOfContentsNavigationItem[] = [];
    for (let j = 0; j < level1Entry.entries.length; j++) {
      const level2Entry = level1Entry.entries[j];
      subnavigation.push({
        text: level2Entry.title,
        url: '#' + level2Entry.heading,
        subnavigation: [],
      });
    }
    navTree.push({
      text: level1Entry.title,
      url: '#' + level1Entry.heading,
      subnavigation: subnavigation,
    });
  }

  console.log('Generating Asset Map', Date.now() / 1000);
  const assetMap = await google_doc.assets();
  // trim some unneeded items
  // nextjs (at time of testing and writing) will store a version of this locally in the page response to hydrate
  // trim to only what we **absolutely** need to hydrate and remount on the client side
  // also this is where we should remove any sensitive information (like documentid)
  // currently for our use case, we only need to send back down the body element of the docSchema and inlineObjects to properly hydrate
  delete docSchema.documentId;
  delete docSchema.footers;
  delete docSchema.footnotes;
  delete docSchema.lists;
  delete docSchema.namedRanges;
  delete docSchema.namedStyles;
  delete docSchema.revisionId;
  delete docSchema.suggestionsViewMode;
  delete docSchema.documentStyle;
  delete docSchema.headers;
  delete docSchema.positionedObjects;
  delete docSchema.suggestedNamedStylesChanges;
  delete docSchema.suggestedDocumentStyleChanges;

  return {
    revalidate: 3600, // 1 hour
    props: {
      navTree: navTree,
      doc: docSchema as docs_v1.Schema$Document,
      assetMap: assetMap || {},
    },
  };
};

export const GuideVOTDPage = (props: GuideVOTDPageProps) => (
  <OffCanvas>
    <Head>
      <title>VOTD Raid Guide | Level Crush</title>
    </Head>
    <SiteHeader />
    <main>
      <Hero
        className="min-h-[35rem]"
        backgroundUrl={ENV.hosts.assets + '/images/VOTDHero.jpg'}
      >
        <Container>
          <H2 className="drop-shadow text-center">
            <span className="pb-2 block">Vow of the Disciple</span>
            <div className="border-b-yellow-400 border-b-2"></div>
            <span className="text-3xl pt-2 block">
              Raid Guide / Walkthrough
            </span>
          </H2>
        </Container>
      </Hero>
      <div className="container mx-auto flex flex-wrap justify-between relative top-0 guide pt-0 pb-8 lg:pt-8">
        <TableOfContents
          key="tableOfContents"
          navTree={props.navTree}
        ></TableOfContents>
        <GoogleDocDisplay
          key="googleDocDisplay"
          doc={props.doc}
          assetMap={props.assetMap}
          feed={'raidguide_votd'}
        ></GoogleDocDisplay>
      </div>
    </main>
  </OffCanvas>
);

export default GuideVOTDPage;
