CREATE TABLE "GachaInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "ScheduleInfoIndex" TEXT, -- References GachaScheduleInfo.InvenIndex
    "GachaUserInfoIndex" TEXT, -- References GachaUserInfo.InvenIndex
    "ScheduleEndExchangePoint" INTEGER,
    "GachaFixedInfoIndex" TEXT, -- References GachaFixedInfo.InvenIndex
    "GachaSelectionInfoIndex" TEXT, -- References GachaSelectionInfo.InvenIndex
    "GachaSelectionChangeCountInfoIndex" TEXT, -- References GachaSelectionCountChangeInfo.InvenIndex
    "StepUpScheduleInfoIndex" TEXT, -- References GachaStepUpScheduleInfo.InvenIndex
    "StepUpUserInfoIndex" TEXT, -- References GachaStepUpUserInfo.InvenIndex
    "ResemaraPreviewItemInfoIndex" TEXT -- References ResemaraGachaInfo.InvenIndex
);