import Job from './job';
import { APIResponse, ServiceDestiny } from '@levelcrush';

async function check_in(bungie_name: string, season: string) {
    console.log('Checking report for: ', bungie_name, ' during ', season);
    const destiny_api = process.env['HOST_DESTINY'] || 'https://destiny.levelcrush.local';
    const report = season === 'lifetime' ? 'lifetime' : 'season/' + season;
    const api_response = await fetch(destiny_api + '/member/' + encodeURIComponent(bungie_name) + '/report/' + report);
    if (api_response.ok) {
        const json = (await api_response.json()) as APIResponse<ServiceDestiny.ReportOutput>;
        if (typeof json.response === 'number') {
            // if its a number, this job is still processing, so check back in another 10 seconds
            await new Promise((resolve) => {
                setTimeout(async () => {
                    resolve(await check_in(bungie_name, season));
                }, 10 * 1000);
            });
        } else {
            return json;
        }
    }
    return null;
}

export const DestinyMemberReportJob = async (bungie_name: string, report: string) => {
    const run = async () => {
        const fetch_result = await check_in(bungie_name, report);
        if (fetch_result === null || fetch_result.response === null) {
            console.log('No member found');
        } else {
            /*
            console.log('Saving into feed database');
            const feed_endpoint = process.env['HOST_API_FEED'] || '';
            const feed_target = feed_endpoint + '/' + encodeURIComponent(bungie_name + '_' + report);
            console.log(feed_target);
            await fetch(feed_target, {
                method: 'POST',
                cache: 'no-store',
                body: JSON.stringify(fetch_result),
                headers: {
                    'Content-Type': 'application/json',
                    'Public-Key': process.env['FEED_PUBLIC_KEY'] || '',
                    'Private-Key': process.env['FEED_PRIVATE_KEY'] || '',
                },
            }); */
            console.log('Done saving!');
        }
    };

    const cleanup = async () => {
        //todo!
    };

    return { run, cleanup } as Job;
};

export default DestinyMemberReportJob;
