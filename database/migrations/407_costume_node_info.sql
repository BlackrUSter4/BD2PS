-- Real per-account "which CostumeNodeTable nodes are activated on this owned costume" state.
-- No pre-existing scaffolding for this (Costume cluster's other 2 stateful tables --
-- CostumePotentialConnectInfo, CostumeUseInfo -- predate this project; this one is new).
CREATE TABLE "CostumeNodeInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "CostumeInvenIndex" BIGINT NOT NULL,
    "NodeId" INTEGER NOT NULL
);
