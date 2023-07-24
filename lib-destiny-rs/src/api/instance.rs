use levelcrush::anyhow;

use crate::aliases::*;
use crate::bungie::enums::DestinyRouteParam;
use crate::bungie::schemas::DestinyPostGameCarnageReportData;
use crate::bungie::BungieClient;

/// get the carnage report from the bungie api
pub async fn carnage_report(
    instance_id: InstanceId,
    bungie: &BungieClient,
) -> anyhow::Result<Option<DestinyPostGameCarnageReportData>> {
    let request = bungie
        .get("/Destiny2/Stats/PostGameCarnageReport/{activityId}")
        .param(DestinyRouteParam::Activity, instance_id.to_string())
        .send::<DestinyPostGameCarnageReportData>()
        .await?;

    Ok(request.response)
}
