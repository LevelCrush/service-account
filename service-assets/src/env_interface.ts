export interface ENV {
    server?: {
        session?: {
            ttl?: 86400 | number;
            secret?: string;
        };
        port?: number;
        assets?: string;
        guideCache?: string;
        domain?: string;
        url?: string;
    };
    hosts: {
        api: string;
        login: string;
        frontend: string;
    };
    platforms: {
        api: {
            token: string;
            token_secret: string;
            application: string;
        };
    };
}

export default ENV;
