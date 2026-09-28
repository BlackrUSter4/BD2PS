CREATE TABLE "GuildJoinApplication" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "GuildId" BIGINT NOT NULL,
    "ApplicantUid" BIGINT NOT NULL,
    "ApplicantUserId" TEXT,
    "Date" BIGINT
);
