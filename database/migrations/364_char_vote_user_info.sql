CREATE TABLE "CharVoteUserInfo" (
    "Uid" BIGINT NOT NULL,
    "EventId" INTEGER NOT NULL,
    "Round" INTEGER NOT NULL,
    "CandidateId" INTEGER NOT NULL,
    "TotalCount" INTEGER NOT NULL DEFAULT 0,
    "NormalCount" INTEGER NOT NULL DEFAULT 0,
    "AdditionalCount" INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY ("Uid", "EventId", "Round", "CandidateId")
);
