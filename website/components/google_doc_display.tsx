import { docs_v1 } from 'googleapis';
import React from 'react';
import { GoogleDocAssetMap } from '../core/google_doc';
import { H1, H2, H3, H4, H5, H6 } from './elements/headings';
import { Hyperlink } from './elements/hyperlink';
import ENV from '../core/env';

export interface GoogleDocDisplayProps {
  doc: docs_v1.Schema$Document;
  assetMap: GoogleDocAssetMap;
  feed: string;
  className?: string;
}

function renderBlock(
  element: docs_v1.Schema$ParagraphElement,
  elementIndex: number,
  structIndex: number,
  assetMap: GoogleDocAssetMap,
  feed: string
) {
  if (
    element.inlineObjectElement &&
    element.inlineObjectElement.inlineObjectId &&
    typeof assetMap[element.inlineObjectElement.inlineObjectId] !== 'undefined'
  ) {
    const asset = assetMap[element.inlineObjectElement.inlineObjectId];
    const asset_url =
      ENV.hosts.assets +
      '/documents/' +
      'doc_' +
      encodeURIComponent(feed) +
      '/' +
      asset.id +
      '.' +
      asset.extension;

    return (
      <img
        src={asset_url}
        key={structIndex + '_block_' + elementIndex + '_image'}
        loading={'lazy'}
        className="inline-block pr-4 my-8 mr-4 align-top w-auto h-auto max-w-full"
        alt="Image"
      />
    );
    // }
  } else {
    const textStyle =
      element.textRun && element.textRun.textStyle !== undefined
        ? element.textRun.textStyle
        : undefined;
    const isBold =
      textStyle && textStyle.bold !== undefined ? textStyle.bold : false;
    const isUnderline =
      textStyle && textStyle.underline !== undefined
        ? textStyle.underline
        : false;
    const isStrikeThrough =
      textStyle && textStyle.strikethrough !== undefined
        ? textStyle.strikethrough
        : false;
    const classList = [] as string[];
    if (isBold) {
      classList.push('font-bold');
    }
    if (isUnderline) {
      classList.push('underline');
    }
    if (isStrikeThrough) {
      classList.push('line-through');
    }

    const textUrl =
      textStyle && textStyle.link && textStyle.link.url
        ? textStyle.link.url
        : '';
    const isLink = textUrl.trim().length > 0 ? true : false;
    let videoID = '' as string;
    if (isLink) {
      if (textUrl.includes('youtu')) {
        // looking for matches on both //youtu.be and www.youtube.com or youtube.com
        const youtubeURL = new URL(textUrl);
        videoID = youtubeURL.href.includes('//youtu.be')
          ? (youtubeURL.pathname.split('/')[1] as string)
          : (youtubeURL.searchParams.get('v') as string);
      }
    }

    const textContent =
      element.textRun && element.textRun.content
        ? element.textRun.content
        : 'No Text';
    const elementKey =
      'struct_' + structIndex + '_block_' + elementIndex + '_normal';
    if (textContent === '\n') {
      return <br key={elementKey + '_br'} />;
    } else if (isLink) {
      return (
        <>
          <Hyperlink key={elementKey + '_hyperlink'} href={textUrl}>
            {textContent}
          </Hyperlink>
          {videoID && videoID.trim().length > 0 ? (
            <iframe
              key={elementKey + '_iframe'}
              loading="lazy"
              src={'https://www.youtube.com/embed/' + videoID + '?autoplay=0'}
              frameBorder="0"
              width="640"
              height="480"
              className="youtube-player"
            ></iframe>
          ) : (
            <></>
          )}
        </>
      );
    } else if (classList.length > 0) {
      return (
        <span key={elementKey + '_span'} className={classList.join(' ')}>
          {textContent}
        </span>
      );
    } else {
      return <>{textContent}</>;
    }
  }
}

function renderElement(
  structElement: docs_v1.Schema$StructuralElement,
  structIndex: number,
  assetMap: GoogleDocAssetMap,
  feed: string
) {
  if (structElement.paragraph && structElement.paragraph.elements) {
    const elements = structElement.paragraph.elements || [];

    if (
      elements.length === 1 &&
      elements[0].textRun &&
      elements[0].textRun.content === '\n'
    ) {
      return <br key={'struct_' + structIndex + '_br_only'} />;
    } else {
      return (
        <p key={'struct_' + structIndex + '_paragraph'}>
          {elements.map((element, elementIndex) =>
            renderBlock(element, elementIndex, structIndex, assetMap, feed)
          )}
        </p>
      );
    }
  } else {
    return <></>;
  }
}

function renderHeading(paragraph: docs_v1.Schema$Paragraph) {
  const elements = paragraph.elements || [];
  const textRuns: string[] = [];
  elements.forEach((textElement) => {
    const text =
      textElement.textRun && textElement.textRun.content
        ? textElement.textRun.content.trim()
        : '';

    if (text.length > 0) {
      textRuns.push(text);
    }
  });
  const title = textRuns.join(' ').trim();
  const id =
    paragraph.paragraphStyle && paragraph.paragraphStyle.headingId
      ? paragraph.paragraphStyle.headingId
      : '';
  if (
    title.length > 0 &&
    paragraph &&
    paragraph.paragraphStyle &&
    paragraph.paragraphStyle.headingId &&
    paragraph.paragraphStyle.namedStyleType
  ) {
    const headingKey = 'heading_' + id;
    switch (paragraph.paragraphStyle.namedStyleType) {
      case 'HEADING_1':
        return (
          <H1
            id={id}
            key={headingKey}
            minimalCSS={true}
            className="dark:text-yellow-400 text-black  text-5xl font-headline font-bold uppercase tracking-widest mt-8 md:mt-0"
          >
            {title}
          </H1>
        );
      case 'HEADING_2':
        return (
          <H2
            id={id}
            key={headingKey}
            minimalCSS={true}
            className="text-4xl dark:text-yellow-400 text-black font-headline font-bold uppercase tracking-widest mt-8 mb-4"
          >
            {title}
          </H2>
        );
      case 'HEADING_3':
        return (
          <H3
            id={id}
            key={headingKey}
            minimalCSS={true}
            className="text-xl font-sans font-bold uppercase mb-4"
          >
            {title}
          </H3>
        );
      case 'HEADING_4':
        return (
          <H4
            id={id}
            key={headingKey}
            minimalCSS={true}
            className="text-lg font-bold"
          >
            {title}
          </H4>
        );
      case 'HEADING_5':
        return (
          <H5 id={id} key={headingKey} minimalCSS={true} className="text-lg">
            {title}
          </H5>
        );
      case 'HEADING_6':
        return (
          <H6 id={id} key={headingKey} minimalCSS={true} className="text-base">
            {title}
          </H6>
        );
    }
  } else {
    return <></>;
  }
}

export const GoogleDocDisplay = (props: GoogleDocDisplayProps) => (
  <div
    className={
      'flex-[1_1_auto] lg:flex-[0_1_65%]  relative top-0 self-start guide-content mt-8 lg:mt-0 px-4' +
      ' ' +
      (props.className || '')
    }
  >
    {props.doc.body?.content?.map((structElement, structIndex) =>
      structElement.paragraph ? (
        structElement.paragraph.paragraphStyle &&
        structElement.paragraph.paragraphStyle &&
        structElement.paragraph.paragraphStyle.headingId &&
        structElement.paragraph.paragraphStyle.namedStyleType ? (
          renderHeading(structElement.paragraph)
        ) : (
          renderElement(structElement, structIndex, props.assetMap, props.feed)
        )
      ) : (
        <></>
      )
    )}
  </div>
);

export default GoogleDocDisplay;
