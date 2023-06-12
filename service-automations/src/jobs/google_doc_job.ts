import Job from './job';
import { docs_v1, google } from 'googleapis';
import * as fs from 'fs';
import * as path from 'path';
import * as mime from 'mime-types';
import { Magic, MAGIC_MIME_TYPE } from 'mmmagic';
import { finished } from 'stream/promises';
import { Readable } from 'stream';
import { ReadableStream } from 'stream/web';

export enum GoogleDocMapping {
    DocumentTitle = 'HEADING_1',
    Chapter = 'HEADING_2',
    Section = 'HEADING_3',
}

export interface GoogleDocAsset {
    url: string;
    id: string;
    meta: string;
    extension: string;
}

export interface GoogleDocSection {
    title: string;
    id: string;
    content: string;
}

export interface GoogleDocChapter {
    title: string;
    id: string;
    sections: { [key: string]: GoogleDocSection };
}

export interface GoogleDoc {
    title: string;
    id: string;
    chapters: { [key: string]: GoogleDocChapter };
    assets: { [key: string]: GoogleDocAsset };
}

async function download_file(url: string, output: string) {
    const response = await fetch(url);
    const body = Readable.fromWeb(response.body as ReadableStream);
    const stream = fs.createWriteStream(output, { flags: 'wx' });
    await finished(body.pipe(stream));
}

async function download_all_assets(assets: GoogleDoc['assets'], feed_name: string) {
    const updated_assets = assets;

    // make sure asset folder is around
    const asset_path = path.join(process.env['FOLDER_ASSETS'] || 'placeholderAssets', 'doc_' + feed_name);

    // remove all assets in the folder
    await fs.promises.rm(asset_path, { recursive: true, force: true });

    // create directory path
    await fs.promises.mkdir(asset_path, {
        recursive: true,
    });

    const magic = new Magic(MAGIC_MIME_TYPE);
    for (const asset_id in assets) {
        const asset = assets[asset_id];
        const url = asset.url;
        const output_path = path.join(asset_path, asset.id);
        console.log('Downloading asset');
        await download_file(url, output_path);

        const mime_type = await ((): Promise<string> => {
            return new Promise((resolve) => {
                magic.detectFile(output_path, (err, result) => {
                    if (err) {
                        console.log('Mime Type Error');
                        resolve('');
                    } else {
                        if (Array.isArray(result) && result.length > 0) {
                            resolve(result[0].trim());
                        } else {
                            resolve(result as string);
                        }
                    }
                });
            });
        })();

        console.log('Determining mime type and renaming file appropriately');
        // this is wrong technically. But we hope it never breaks since what we are feeding is normally so consistent and common to identify
        const extension = mime.extension(mime_type) as string;
        const new_path = output_path + '.' + extension.trim();
        await fs.promises.rename(output_path, new_path);

        // in our object that is tracking our asset information, update it
        updated_assets[asset_id].extension = extension;
    }

    return updated_assets;
}

/**
 * Parse a google doc paragraph element and return formatted with that contains markdown to identify text style
 * @param element
 */
function parse_text_with_styles(element: docs_v1.Schema$ParagraphElement) {
    if (!element.textRun) {
        return '';
    }

    let text_block = (element.textRun && element.textRun.content) || '';
    if (element.textRun.textStyle) {
        const is_bold = element.textRun.textStyle.bold === true;
        const is_italic = element.textRun.textStyle.italic === true;
        const is_strikethrough = element.textRun.textStyle.strikethrough === true;
        const is_underline = element.textRun.textStyle.underline === true;

        if (text_block.trim().length > 0) {
            text_block = text_block.trim();
            let did_mod = false;
            if (is_italic) {
                text_block = '*' + text_block + '*';
                did_mod = true;
            }

            if (is_bold) {
                text_block = '**' + text_block + '**';
                did_mod = true;
            }

            if (is_underline) {
                text_block = '__' + text_block + '__';
                did_mod = true;
            }

            if (is_strikethrough) {
                text_block = '~~' + text_block + '~~';
                did_mod = true;
            }

            if (did_mod) {
                text_block = ' ' + text_block + ' ';
            }
        }
    }
    return text_block;
}

/**
 * Iterates through the provided schema elements and gets the text of said elements
 * @param elements
 */
function parse_text(elements: docs_v1.Schema$ParagraphElement[]) {
    const data = [] as string[];
    for (const element of elements) {
        if (element.textRun && element.textRun.content && element.textRun.content.trim().length > 0) {
            data.push(element.textRun.content.trim());
        }
    }
    return data.join(' ');
}

/**
 * P
 * @param content
 * @param inline_objects
 * @param doc_id
 */
function parse_google_response(
    content: docs_v1.Schema$StructuralElement[],
    inline_objects: { [p: string]: docs_v1.Schema$InlineObject },
    doc_id: string,
) {
    const doc = {
        title: '',
        id: doc_id,
        chapters: {},
        assets: {},
    } as GoogleDoc;

    // grab document title first
    for (const content_item of content) {
        if (
            content_item.paragraph &&
            content_item.paragraph.paragraphStyle &&
            content_item.paragraph.paragraphStyle.headingId &&
            content_item.paragraph.paragraphStyle.namedStyleType &&
            content_item.paragraph.paragraphStyle.namedStyleType === GoogleDocMapping.DocumentTitle
        ) {
            doc.title = parse_text(content_item.paragraph.elements || []);
            console.log('Document Title Found', doc.title);
            break;
        }
    }

    // now actually parse
    let active_chapter = null as GoogleDocChapter | null;
    let active_section = null as GoogleDocSection | null;
    let active_content = [] as string[];

    for (const content_item of content) {
        if (
            content_item.paragraph &&
            content_item.paragraph.paragraphStyle &&
            content_item.paragraph.paragraphStyle.namedStyleType &&
            content_item.paragraph.paragraphStyle.namedStyleType === GoogleDocMapping.Chapter
        ) {
            // this is the start of a new chapter, close out and store the previous chapter before starting new
            if (active_chapter !== null) {
                console.log('storing old chapter', active_chapter.title);

                // make sure our section is closed
                if (active_section !== null) {
                    active_section.content = active_content.join(' ');
                    active_chapter.sections[active_section.id] = active_section;
                    active_section = null;
                    active_content = [];
                }

                doc.chapters[active_chapter.id] = active_chapter;
            }

            // setup new active chapter
            active_chapter = {
                id:
                    content_item.paragraph.paragraphStyle.headingId ||
                    'chapter_' + (Object.keys(doc.chapters).length + 1),
                title: parse_text(content_item.paragraph.elements ? content_item.paragraph.elements : []),
                sections: {},
            };
        } else if (
            active_chapter &&
            content_item.paragraph &&
            content_item.paragraph.paragraphStyle &&
            content_item.paragraph.paragraphStyle.namedStyleType &&
            content_item.paragraph.paragraphStyle.namedStyleType === GoogleDocMapping.Section
        ) {
            if (active_section !== null) {
                // close out previous section and store into our active chapter
                active_section.content = active_content.join('');
                active_chapter.sections[active_section.id] = active_section;
                active_content = [];
            }

            // now start a new section
            active_section = {
                id:
                    content_item.paragraph.paragraphStyle.headingId ||
                    active_chapter.id + '_section_' + (Object.keys(active_chapter.sections).length + 1),
                title: parse_text(content_item.paragraph.elements || []),
                content: '',
            };
        } else if (active_chapter && active_section && content_item.paragraph && content_item.paragraph.elements) {
            for (const element of content_item.paragraph.elements) {
                if (element.inlineObjectElement && element.inlineObjectElement.inlineObjectId) {
                    active_content.push('[[{{ ' + element.inlineObjectElement.inlineObjectId + ' }}]]');
                } else if (element.textRun && element.textRun.content) {
                    active_content.push(parse_text_with_styles(element));
                }
            }
        }
    }

    console.log('Final commit');
    if (active_chapter && active_section) {
        //flatten
        active_section.content = active_content.join('');
        active_chapter.sections[active_section.id] = active_section;
        console.log('Final section', active_section.title);
    }

    if (active_chapter) {
        console.log('Final chapter', active_chapter.title);
        doc.chapters[active_chapter.id] = active_chapter;
    }

    console.log('Parsing inline objects');
    for (const asset_id in inline_objects) {
        const inline_object = inline_objects[asset_id];
        if (inline_object.inlineObjectProperties && inline_object.inlineObjectProperties.embeddedObject) {
            const id = asset_id;
            const meta = JSON.stringify(inline_object.inlineObjectProperties);
            const url =
                inline_object.inlineObjectProperties.embeddedObject.imageProperties &&
                inline_object.inlineObjectProperties.embeddedObject.imageProperties.contentUri
                    ? inline_object.inlineObjectProperties.embeddedObject.imageProperties.contentUri
                    : '';

            const extension = '';

            doc.assets[asset_id] = {
                id,
                meta,
                url,
                extension,
            };
        }
    }

    return doc;
}

export const GoogleDocJob = async (google_doc_id: string, target_feed: string) => {
    // construct google authenticator
    const auth = new google.auth.GoogleAuth({
        scopes: [
            'https://www.googleapis.com/auth/documents',
            'https://www.googleapis.com/auth/documents.readonly',
            'https://www.googleapis.com/auth/drive',
            'https://www.googleapis.com/auth/drive.file',
            'https://www.googleapis.com/auth/drive.readonly',
        ],
        keyFile: process.env['KEY_FILE_GOOGLE'],
    });

    const auth_client = await auth.getClient();
    const docs = google.docs({
        version: 'v1',
        auth: auth_client as unknown as string, // typescript complains, so force cast like this.
    });

    const run = async () => {
        // todo!

        const response = await docs.documents.get({
            documentId: google_doc_id,
        });

        // copy the raw response into our feed in case we ever need to inspect it on our own from another source
        try {
            console.log('Saving feed in db');
            const raw_response = JSON.stringify(response.data);
            await fetch((process.env['HOST_API_FEED'] || '') + '/' + encodeURIComponent(target_feed + '_raw'), {
                method: 'POST',
                cache: 'no-store',
                body: raw_response,
                headers: {
                    'Content-Type': 'application/json',
                    'Public-Key': process.env['FEED_PUBLIC_KEY'] || '',
                    'Private-Key': process.env['FEED_PRIVATE_KEY'] || '',
                },
            });
        } catch (err) {
            console.log('Google Doc Cache Error:', err);
        }

        // parse the Google response into our own local google doc object
        const parsed_doc = parse_google_response(
            response.data.body && response.data.body.content ? response.data.body.content : [],
            response.data.inlineObjects || {},
            google_doc_id,
        );

        console.log('Downloading assets and updating asset map');
        parsed_doc.assets = await download_all_assets(parsed_doc.assets, target_feed);

        console.log('Sending data + seperate asset information to feeds');
        const feed_endpoint = process.env['HOST_API_FEED'] || '';
        await Promise.allSettled([
            fetch(feed_endpoint + '/' + encodeURIComponent(target_feed), {
                method: 'POST',
                cache: 'no-store',
                body: JSON.stringify(parsed_doc),
                headers: {
                    'Content-Type': 'application/json',
                    'Public-Key': process.env['FEED_PUBLIC_KEY'] || '',
                    'Private-Key': process.env['FEED_PRIVATE_KEY'] || '',
                },
            }),
            fetch(feed_endpoint + '/' + encodeURIComponent(target_feed) + '_assets', {
                method: 'POST',
                cache: 'no-store',
                body: JSON.stringify(parsed_doc.assets),
                headers: {
                    'Content-Type': 'application/json',
                    'Public-Key': process.env['FEED_PUBLIC_KEY'] || '',
                    'Private-Key': process.env['FEED_PRIVATE_KEY'] || '',
                },
            }),
        ]);

        console.log('Operation Done!');
    };

    const cleanup = async () => {
        // todo!
    };

    return { run, cleanup } as Job;
};

export default GoogleDocJob;
