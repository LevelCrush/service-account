import * as dotenv from 'dotenv';
import router from './routes';
import * as server from './server';
import path from 'path';
import express from 'express';

// import env settings into the process env
dotenv.config();

async function main() {
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
    api.app.use('/robots.txt', express.static(path.join(asset_path, 'robots.txt')));
    api.app.use('/robot.txt', express.static(path.join(asset_path, 'robots.txt')));

    // for this application there is only one router
    // assign it to our server
    api.app.use('/jobs', router());

    // start and run our server
    await server.start(api);
}

main()
    .then(() => 'Completed Successfully')
    .catch((err) => console.log('An internal error has occurred', err));
