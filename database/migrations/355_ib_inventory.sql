CREATE TABLE "IbInventory" (
    "Uid" BIGINT NOT NULL,
    "InvenIndex" BIGINT NOT NULL,
    "Type" INTEGER NOT NULL,
    "ItemId" INTEGER NOT NULL,
    "Level" INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY ("Uid", "InvenIndex")
);
