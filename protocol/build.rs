use std::path::Path;

fn main() {
    let proto_files = [
        "proto/Network_TCP.proto",
        "proto/NLD_MID.proto",
        "proto/PvpBattleDBInfo.proto",
        "proto/PvpBattleRankDetailInfo.proto",
        "proto/PvpBattleUserInfo.proto",
        "proto/block.db/BlockListTable/Proto_Design_block.proto",
        "proto/common.db/AchievementTable/Proto_Design_common.proto",
        "proto/Commons/CashShopDBInfo.proto",
        "proto/Commons/HuntingGroundInfo.proto",
        "proto/Commons/NpcReputationAvgDBInfo.proto",
        "proto/Commons/NpcReputationDBInfo.proto",
        "proto/Commons/proto_net.proto",
        "proto/Commons/WorldBuffDBInfo.proto",
        "proto/FieldObjectSceneData.db/FieldObjectSceneDataTable/Proto_Design_FieldObjectSceneData.proto",
        "proto/pack1.db/BattleTable/Proto_Design_pack1.proto",
        "proto/pack10.db/BattleTable/Proto_Design_pack10.proto",
        "proto/pack10001.db/BattleTable/Proto_Design_pack10001.proto",
        "proto/pack10004.db/BattleTable/Proto_Design_pack10004.proto",
        "proto/pack10006.db/BattleTable/Proto_Design_pack10006.proto",
        "proto/pack1001.db/BattleTable/Proto_Design_pack1001.proto",
        "proto/pack1002.db/BattleTable/Proto_Design_pack1002.proto",
        "proto/pack1003.db/BattleTable/Proto_Design_pack1003.proto",
        "proto/pack1004.db/BattleTable/Proto_Design_pack1004.proto",
        "proto/pack1005.db/BattleTable/Proto_Design_pack1005.proto",
        "proto/pack1006.db/BattleTable/Proto_Design_pack1006.proto",
        "proto/pack1007.db/BattleTable/Proto_Design_pack1007.proto",
        "proto/pack11.db/BattleTable/Proto_Design_pack11.proto",
        "proto/pack11001.db/BattleTable/Proto_Design_pack11001.proto",
        "proto/pack11002.db/BattleTable/Proto_Design_pack11002.proto",
        "proto/pack11003.db/BattleTable/Proto_Design_pack11003.proto",
        "proto/pack11004.db/BattleTable/Proto_Design_pack11004.proto",
        "proto/pack11005.db/BattleTable/Proto_Design_pack11005.proto",
        "proto/pack11006.db/BattleTable/Proto_Design_pack11006.proto",
        "proto/pack11007.db/BattleTable/Proto_Design_pack11007.proto",
        "proto/pack12.db/BattleTable/Proto_Design_pack12.proto",
        "proto/pack12001.db/BattleTable/Proto_Design_pack12001.proto",
        "proto/pack12002.db/BattleTable/Proto_Design_pack12002.proto",
        "proto/pack12003.db/BattleTable/Proto_Design_pack12003.proto",
        "proto/pack12004.db/BattleTable/Proto_Design_pack12004.proto",
        "proto/pack12005.db/BattleTable/Proto_Design_pack12005.proto",
        "proto/pack12006.db/BattleTable/Proto_Design_pack12006.proto",
        "proto/pack12007.db/BattleTable/Proto_Design_pack12007.proto",
        "proto/pack13.db/BattleTable/Proto_Design_pack13.proto",
        "proto/pack14.db/BattleTable/Proto_Design_pack14.proto",
        "proto/pack15.db/BattleTable/Proto_Design_pack15.proto",
        "proto/pack16.db/BattleTable/Proto_Design_pack16.proto",
        "proto/pack17.db/BattleTable/Proto_Design_pack17.proto",
        "proto/pack18.db/BattleTable/Proto_Design_pack18.proto",
        "proto/pack2.db/BattleTable/Proto_Design_pack2.proto",
        "proto/pack2001.db/BattleTable/Proto_Design_pack2001.proto",
        "proto/pack2002.db/BattleTable/Proto_Design_pack2002.proto",
        "proto/pack2003.db/BattleTable/Proto_Design_pack2003.proto",
        "proto/pack2004.db/BattleTable/Proto_Design_pack2004.proto",
        "proto/pack2005.db/BattleTable/Proto_Design_pack2005.proto",
        "proto/pack2006.db/BattleTable/Proto_Design_pack2006.proto",
        "proto/pack2007.db/BattleTable/Proto_Design_pack2007.proto",
        "proto/pack3.db/BattleTable/Proto_Design_pack3.proto",
        "proto/pack3001.db/BattleTable/Proto_Design_pack3001.proto",
        "proto/pack3002.db/BattleTable/Proto_Design_pack3002.proto",
        "proto/pack3003.db/BattleTable/Proto_Design_pack3003.proto",
        "proto/pack3004.db/BattleTable/Proto_Design_pack3004.proto",
        "proto/pack3005.db/BattleTable/Proto_Design_pack3005.proto",
        "proto/pack3006.db/BattleTable/Proto_Design_pack3006.proto",
        "proto/pack3007.db/BattleTable/Proto_Design_pack3007.proto",
        "proto/pack4.db/BattleTable/Proto_Design_pack4.proto",
        "proto/pack5.db/BattleTable/Proto_Design_pack5.proto",
        "proto/pack6.db/BattleTable/Proto_Design_pack6.proto",
        "proto/pack7.db/BattleTable/Proto_Design_pack7.proto",
        "proto/pack8.db/BattleTable/Proto_Design_pack8.proto",
        "proto/pack9.db/BattleTable/Proto_Design_pack9.proto",
        "proto/pack19.db/BattleTable/Proto_Design_pack19.proto",
        "proto/pack20.db/BattleTable/Proto_Design_pack20.proto",
        "proto/pack21.db/BattleTable/Proto_Design_pack21.proto",
        "proto/pack22.db/BattleTable/Proto_Design_pack22.proto",
        "proto/pack1008.db/BattleTable/Proto_Design_pack1008.proto",
        "proto/pack1009.db/BattleTable/Proto_Design_pack1009.proto",
        "proto/pack1010.db/BattleTable/Proto_Design_pack1010.proto",
        "proto/pack2008.db/BattleTable/Proto_Design_pack2008.proto",
        "proto/pack2009.db/BattleTable/Proto_Design_pack2009.proto",
        "proto/pack2010.db/BattleTable/Proto_Design_pack2010.proto",
        "proto/pack3008.db/BattleTable/Proto_Design_pack3008.proto",
        "proto/pack3009.db/BattleTable/Proto_Design_pack3009.proto",
        "proto/pack3010.db/BattleTable/Proto_Design_pack3010.proto",
        "proto/pack3011.db/BattleTable/Proto_Design_pack3011.proto",
        "proto/pack3012.db/BattleTable/Proto_Design_pack3012.proto",
        "proto/pack11008.db/BattleTable/Proto_Design_pack11008.proto",
        "proto/pack11009.db/BattleTable/Proto_Design_pack11009.proto",
        "proto/pack11010.db/BattleTable/Proto_Design_pack11010.proto",
        "proto/pack12008.db/BattleTable/Proto_Design_pack12008.proto",
        "proto/pack12009.db/BattleTable/Proto_Design_pack12009.proto",
        "proto/pack12010.db/BattleTable/Proto_Design_pack12010.proto",
        "proto/pack13007.db/BattleTable/Proto_Design_pack13007.proto",
        "proto/pack20000.db/BattleTable/Proto_Design_pack20000.proto",
        "proto/Request/ChargeCostRequest.proto",
        "proto/Request/EvilCastleMyRankingInfoRequest.proto",
        "proto/Request/ImmortalQuestResetRequest.proto",
        "proto/Request/NpcReputationInfoRequest.proto",
        "proto/Request/NpcReputationRecoveryRequest.proto",
        "proto/Request/SendLogRequest.proto",
        "proto/Response/NpcReputationInfoResponse.proto",
        "proto/Response/NpcReputationRecoveryResponse.proto",
        "proto/Response/SendLogResponse.proto",
    ];

    for proto in &proto_files {
        println!("cargo::rerun-if-changed={proto}");
    }

    // Filter only files that actually exist
    let existing: Vec<_> = proto_files
        .iter()
        .filter(|f| Path::new(f).exists())
        .collect();

    if existing.is_empty() {
        eprintln!(" No .proto files found — check your proto/ directory paths!");
        return;
    }

    prost_build::Config::new()
            .type_attribute(
                ".",
                "#[derive(serde::Serialize, serde::Deserialize)]\n#[serde(rename_all = \"PascalCase\")]",
            )
            .message_attribute(".", r#"#[serde(default)]"#)
            .field_attribute("*.type", "#[serde(rename = \"type\")]")
            .out_dir("include/")
            .compile_protos(&existing, &["proto"])
            .expect("Failed to compile proto files");
}
