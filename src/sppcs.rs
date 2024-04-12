#![allow(non_snake_case)]

#[no_mangle]
extern "system" fn SLCallServer() {}
#[no_mangle]
extern "system" fn SLpAuthenticateGenuineTicketResponse() {}
#[no_mangle]
extern "system" fn SLpBeginGenuineTicketTransaction() {}
#[no_mangle]
extern "system" fn SLpClearActivationInProgress() {}
#[no_mangle]
extern "system" fn SLpDepositDownlevelGenuineTicket() {}
#[no_mangle]
extern "system" fn SLpDepositTokenActivationResponse() {}
#[no_mangle]
extern "system" fn SLpGenerateTokenActivationChallenge() {}
#[no_mangle]
extern "system" fn SLpGetGenuineBlob() {}
#[no_mangle]
extern "system" fn SLpGetGenuineLocal() {}
#[no_mangle]
extern "system" fn SLpGetLicenseAcquisitionInfo() {}
#[no_mangle]
extern "system" fn SLpGetMSPidInformation() {}
#[no_mangle]
extern "system" fn SLpGetMachineUGUID() {}
#[no_mangle]
extern "system" fn SLpGetTokenActivationGrantInfo() {}
#[no_mangle]
extern "system" fn SLpIAActivateProduct() {}
#[no_mangle]
extern "system" fn SLpIsCurrentInstalledProductKeyDefaultKey() {}
#[no_mangle]
extern "system" fn SLpProcessVMPipeMessage() {}
#[no_mangle]
extern "system" fn SLpSetActivationInProgress() {}
#[no_mangle]
extern "system" fn SLpTriggerServiceWorker() {}
#[no_mangle]
extern "system" fn SLpVLActivateProduct() {}
#[no_mangle]
extern "system" fn SLClose() {}
#[no_mangle]
extern "system" fn SLConsumeRight() {}
#[no_mangle]
extern "system" fn SLDepositMigrationBlob() {}
#[no_mangle]
extern "system" fn SLDepositOfflineConfirmationId() {}
#[no_mangle]
extern "system" fn SLDepositOfflineConfirmationIdEx() {}
#[no_mangle]
extern "system" fn SLDepositStoreToken() {}
#[no_mangle]
extern "system" fn SLFireEvent() {}
#[no_mangle]
extern "system" fn SLGatherMigrationBlob() {}
#[no_mangle]
extern "system" fn SLGatherMigrationBlobEx() {}
#[no_mangle]
extern "system" fn SLGenerateOfflineInstallationId() {}
#[no_mangle]
extern "system" fn SLGenerateOfflineInstallationIdEx() {}
#[no_mangle]
extern "system" fn SLGetActiveLicenseInfo() {}
#[no_mangle]
extern "system" fn SLGetApplicationInformation() {}
#[no_mangle]
extern "system" fn SLGetApplicationPolicy() {}
#[no_mangle]
extern "system" fn SLGetAuthenticationResult() {}
#[no_mangle]
extern "system" fn SLGetEncryptedPIDEx() {}
#[no_mangle]
extern "system" fn SLGetGenuineInformation() {}
#[no_mangle]
extern "system" fn SLGetInstalledProductKeyIds() {}
#[no_mangle]
extern "system" fn SLGetLicense() {}
#[no_mangle]
extern "system" fn SLGetLicenseFileId() {}
#[no_mangle]
extern "system" fn SLGetLicenseInformation() {}
#[no_mangle]
extern "system" fn SLGetPKeyId() {}
#[no_mangle]
extern "system" fn SLGetPKeyInformation() {}
#[no_mangle]
extern "system" fn SLGetPolicyInformation() {}
#[no_mangle]
extern "system" fn SLGetPolicyInformationDWORD() {}
#[no_mangle]
extern "system" fn SLGetSLIDList() {}
#[no_mangle]
extern "system" fn SLGetServiceInformation() {}
#[no_mangle]
extern "system" fn SLInstallLicense() {}
#[no_mangle]
extern "system" fn SLInstallProofOfPurchase() {}
#[no_mangle]
extern "system" fn SLInstallProofOfPurchaseEx() {}
#[no_mangle]
extern "system" fn SLIsGenuineLocalEx() {}
#[no_mangle]
extern "system" fn SLLoadApplicationPolicies() {}
#[no_mangle]
extern "system" fn SLOpen() {}
#[no_mangle]
extern "system" fn SLPersistApplicationPolicies() {}
#[no_mangle]
extern "system" fn SLPersistRTSPayloadOverride() {}
#[no_mangle]
extern "system" fn SLReArm() {}
#[no_mangle]
extern "system" fn SLRegisterEvent() {}
#[no_mangle]
extern "system" fn SLRegisterPlugin() {}
#[no_mangle]
extern "system" fn SLSetAuthenticationData() {}
#[no_mangle]
extern "system" fn SLSetCurrentProductKey() {}
#[no_mangle]
extern "system" fn SLSetGenuineInformation() {}
#[no_mangle]
extern "system" fn SLUninstallLicense() {}
#[no_mangle]
extern "system" fn SLUninstallProofOfPurchase() {}
#[no_mangle]
extern "system" fn SLUnloadApplicationPolicies() {}
#[no_mangle]
extern "system" fn SLUnregisterEvent() {}
#[no_mangle]
extern "system" fn SLUnregisterPlugin() {}
