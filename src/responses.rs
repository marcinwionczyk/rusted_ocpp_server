use serde::Serialize;


#[derive(Serialize)]
pub struct AuthorizeResponse {
    #[serde(rename = "idTagInfo")]
    pub id_tag_info: IdTagInfo,
}

#[derive(Serialize)]
pub struct IdTagInfo {
    #[serde(rename = "expiryDate")]
    pub expiry_date: Option<String>,
    #[serde(rename = "parentIdTag")]
    pub parent_id_tag: Option<String>,
    pub status: IdTagInfoStatus,
}
////////////////////
#[derive(Serialize)]
pub struct BootNotificationResponse {
    #[serde(rename = "currentTime")]
    pub current_time: String,
    pub interval: i64,
    pub status: BootNotificationResponseStatus,
}

#[derive(Serialize)]
pub struct CancelReservationResponse {
    pub status: AcceptedRejectedStatus,
}

#[derive(Serialize)]
pub struct ChangeAvailabilityResponse {
    pub status: ChangeAvailabilityResponseStatus,
}

#[derive(Serialize)]
pub struct ChangeConfigurationResponse {
    pub status: ChangeConfigurationResponseStatus,
}

#[derive(Serialize)]
pub struct ClearCacheResponse {
    pub status: ClearCacheResponseStatus,
}

#[derive(Serialize)]
pub struct ClearChargingProfileResponse {
    pub status: ClearChargingProfileResponseStatus,
}

#[derive(Serialize)]
pub struct DataTransferResponse {
    pub data: Option<String>,
    pub status: DataTransferResponseStatus,
}

#[derive(Serialize)]
pub struct DiagnosticsStatusNotificationResponse {}

#[derive(Serialize)]
pub struct FirmwareStatusNotificationResponse {}

#[derive(Serialize)]
pub struct GetCompositeScheduleResponse {
    #[serde(rename = "chargingSchedule")]
    pub charging_schedule: Option<ChargingSchedule>,
    #[serde(rename = "connectorId")]
    pub connector_id: Option<i64>,
    #[serde(rename = "scheduleStart")]
    pub schedule_start: Option<String>,
    pub status: AcceptedRejectedStatus,
}

#[derive(Serialize)]
pub struct ChargingSchedule {
    #[serde(rename = "chargingRateUnit")]
    pub charging_rate_unit: ChargingRateUnit,
    #[serde(rename = "chargingSchedulePeriod")]
    pub charging_schedule_period: Vec<ChargingSchedulePeriod>,
    pub duration: Option<i64>,
    #[serde(rename = "minChargingRate")]
    pub min_charging_rate: Option<f64>,
    #[serde(rename = "startSchedule")]
    pub start_schedule: Option<String>,
}

#[derive(Serialize)]
pub struct ChargingSchedulePeriod {
    pub limit: f64,
    #[serde(rename = "numberPhases")]
    pub number_phases: Option<i64>,
    #[serde(rename = "startPeriod")]
    pub start_period: i64,
}

#[derive(Serialize)]
pub struct GetConfigurationResponse {
    #[serde(rename = "configurationKey")]
    pub configuration_key: Option<Vec<ConfigurationKey>>,
    #[serde(rename = "unknownKey")]
    pub unknown_key: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct ConfigurationKey {
    pub key: String,
    pub readonly: bool,
    pub value: Option<String>,
}

#[derive(Serialize)]
pub struct GetDiagnosticsResponse {
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
}

#[derive(Serialize)]
pub struct GetLocalListVersionResponse {
    #[serde(rename = "listVersion")]
    pub list_version: i64,
}

#[derive(Serialize)]
pub struct HeartbeatResponse {
    #[serde(rename = "currentTime")]
    pub current_time: String,
}

#[derive(Serialize)]
pub struct MeterValuesResponse {}

#[derive(Serialize)]
pub struct RemoteStartTransactionResponse {
    pub status: AcceptedRejectedStatus,
}

#[derive(Serialize)]
pub struct RemoteStopTransactionResponse {
    pub status: AcceptedRejectedStatus,
}

#[derive(Serialize)]
pub struct ReserveNowResponse {
    pub status: ReserveNowResponseStatus,
}

#[derive(Serialize)]
pub struct ResetResponse {
    pub status: AcceptedRejectedStatus,
}

#[derive(Serialize)]
pub struct SendLocalListResponse {
    pub status: SendLocalListResponseStatus,
}

#[derive(Serialize)]
pub struct SetChargingProfileResponse {
    pub status: SetChargingProfileResponseStatus,
}

#[derive(Serialize)]
pub struct StartTransactionResponse {
    #[serde(rename = "idTagInfo")]
    pub id_tag_info: IdTagInfo,
    #[serde(rename = "transactionId")]
    pub transaction_id: i64,
}

#[derive(Serialize)]
pub struct StatusNotificationResponse {}

#[derive(Serialize)]
pub struct StopTransactionResponse {
    #[serde(rename = "idTagInfo")]
    pub id_tag_info: Option<IdTagInfo>,
}

#[derive(Serialize)]
pub struct TriggerMessageResponse {
    pub status: AcceptedNotImplementedRejectedStatus,
}

#[derive(Serialize)]
pub struct UnlockConnectorResponse {
    pub status: UnlockConnectorResponseStatus,
}

#[derive(Serialize)]
pub struct UpdateFirmwareResponse {}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize)]
pub enum BootNotificationResponseStatus {
    Accepted,
    Pending,
    Rejected,
}

#[derive(Serialize)]
pub enum IdTagInfoStatus {
    Accepted,
    Blocked,
    ConcurrentTx,
    Expired,
    Invalid,
}

#[derive(Serialize)]
pub enum AcceptedRejectedStatus {
    Accepted,
    Rejected,
}

#[derive(Serialize)]
pub enum ChangeAvailabilityResponseStatus {
    Accepted,
    Rejected,
    Scheduled,
}

#[derive(Serialize)]
pub enum ChangeConfigurationResponseStatus {
    Accepted,
    NotSupported,
    RebootRequired,
    Rejected,
}

#[derive(Serialize)]
pub enum ClearCacheResponseStatus {
    Accepted,
    Rejected,
}

#[derive(Serialize)]
pub enum ClearChargingProfileResponseStatus {
    Accepted,
    Unknown,
}

#[derive(Serialize)]
pub enum DataTransferResponseStatus {
    Accepted,
    Rejected,
    UnknownMessageId,
    UnknownVendorId,
}

#[derive(Serialize)]
pub enum ChargingRateUnit {
    A,
    W,
}

#[derive(Serialize)]
pub enum ReserveNowResponseStatus {
    Accepted,
    Faulted,
    Occupied,
    Rejected,
    Unavailable,
}

#[derive(Serialize)]
pub enum SendLocalListResponseStatus {
    Accepted,
    Failed,
    NotSupported,
    VersionMismatch,
}

#[derive(Serialize)]
pub enum SetChargingProfileResponseStatus {
    Accepted,
    NotSupported,
    Rejected,
}

#[derive(Serialize)]
pub enum AcceptedNotImplementedRejectedStatus {
    Accepted,
    NotImplemented,
    Rejected,
}

#[derive(Serialize)]
pub enum UnlockConnectorResponseStatus {
    NotSupported,
    UnlockFailed,
    Unlocked,
}
