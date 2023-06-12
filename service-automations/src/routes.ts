import express from 'express';
import GoogleDocJob from './jobs/google_doc_job';
import Job from './jobs/job';
import DestinyMemberReportJob from './jobs/destiny_report';
import DestinyReportNetwork from './jobs/destiny_report_network';

/**
 * An express router that includes all routes
 */
export function router() {
    return express.Router().get('/:job', run_job);
}

async function run_job(request: express.Request, response: express.Response) {
    const public_key = request.header('PUBLIC-KEY') || '';
    const private_key = request.header('PRIVATE-KEY') || '';

    const server_public_key = process.env['KEY_AUTOMATION_ACCESS_PUBLIC'] || '';
    const server_private_key = process.env['KEY_AUTOMATION_ACCESS_PRIVATE'] || '';

    if (server_public_key.trim().length == 0 || server_public_key.trim().length == 0) {
        console.log('No keys provided');
        response.sendStatus(404);
    } else if (public_key.trim().length == 0 || private_key.trim().length == 0) {
        console.log('Public or Private key empty');
        response.sendStatus(403);
    } else if (public_key !== server_public_key || private_key !== server_private_key) {
        console.log('Not a match');
        response.sendStatus(403);
    } else {
        console.log('Parsing job');
        let job = null as Job | null;
        // we are ok to proceed. We have passed our access requirements
        switch (request.params.job) {
            case 'google-doc':
                console.log('Running Google Doc Job');
                const doc_id = typeof request.query.doc === 'string' ? request.query.doc : '';
                const feed = typeof request.query.feed === 'string' ? request.query.feed : '';
                if (doc_id.length > 0 && feed.length > 0) {
                    job = await GoogleDocJob(doc_id, feed);
                } else {
                    console.log('No google doc or target feed provided.');
                    job = null;
                }
                break;
            case 'destiny-member':
                const bungie_name = typeof request.query.bungie_name === 'string' ? request.query.bungie_name : '';
                const season = typeof request.query.season === 'string' ? request.query.season : '';
                if (bungie_name.trim().length > 0 && season.trim().length > 0) {
                    job = await DestinyMemberReportJob(bungie_name, season);
                } else {
                    console.log('Please supply a bungie name and season');
                    job = null;
                }
                break;
            case 'destiny-network-snapshot':
                job = await DestinyReportNetwork();
                break;
            default:
                console.log('Target job does not exist', request.params.job);
                job = null;
                break;
        }

        // if we do have a job, run it and cleanup
        try {
            if (job) {
                console.log('Running job');
                await job.run();

                console.log('Cleanup job');
                await job.cleanup();

                response.sendStatus(200);
            } else {
                response.sendStatus(404);
            }
        } catch (err) {
            console.log('Job error occurred. Cannot continue.', err);
            response.sendStatus(500);
        }
    }
}

export default router;
