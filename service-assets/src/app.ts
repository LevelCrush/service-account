import * as server from './server';
import * as dotenv from 'dotenv';
import * as express from 'express';

// import env settings into the process env
dotenv.config();

async function main(): Promise<void> {
    // is debug
    const IS_DEBUG = process.env['SERVER_DEBUG'] === '1';

    // create our server and pass in a starting global state
    const api = server.create(
        {
            port: parseInt(process.env['SERVER_PORT'] || '3001'),
            body_limit: process.env['SERVER_INCOMING_BODY_LIMIT'],
            origins: IS_DEBUG
                ? ['*']
                : [
                      'https://levelcrush.local',
                      'https://preview.levelcrush.com',
                      'https://levelcrush.com',
                      'https://www.levelcrush.com',
                  ],
        },
        {
            debug: IS_DEBUG,
        },
    );

    // setup static file hosting
    const asset_path = process.env['FOLDER_ASSETS'] || '';
    api.app.use('/', express.static(asset_path));

    // start and run our server
    await server.start(api);
}

main()
    .then(() => console.log('Done'))
    .catch((err) => console.log('An error occurred', err));
