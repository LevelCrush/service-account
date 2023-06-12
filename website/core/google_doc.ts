import ENV from './env';
import * as fs from 'fs';
import * as path from 'path';
import { docs_v1 } from 'googleapis';

export interface GoogleDocAsset {
  url: string;
  id: string;
  meta: string;
  extension: string;
}

export interface GoogleDocAssetMap {
  [key: string]: GoogleDocAsset;
}

export interface GoogleDocRawAsset {
  url: string;
  id: string;
  meta: string;
  extension: string;
}

export interface GoogleDocRawAssetMap {
  [key: string]: GoogleDocAsset;
}

export interface GoogleDocOutlineEntry {
  title: string; // comes from the text value of the heading
  heading: string; // the id that is associated with the heading
  entries: GoogleDocOutlineEntry[];
}

export interface GoogleDocOutline extends GoogleDocOutlineEntry {
  document: string;
}

/**
 * Goes through all paragraph elements and gets the text associated with each elements and joins into a single string
 * @param elements
 */
function parse_text(elements: docs_v1.Schema$ParagraphElement[]) {
  const data = [] as string[];
  for (const element of elements) {
    if (
      element.textRun &&
      element.textRun.content &&
      element.textRun.content.trim().length > 0
    ) {
      data.push(element.textRun.content.trim());
    }
  }
  return data.join(' ');
}

/**
 * Loop through the google doc data and generate an outline
 * @param target_feed
 * @param content
 */
function generate_outline(
  target_feed: string,
  content: docs_v1.Schema$StructuralElement[]
) {
  const outline = {
    document: target_feed,
    title: '',
    heading: '',
    entries: [],
  } as GoogleDocOutline;

  // get outline title
  for (const content_item of content) {
    if (
      content_item.paragraph &&
      content_item.paragraph.paragraphStyle &&
      content_item.paragraph.paragraphStyle.headingId &&
      content_item.paragraph.paragraphStyle.namedStyleType &&
      content_item.paragraph.paragraphStyle.namedStyleType === 'HEADING_1'
    ) {
      outline.title = parse_text(content_item.paragraph.elements || []);
      outline.heading = content_item.paragraph.paragraphStyle.headingId;
      break;
    }
  }

  if (outline.title.length > 0) {
    const heading_matches = ['HEADING_2', 'HEADING_3'];
    let active_heading2 = null as GoogleDocOutlineEntry | null;
    let active_heading3 = null as GoogleDocOutlineEntry | null;

    for (const content_item of content) {
      if (
        content_item.paragraph &&
        content_item.paragraph.paragraphStyle &&
        content_item.paragraph.paragraphStyle.headingId &&
        content_item.paragraph.paragraphStyle.namedStyleType &&
        heading_matches.includes(
          content_item.paragraph.paragraphStyle.namedStyleType
        )
      ) {
        const title = parse_text(content_item.paragraph.elements || []);
        const heading = content_item.paragraph.paragraphStyle.headingId;
        switch (content_item.paragraph.paragraphStyle.namedStyleType) {
          case 'HEADING_2':
            if (active_heading3 !== null && active_heading2 !== null) {
              active_heading2.entries.push(active_heading3);
              active_heading3 = null;
            }

            if (active_heading2 !== null) {
              outline.entries.push(active_heading2);
              active_heading2 = null;
            }

            if (title.length > 0) {
              active_heading2 = {
                title: title,
                heading: heading,
                entries: [],
              };
            }
            break;

          case 'HEADING_3':
            if (active_heading2 !== null && active_heading3 !== null) {
              active_heading2.entries.push(active_heading3);
              active_heading3 = null;
            }
            if (title.length > 0) {
              active_heading3 = {
                title: title,
                heading: heading,
                entries: [],
              };
            }
            break;
        }
      }
    }

    // finish and closse out the outline
    if (active_heading2 !== null && active_heading3 !== null) {
      active_heading2.entries.push(active_heading3);
      active_heading3 = null;
    }

    if (active_heading2 !== null) {
      outline.entries.push(active_heading2);
      active_heading2 = null;
    }
  }
  return outline;
}

async function pull_feed<T>(target_feed: string, fresh = false) {
  const cache_dir = './feed_cache';
  const cache_file_path = path.join(cache_dir, target_feed + '.json');

  let result = null as T | null;
  // check cache first
  if (!fresh) {
    try {
      console.log('Checking for cache:', cache_file_path);
      await fs.promises.access(cache_file_path);

      console.log('Parsing cache results:', target_feed);
      result = JSON.parse(
        await fs.promises.readFile(cache_file_path, { encoding: 'utf-8' })
      ) as T;
      return result;
    } catch {
      result = null;
    }
  }

  // no cached doc found, request a new one via the feed
  if (result === null) {
    console.log(ENV.hosts.feed);
    const endpoint = ENV.hosts.feed + '/' + encodeURIComponent(target_feed);
    console.log(endpoint, ENV.feed.public_key);
    const response = await fetch(endpoint, {
      method: 'GET',
      headers: {
        'Public-Key': ENV.feed.public_key,
      },
    });
    try {
      console.log('Parsing feed', target_feed);
      const result = (await response.json()) as T;

      // store off result
      // it's a little funny to restring our result but fetch only allows the body to be consumed once (to my understanding currently)
      console.log('Storing feed', target_feed);
      await fs.promises.mkdir(cache_dir, { recursive: true });
      await fs.promises.writeFile(cache_file_path, JSON.stringify(result), {
        encoding: 'utf-8',
      });

      return result;
    } catch (err) {
      console.log('Error Parsing or storing cached feed: ', target_feed, err);
      return null;
    }
  } else {
    return null;
  }
}

/**
 * Fetches a google doc representation from our own database side
 * @param target_feed
 * @constructor
 */
export const GoogleDoc = async (target_feed: string) => {
  let doc = null as docs_v1.Schema$Document | null;
  let assets = null as GoogleDocAssetMap | null;
  return {
    pull: async (fresh = false) => {
      [doc, assets] = await Promise.all([
        pull_feed<docs_v1.Schema$Document>(target_feed + '_raw', fresh),
        pull_feed<GoogleDocAssetMap>(target_feed + '_assets', fresh),
      ]);
    },
    outline: () => {
      return doc !== null && doc.body
        ? generate_outline(target_feed, doc.body.content || [])
        : ({
            document: '',
            title: '',
            heading: '',
            entries: [],
          } as GoogleDocOutline);
    },
    data: () => doc,
    assets: () => assets,
  };
};

export default GoogleDoc;
