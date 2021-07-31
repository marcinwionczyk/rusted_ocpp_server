use serde::{Serialize, Deserialize};
use serde_json::Value;

// structures created with the help of https://app.quicktype.io/ and json schema provided by
// https://www.openchargealliance.org/

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizeRequest {
    #[serde(rename = "idTag")]
    pub id_tag: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BootNotificationRequest {
    #[serde(rename = "chargeBoxSerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_box_serial_number: Option<String>,
    #[serde(rename = "chargePointModel")]
    pub charge_point_model: String,
    #[serde(rename = "chargePointSerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_point_serial_number: Option<String>,
    #[serde(rename = "chargePointVendor")]
    pub charge_point_vendor: String,
    #[serde(rename = "firmwareVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
    #[serde(rename = "meterSerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_serial_number: Option<String>,
    #[serde(rename = "meterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFirmwareRequest {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    #[serde(rename = "retrieveDate")]
    pub retrieve_date: String,
    #[serde(rename = "retryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnlockConnectorRequest {
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CancelReservationRequest {
    #[serde(rename = "reservationId")]
    pub reservation_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeAvailabilityRequest {
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
    #[serde(rename = "type")]
    pub change_availability_request_type: Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Type {
    Inoperative,
    Operative,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeConfigurationRequest {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClearCacheRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClearChargingProfileRequest {
    #[serde(rename = "chargingProfilePurpose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurpose>,
    #[serde(rename = "connectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "stackLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChargingProfilePurpose {
    ChargePointMaxProfile,
    TxDefaultProfile,
    TxProfile,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataTransferRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "vendorId")]
    pub vendor_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnosticsStatusNotificationRequest {
    pub status: DiagnosticsStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DiagnosticsStatus {
    Idle,
    UploadFailed,
    Uploaded,
    Uploading,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FirmwareStatusNotificationRequest {
    pub status: FirmwareStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FirmwareStatus {
    DownloadFailed,
    Downloaded,
    Downloading,
    Idle,
    InstallationFailed,
    Installed,
    Installing,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCompositeScheduleRequest {
    #[serde(rename = "chargingRateUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_rate_unit: Option<ChargingRateUnit>,
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
    pub duration: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChargingRateUnit {
    A,
    W,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetConfigurationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetDiagnosticsRequest {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    #[serde(rename = "retryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i64>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "stopTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLocalListVersionRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeartbeatRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeterValuesRequest {
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
    #[serde(rename = "meterValue")]
    pub meter_value: Vec<MeterValue>,
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeterValue {
    #[serde(rename = "sampledValue")]
    pub sampled_value: Vec<SampledValue>,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SampledValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurand: Option<Measurand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<Phase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Context {
    #[serde(rename = "Interruption.Begin")]
    InterruptionBegin,
    #[serde(rename = "Interruption.End")]
    InterruptionEnd,
    Other,
    #[serde(rename = "Sample.Clock")]
    SampleClock,
    #[serde(rename = "Sample.Periodic")]
    SamplePeriodic,
    #[serde(rename = "Transaction.Begin")]
    TransactionBegin,
    #[serde(rename = "Transaction.End")]
    TransactionEnd,
    Trigger,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Format {
    Raw,
    SignedData,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Location {
    Body,
    Cable,
    #[serde(rename = "EV")]
    Ev,
    Inlet,
    Outlet,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Measurand {
    #[serde(rename = "Current.Export")]
    CurrentExport,
    #[serde(rename = "Current.Import")]
    CurrentImport,
    #[serde(rename = "Current.Offered")]
    CurrentOffered,
    #[serde(rename = "Energy.Active.Export.Interval")]
    EnergyActiveExportInterval,
    #[serde(rename = "Energy.Active.Export.Register")]
    EnergyActiveExportRegister,
    #[serde(rename = "Energy.Active.Import.Interval")]
    EnergyActiveImportInterval,
    #[serde(rename = "Energy.Active.Import.Register")]
    EnergyActiveImportRegister,
    #[serde(rename = "Energy.Reactive.Export.Interval")]
    EnergyReactiveExportInterval,
    #[serde(rename = "Energy.Reactive.Export.Register")]
    EnergyReactiveExportRegister,
    #[serde(rename = "Energy.Reactive.Import.Interval")]
    EnergyReactiveImportInterval,
    #[serde(rename = "Energy.Reactive.Import.Register")]
    EnergyReactiveImportRegister,
    Frequency,
    #[serde(rename = "Power.Active.Export")]
    PowerActiveExport,
    #[serde(rename = "Power.Active.Import")]
    PowerActiveImport,
    #[serde(rename = "Power.Factor")]
    PowerFactor,
    #[serde(rename = "Power.Offered")]
    PowerOffered,
    #[serde(rename = "Power.Reactive.Export")]
    PowerReactiveExport,
    #[serde(rename = "Power.Reactive.Import")]
    PowerReactiveImport,
    #[serde(rename = "RPM")]
    Rpm,
    SoC,
    Temperature,
    Voltage,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Phase {
    L1,
    #[serde(rename = "L1-L2")]
    L1L2,
    #[serde(rename = "L1-N")]
    L1N,
    L2,
    #[serde(rename = "L2-L3")]
    L2L3,
    #[serde(rename = "L2-N")]
    L2N,
    L3,
    #[serde(rename = "L3-L1")]
    L3L1,
    #[serde(rename = "L3-N")]
    L3N,
    N,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Unit {
    A,
    Celcius,
    Celsius,
    Fahrenheit,
    K,
    #[serde(rename = "kVA")]
    KVa,
    #[serde(rename = "kW")]
    KW,
    #[serde(rename = "kWh")]
    KWh,
    #[serde(rename = "kvar")]
    Kvar,
    #[serde(rename = "kvarh")]
    Kvarh,
    Percent,
    V,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "var")]
    Var,
    #[serde(rename = "varh")]
    Varh,
    W,
    Wh,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoteStartTransactionRequest {
    #[serde(rename = "chargingProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile: Option<ChargingProfile>,
    #[serde(rename = "connectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i64>,
    #[serde(rename = "idTag")]
    pub id_tag: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChargingProfile {
    #[serde(rename = "chargingProfileId")]
    pub charging_profile_id: i64,
    #[serde(rename = "chargingProfileKind")]
    pub charging_profile_kind: ChargingProfileKind,
    #[serde(rename = "chargingProfilePurpose")]
    pub charging_profile_purpose: ChargingProfilePurpose,
    #[serde(rename = "chargingSchedule")]
    pub charging_schedule: ChargingSchedule,
    #[serde(rename = "recurrencyKind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrency_kind: Option<RecurrencyKind>,
    #[serde(rename = "stackLevel")]
    pub stack_level: i64,
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<i64>,
    #[serde(rename = "validFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
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
pub enum ChargingProfileKind {
    Absolute,
    Recurring,
    Relative,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RecurrencyKind {
    Daily,
    Weekly,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoteStopTransactionRequest {
    #[serde(rename = "transactionId")]
    pub transaction_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReserveNowRequest {
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
    #[serde(rename = "expiryDate")]
    pub expiry_date: String,
    #[serde(rename = "idTag")]
    pub id_tag: String,
    #[serde(rename = "parentIdTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id_tag: Option<String>,
    #[serde(rename = "reservationId")]
    pub reservation_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResetRequest {
    #[serde(rename = "type")]
    pub reset_request_type: ResetType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResetType {
    Hard,
    Soft,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendLocalListRequest {
    #[serde(rename = "listVersion")]
    pub list_version: i64,
    #[serde(rename = "localAuthorizationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_authorization_list: Option<Vec<LocalAuthorizationList>>,
    #[serde(rename = "updateType")]
    pub update_type: UpdateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalAuthorizationList {
    #[serde(rename = "idTag")]
    pub id_tag: String,
    #[serde(rename = "idTagInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag_info: Option<IdTagInfo>,
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
pub enum UpdateType {
    Differential,
    Full,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetChargingProfileRequest {
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
    #[serde(rename = "csChargingProfiles")]
    pub cs_charging_profiles: CsChargingProfiles,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CsChargingProfiles {
    #[serde(rename = "chargingProfileId")]
    pub charging_profile_id: i64,
    #[serde(rename = "chargingProfileKind")]
    pub charging_profile_kind: ChargingProfileKind,
    #[serde(rename = "chargingProfilePurpose")]
    pub charging_profile_purpose: ChargingProfilePurpose,
    #[serde(rename = "chargingSchedule")]
    pub charging_schedule: ChargingSchedule,
    #[serde(rename = "recurrencyKind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrency_kind: Option<RecurrencyKind>,
    #[serde(rename = "stackLevel")]
    pub stack_level: i64,
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<i64>,
    #[serde(rename = "validFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartTransactionRequest {
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
    #[serde(rename = "idTag")]
    pub id_tag: String,
    #[serde(rename = "meterStart")]
    pub meter_start: i64,
    #[serde(rename = "reservationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<i64>,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusNotificationRequest {
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
    #[serde(rename = "errorCode")]
    pub error_code: ErrorCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    pub status: StatusNotificationStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "vendorErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_error_code: Option<String>,
    #[serde(rename = "vendorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ErrorCode {
    ConnectorLockFailure,
    #[serde(rename = "EVCommunicationError")]
    EvCommunicationError,
    GroundFailure,
    HighTemperature,
    InternalError,
    LocalListConflict,
    NoError,
    OtherError,
    OverCurrentFailure,
    OverVoltage,
    PowerMeterFailure,
    PowerSwitchFailure,
    ReaderFailure,
    ResetFailure,
    UnderVoltage,
    WeakSignal,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StatusNotificationStatus {
    Available,
    Charging,
    Faulted,
    Finishing,
    Preparing,
    Reserved,
    #[serde(rename = "SuspendedEV")]
    SuspendedEv,
    #[serde(rename = "SuspendedEVSE")]
    SuspendedEvse,
    Unavailable,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StopTransactionRequest {
    #[serde(rename = "idTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag: Option<String>,
    #[serde(rename = "meterStop")]
    pub meter_stop: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
    pub timestamp: String,
    #[serde(rename = "transactionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_data: Option<Vec<TransactionDatum>>,
    #[serde(rename = "transactionId")]
    pub transaction_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionDatum {
    #[serde(rename = "sampledValue")]
    pub sampled_value: Vec<SampledValue>,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Reason {
    DeAuthorized,
    EmergencyStop,
    #[serde(rename = "EVDisconnected")]
    EvDisconnected,
    HardReset,
    Local,
    Other,
    PowerLoss,
    Reboot,
    Remote,
    SoftReset,
    UnlockCommand,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TriggerMessageRequest {
    #[serde(rename = "connectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i64>,
    #[serde(rename = "requestedMessage")]
    pub requested_message: RequestedMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestedMessage {
    BootNotification,
    DiagnosticsStatusNotification,
    FirmwareStatusNotification,
    Heartbeat,
    MeterValues,
    StatusNotification,
}

