CREATE TABLE "CharVoteDailyState" (
    "Uid" BIGINT PRIMARY KEY,
    "LastResetDate" TEXT NOT NULL DEFAULT '',
    "NormalVoteCandidateIds" TEXT NOT NULL DEFAULT ''
);
