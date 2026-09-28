CREATE TABLE "RoomChatBlock" (
    "Uid" BIGINT NOT NULL,
    "TargetOwnerIndex" BIGINT NOT NULL,
    "TargetUserId" TEXT,
    "BlockDate" BIGINT NOT NULL,
    PRIMARY KEY ("Uid", "TargetOwnerIndex")
);
