import DestinyMemberReportJob from './destiny_report';
import Job from './job';
import { APIResponse, ServiceDestiny } from '@levelcrush';

async function get_clan_members() {
    const destiny_api = process.env['HOST_DESTINY'] || '';
    const request = await fetch(destiny_api + '/network/roster');
    if (request.ok) {
        const data = (await request.json()) as APIResponse<ServiceDestiny.MemberResponse[]>;
        return data.response === null ? [] : data.response;
    } else {
        return [];
    }
}

export const DestinyReportNetwork = async () => {
    const run = async () => {
        const clan_roster = await get_clan_members();
        const report_task = [];
        const max_snapshot = parseInt(process.env['SNAPSHOT_SEASON_MAX'] || '20');
        for (const member of clan_roster) {
            for (let season = 0; season < max_snapshot; season++) {
                const job = await DestinyMemberReportJob(member.display_name, (season + 1).toString());
                report_task.push(job);
            }
        }

        for (let i = 0; i < report_task.length; i++) {
            report_task[i].run();
            console.log('Waiting buffer time to move on');
            await new Promise((resolve) => {
                setTimeout(() => resolve(1), 100);
            });
            console.log('Moving on...');
        }
        //await Promise.allSettled(report_task);
    };

    const cleanup = async () => {
        //todo!
    };

    return { run, cleanup } as Job;
};

export default DestinyReportNetwork;
