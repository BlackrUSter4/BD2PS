-- UserInfo (predates this project) has no column for the account's greeting message or
-- title selection, despite UserGreetingChange/UserTitleChange/UserContentsInfo all needing
-- exactly that.
ALTER TABLE "UserInfo" ADD COLUMN "Greeting" TEXT;
ALTER TABLE "UserInfo" ADD COLUMN "TitleId" INTEGER;
ALTER TABLE "UserInfo" ADD COLUMN "IsAllPrivate" INTEGER NOT NULL DEFAULT 0;
ALTER TABLE "UserInfo" ADD COLUMN "PrivacyOptions" TEXT;
