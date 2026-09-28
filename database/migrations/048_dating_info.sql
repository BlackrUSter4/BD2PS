CREATE TABLE "DatingInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "EpisodeInfoIndex" TEXT, -- References DatingEpisodeInfo.InvenIndex
    "MessageChoiceInfoIndex" TEXT -- References DatingMessageChoiceInfo.InvenIndex
);