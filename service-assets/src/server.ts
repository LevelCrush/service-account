import express from 'express';

/**
 * Expected parameters when configuring our express web server
 */
export interface ServerConfiguration {
    port: number;
    origins: string[];
    body_limit: string | '1mb' | '5mb';
}

/**
 * Default options used to spread our inputed configuration
 */
const DEFAULT_SERVER_CONFIGURATION = {
    port: 3001,
    origins: ['*'],
    body_limit: '5mb',
} as ServerConfiguration;

/**
 * An expected request with every express request
 */
export interface ServerRequest<T> extends express.Request {
    globals: T;
}

/**
 * Interface describing the server object
 */
export interface Server {
    app: express.Express;
    config: ServerConfiguration;
}

/**
 * Creates a server object with a predefined setup that we require
 * @param server_config
 * @param globals
 */
export function create<Globals>(server_config: Partial<ServerConfiguration>, globals: Globals) {
    const app = express();
    const config = { ...DEFAULT_SERVER_CONFIGURATION, ...server_config };

    //setup globals to be used throughout our server calls
    //this will be mainly used for passing around database connection but can be used for other things
    app.use((req, res, next) => {
        (req as ServerRequest<Globals>).globals = globals;
        next();
    });

    // setup body parsing
    app.use(express.json({ limit: config.body_limit }));
    app.use(express.urlencoded({ extended: true, limit: config.body_limit }));

    // ping route
    app.use('/ping', (req, res) => res.sendStatus(200));

    // serve empty favicon
    app.use('/favicon.ico', (req, res) => res.sendStatus(200));

    // default route to prevent google form crawling our routes just in case
    app.use('/robot.txt', (req, res) => {
        res.type('text/plain');
        res.send('User-agent: *\nDisallow: /*');
    });

    app.use('/robots.txt', (req, res) => {
        res.type('text/plain');
        res.send('User-agent: *\nDisallow: /*');
    });

    return {
        app: app,
        config: config,
    };
}

/**
 * Starts the  provided server object
 * @param server
 */
export async function start(server: Server) {
    // set up one final route that will catch everything else
    server.app.use((req, res) => res.sendStatus(404));

    return new Promise(() => {
        // this is a **http** server. Https comes in through reverse proxy.
        server.app.listen(server.config.port, () => {
            console.log('Now listening on', server.config.port);
        });
    });
}

export default create;
