use serde::{Serialize, Deserialize};

// structures created with the help of https://app.quicktype.io/ and json schema provided by
// https://www.openchargealliance.org/

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizeResponse {
    #[serde(rename = "idTagInfo")]
    pub id_tag_info: IdTagInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdTagInfo {
    #[serde(rename = "expiryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(rename = "parentIdTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id_tag: Option<String>,
    pub status: IdTagInfoStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IdTagInfoStatus {
    Accepted,
    Blocked,
    ConcurrentTx,
    Expired,
    Invalid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BootNotificationResponse {
    #[serde(rename = "currentTime")]
    pub current_time: String,
    pub interval: i64,
    pub status: BootNotificationStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BootNotificationStatus {
    Accepted,
    Pending,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CancelReservationResponse {
    pub status: CancelReservationStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CancelReservationStatus {
    Accepted,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeAvailabilityResponse {
    pub status: ChangeAvailabilityStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChangeAvailabilityStatus {
    Accepted,
    Rejected,
    Scheduled,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeConfigurationResponse {
    pub status: ChangeConfigurationStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChangeConfigurationStatus {
    Accepted,
    NotSupported,
    RebootRequired,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClearCacheResponse {
    pub status: ClearCacheStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClearCacheStatus {
    Accepted,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClearChargingProfileResponse {
    pub status: ClearChargingProfileStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClearChargingProfileStatus {
    Accepted,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataTransferResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    pub status: DataTransferStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DataTransferStatus {
    Accepted,
    Rejected,
    UnknownMessageId,
    UnknownVendorId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnosticsStatusNotificationResponse {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FirmwareStatusNotificationResponse {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCompositeScheduleResponse {
    #[serde(rename = "chargingSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_schedule: Option<ChargingSchedule>,
    #[serde(rename = "connectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i64>,
    #[serde(rename = "scheduleStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_start: Option<String>,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChargingSchedule {
    #[serde(rename = "chargingRateUnit")]
    pub charging_rate_unit: ChargingRateUnit,
    #[serde(rename = "chargingSchedulePeriod")]
    pub charging_schedule_period: Vec<ChargingSchedulePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "minChargingRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charging_rate: Option<f64>,
    #[serde(rename = "startSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_schedule: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChargingSchedulePeriod {
    pub limit: f64,
    #[serde(rename = "numberPhases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_phases: Option<i64>,
    #[serde(rename = "startPeriod")]
    pub start_period: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChargingRateUnit {
    A,
    W,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Accepted,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetConfigurationResponse {
    #[serde(rename = "configurationKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_key: Option<Vec<ConfigurationKey>>,
    #[serde(rename = "unknownKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_key: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigurationKey {
    pub key: String,
    pub readonly: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetDiagnosticsResponse {
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLocalListVersionResponse {
    #[serde(rename = "listVersion")]
    pub list_version: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeartbeatResponse {
    #[serde(rename = "currentTime")]
    pub current_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeterValuesResponse {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoteStartTransactionResponse {
    pub status: RemoteStartTransactionStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RemoteStartTransactionStatus {
    Accepted,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoteStopTransactionResponse {
    pub status: RemoteStopTransactionStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RemoteStopTransactionStatus {
    Accepted,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReserveNowResponse {
    pub status: ReserveNowStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReserveNowStatus {
    Accepted,
    Faulted,
    Occupied,
    Rejected,
    Unavailable,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResetResponse {
    pub status: ResetStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResetStatus {
    Accepted,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendLocalListResponse {
    pub status: SendLocalListStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SendLocalListStatus {
    Accepted,
    Failed,
    NotSupported,
    VersionMismatch,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetChargingProfileResponse {
    pub status: SetChargingProfileStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SetChargingProfileStatus {
    Accepted,
    NotSupported,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartTransactionResponse {
    #[serde(rename = "idTagInfo")]
    pub id_tag_info: IdTagInfo,
    #[serde(rename = "transactionId")]
    pub transaction_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusNotificationResponse {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StopTransactionResponse {
    #[serde(rename = "idTagInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag_info: Option<IdTagInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TriggerMessageResponse {
    pub status: TriggerMessageStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TriggerMessageStatus {
    Accepted,
    NotImplemented,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnlockConnectorResponse {
    pub status: UnlockConnectorStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UnlockConnectorStatus {
    NotSupported,
    UnlockFailed,
    Unlocked,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFirmwareResponse {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CertificateSignedResponse {
    pub status: CertificateSignedStatusEnumType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CertificateSignedStatusEnumType {
    Accepted,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteCertificateResponse {
    pub status: DeleteCertificateStatusEnumType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DeleteCertificateStatusEnumType {
    Accepted,
    Failed,
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtendedTriggerMessageResponse {
    pub status: TriggerMessageStatusEnumType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TriggerMessageStatusEnumType {
    Accepted,
    NotImplemented,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInstalledCertificateIdsResponse {
    #[serde(rename = "certificateHashData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_hash_data: Option<Vec<CertificateHashDataType>>,
    pub status: GetInstalledCertificateStatusEnumType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CertificateHashDataType {
    #[serde(rename = "hashAlgorithm")]
    pub hash_algorithm: HashAlgorithmEnumType,
    #[serde(rename = "issuerKeyHash")]
    pub issuer_key_hash: String,
    #[serde(rename = "issuerNameHash")]
    pub issuer_name_hash: String,
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HashAlgorithmEnumType {
    #[serde(rename = "SHA256")]
    Sha256,
    #[serde(rename = "SHA384")]
    Sha384,
    #[serde(rename = "SHA512")]
    Sha512,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetInstalledCertificateStatusEnumType {
    Accepted,
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLogResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    pub status: LogStatusEnumType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LogStatusEnumType {
    Accepted,
    AcceptedCanceled,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstallCertificateResponse {
    pub status: InstallCertificateStatusEnumType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InstallCertificateStatusEnumType {
    Accepted,
    Failed,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogStatusNotificationResponse {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecurityEventNotificationResponse {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignCertificateResponse {
    pub status: GenericStatusEnumType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GenericStatusEnumType {
    Accepted,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignedFirmwareStatusNotificationResponse {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignedUpdateFirmwareResponse {
    pub status: UpdateFirmwareStatusEnumType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UpdateFirmwareStatusEnumType {
    Accepted,
    AcceptedCanceled,
    InvalidCertificate,
    Rejected,
    RevokedCertificate,
}
