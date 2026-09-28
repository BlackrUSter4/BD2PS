CREATE TABLE "MiniGameHopscotchGallery" (
    "Uid" BIGINT NOT NULL,
    "EventScheduleId" INTEGER NOT NULL,
    "GalleryId" INTEGER NOT NULL,
    PRIMARY KEY ("Uid", "EventScheduleId", "GalleryId")
);
