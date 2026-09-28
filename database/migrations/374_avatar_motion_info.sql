CREATE TABLE "AvatarMotionInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "MotionId" INTEGER NOT NULL,
    UNIQUE("Uid", "MotionId")
);
