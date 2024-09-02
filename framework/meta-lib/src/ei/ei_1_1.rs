/// New hooks added in Q4 2021.
///
/// Added a few more managed type & DCDT utilities.
///
/// This list of hooks is reconstructed from history.
pub const EI_1_1_NAMES: &[&str] = &[
    "getGasLeft",
    "getSCAddress",
    "getOwnerAddress",
    "getShardOfAddress",
    "isSmartContract",
    "signalError",
    "getExternalBalance",
    "getBlockHash",
    "getDCDTBalance",
    "getDCDTNFTNameLength",
    "getDCDTNFTAttributeLength",
    "getDCDTNFTURILength",
    "getDCDTTokenData",
    "getDCDTLocalRoles",
    "validateTokenIdentifier",
    "transferValue",
    "transferValueExecute",
    "transferDCDTExecute",
    "transferDCDTNFTExecute",
    "multiTransferDCDTNFTExecute",
    "createAsyncCall",
    "setAsyncContextCallback",
    "upgradeContract",
    "upgradeFromSourceContract",
    "deleteContract",
    "asyncCall",
    "getArgumentLength",
    "getArgument",
    "getFunction",
    "getNumArguments",
    "storageStore",
    "storageLoadLength",
    "storageLoadFromAddress",
    "storageLoad",
    "setStorageLock",
    "getStorageLock",
    "isStorageLocked",
    "clearStorageLock",
    "getCaller",
    "checkNoPayment",
    "getCallValue",
    "getDCDTValue",
    "getDCDTValueByIndex",
    "getDCDTTokenName",
    "getDCDTTokenNameByIndex",
    "getDCDTTokenNonce",
    "getDCDTTokenNonceByIndex",
    "getCurrentDCDTNFTNonce",
    "getDCDTTokenType",
    "getDCDTTokenTypeByIndex",
    "getNumDCDTTransfers",
    "getCallValueTokenName",
    "getCallValueTokenNameByIndex",
    "writeLog",
    "writeEventLog",
    "getBlockTimestamp",
    "getBlockNonce",
    "getBlockRound",
    "getBlockEpoch",
    "getBlockRandomSeed",
    "getStateRootHash",
    "getPrevBlockTimestamp",
    "getPrevBlockNonce",
    "getPrevBlockRound",
    "getPrevBlockEpoch",
    "getPrevBlockRandomSeed",
    "finish",
    "executeOnSameContext",
    "executeOnDestContext",
    "executeReadOnly",
    "createContract",
    "deployFromSourceContract",
    "getNumReturnData",
    "getReturnDataSize",
    "getReturnData",
    "cleanReturnData",
    "deleteFromReturnData",
    "getOriginalTxHash",
    "getCurrentTxHash",
    "getPrevTxHash",
    "managedSCAddress",
    "managedOwnerAddress",
    "managedCaller",
    "managedSignalError",
    "managedWriteLog",
    "managedGetOriginalTxHash",
    "managedGetStateRootHash",
    "managedGetBlockRandomSeed",
    "managedGetPrevBlockRandomSeed",
    "managedGetReturnData",
    "managedGetMultiDCDTCallValue",
    "managedGetDCDTBalance",
    "managedGetDCDTTokenData",
    "managedAsyncCall",
    "managedUpgradeFromSourceContract",
    "managedUpgradeContract",
    "managedDeleteContract",
    "managedDeployFromSourceContract",
    "managedCreateContract",
    "managedExecuteReadOnly",
    "managedExecuteOnSameContext",
    "managedExecuteOnDestContext",
    "managedMultiTransferDCDTNFTExecute",
    "managedTransferValueExecute",
    "bigIntGetUnsignedArgument",
    "bigIntGetSignedArgument",
    "bigIntStorageStoreUnsigned",
    "bigIntStorageLoadUnsigned",
    "bigIntGetCallValue",
    "bigIntGetDCDTCallValue",
    "bigIntGetDCDTCallValueByIndex",
    "bigIntGetExternalBalance",
    "bigIntGetDCDTExternalBalance",
    "bigIntNew",
    "bigIntUnsignedByteLength",
    "bigIntSignedByteLength",
    "bigIntGetUnsignedBytes",
    "bigIntGetSignedBytes",
    "bigIntSetUnsignedBytes",
    "bigIntSetSignedBytes",
    "bigIntIsInt64",
    "bigIntGetInt64",
    "bigIntSetInt64",
    "bigIntAdd",
    "bigIntSub",
    "bigIntMul",
    "bigIntTDiv",
    "bigIntTMod",
    "bigIntEDiv",
    "bigIntEMod",
    "bigIntSqrt",
    "bigIntPow",
    "bigIntLog2",
    "bigIntAbs",
    "bigIntNeg",
    "bigIntSign",
    "bigIntCmp",
    "bigIntNot",
    "bigIntAnd",
    "bigIntOr",
    "bigIntXor",
    "bigIntShr",
    "bigIntShl",
    "bigIntFinishUnsigned",
    "bigIntFinishSigned",
    "mBufferNew",
    "mBufferNewFromBytes",
    "mBufferGetLength",
    "mBufferGetBytes",
    "mBufferGetByteSlice",
    "mBufferCopyByteSlice",
    "mBufferEq",
    "mBufferSetBytes",
    "mBufferSetByteSlice",
    "mBufferAppend",
    "mBufferAppendBytes",
    "mBufferToBigIntUnsigned",
    "mBufferToBigIntSigned",
    "mBufferFromBigIntUnsigned",
    "mBufferFromBigIntSigned",
    "mBufferStorageStore",
    "mBufferStorageLoad",
    "mBufferStorageLoadFromAddress",
    "mBufferGetArgument",
    "mBufferFinish",
    "mBufferSetRandom",
    "managedMapNew",
    "managedMapPut",
    "managedMapGet",
    "managedMapRemove",
    "managedMapContains",
    "smallIntGetUnsignedArgument",
    "smallIntGetSignedArgument",
    "smallIntFinishUnsigned",
    "smallIntFinishSigned",
    "smallIntStorageStoreUnsigned",
    "smallIntStorageStoreSigned",
    "smallIntStorageLoadUnsigned",
    "smallIntStorageLoadSigned",
    "int64getArgument",
    "int64finish",
    "int64storageStore",
    "int64storageLoad",
    "sha256",
    "managedSha256",
    "keccak256",
    "managedKeccak256",
    "ripemd160",
    "verifyBLS",
    "verifyEd25519",
    "verifyCustomSecp256k1",
    "verifySecp256k1",
    "encodeSecp256k1DerSignature",
    "addEC",
    "doubleEC",
    "isOnCurveEC",
    "scalarBaseMultEC",
    "scalarMultEC",
    "marshalEC",
    "marshalCompressedEC",
    "unmarshalEC",
    "unmarshalCompressedEC",
    "generateKeyEC",
    "createEC",
    "getCurveLengthEC",
    "getPrivKeyByteLengthEC",
    "ellipticCurveGetValues",
];
