CREATE TABLE "TalentSkillInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "GroupId" INTEGER,
    "EndTime" BIGINT,
    "CoolTime" BIGINT,
    "UseCount" BIGINT
);