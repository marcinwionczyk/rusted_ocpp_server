use serde::Deserialize;
use validator_derive::Validate;

const CALL: u8 = 3;

#[derive(Deserialize)]
pub struct AuthorizeRequest {
    #[serde(rename = "idTag")]
    id_tag: String,
}

#[derive(Deserialize)]
pub struct BootNotificationRequest {
    #[serde(rename = "chargeBoxSerialNumber")]
    pub charge_box_serial_number: Option<String>,
    #[serde(rename = "chargePointModel")]
    pub charge_point_model: String,
    #[serde(rename = "chargePointSerialNumber")]
    pub charge_point_serial_number: Option<String>,
    #[serde(rename = "chargePointVendor")]
    pub charge_point_vendor: String,
    #[serde(rename = "firmwareVersion")]
    pub firmware_version: Option<String>,
    pub iccid: Option<String>,
    pub imsi: Option<String>,
    #[serde(rename = "meterSerialNumber")]
    pub meter_serial_number: Option<String>,
    #[serde(rename = "meterType")]
    pub meter_type: Option<String>,
}

#[derive(Deserialize)]
pub struct CancelReservationRequest {
    #[serde(rename = "reservationId")]
    pub reservation_id: i64,
}

#[derive(Deserialize)]
pub struct ChangeAvailabilityRequest {
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
    #[serde(rename = "type")]
    pub change_availability_request_type: ChangeAvailabilityRequestType,
}


#[derive(Deserialize)]
pub struct ChangeConfigurationRequest {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct ClearCacheRequest {}

#[derive(Deserialize)]
pub struct ClearChargingProfileRequest {
    #[serde(rename = "chargingProfilePurpose")]
    pub charging_profile_purpose: Option<ChargingProfilePurpose>,
    #[serde(rename = "connectorId")]
    pub connector_id: Option<i64>,
    pub id: Option<i64>,
    #[serde(rename = "stackLevel")]
    pub stack_level: Option<i64>,
}

#[derive(Deserialize)]
pub struct DataTransferRequest {
    pub data: Option<String>,
    #[serde(rename = "messageId")]
    pub message_id: Option<String>,
    #[serde(rename = "vendorId")]
    pub vendor_id: String,
}

#[derive(Deserialize)]
pub struct DiagnosticsStatusNotificationRequest {
    pub status: DiagnosticsStatusNotificationRequestStatus,
}

#[derive(Deserialize)]
pub enum DiagnosticsStatusNotificationRequestStatus {
    Idle,
    UploadFailed,
    Uploaded,
    Uploading,
}

#[derive(Deserialize)]
pub struct FirmwareStatusNotificationRequest {
    pub status: FirmwareStatusNotificationRequestStatus,
}


#[derive(Deserialize)]
pub struct GetCompositeScheduleRequest {
    #[serde(rename = "chargingRateUnit")]
    pub charging_rate_unit: Option<ChargingRateUnit>,
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
    pub duration: i64,
}

#[derive(Deserialize)]
pub struct GetConfigurationRequest {
    pub key: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct GetDiagnosticsRequest {
    pub location: String,
    pub retries: Option<i64>,
    #[serde(rename = "retryInterval")]
    pub retry_interval: Option<i64>,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(rename = "stopTime")]
    pub stop_time: Option<String>,
}

#[derive(Deserialize)]
pub struct GetLocalListVersionRequest {}

#[derive(Deserialize)]
pub struct HeartbeatRequest {}

#[derive(Deserialize)]
pub struct MeterValuesRequest {
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
    #[serde(rename = "meterValue")]
    pub meter_value: Vec<MeterValue>,
    #[serde(rename = "transactionId")]
    pub transaction_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct MeterValue {
    #[serde(rename = "sampledValue")]
    pub sampled_value: Vec<SampledValue>,
    pub timestamp: String,
}

#[derive(Deserialize)]
pub struct SampledValue {
    pub context: Option<Context>,
    pub format: Option<Format>,
    pub location: Option<Location>,
    pub measurand: Option<Measurand>,
    pub phase: Option<Phase>,
    pub unit: Option<Unit>,
    pub value: String,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub enum Format {
    Raw,
    SignedData,
}

#[derive(Deserialize)]
pub enum Location {
    Body,
    Cable,
    #[serde(rename = "EV")]
    Ev,
    Inlet,
    Outlet,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct RemoteStartTransactionRequest {
    #[serde(rename = "chargingProfile")]
    pub charging_profile: Option<ChargingProfile>,
    #[serde(rename = "connectorId")]
    pub connector_id: Option<i64>,
    #[serde(rename = "idTag")]
    pub id_tag: String,
}

#[derive(Deserialize)]
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
    pub recurrency_kind: Option<RecurrencyKind>,
    #[serde(rename = "stackLevel")]
    pub stack_level: i64,
    #[serde(rename = "transactionId")]
    pub transaction_id: Option<i64>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct ChargingSchedulePeriod {
    pub limit: f64,
    #[serde(rename = "numberPhases")]
    pub number_phases: Option<i64>,
    #[serde(rename = "startPeriod")]
    pub start_period: i64,
}

#[derive(Deserialize)]
pub struct RemoteStopTransactionRequest {
    #[serde(rename = "transactionId")]
    pub transaction_id: i64,
}

#[derive(Deserialize)]
pub struct ReserveNowRequest {
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
    #[serde(rename = "expiryDate")]
    pub expiry_date: String,
    #[serde(rename = "idTag")]
    pub id_tag: String,
    #[serde(rename = "parentIdTag")]
    pub parent_id_tag: Option<String>,
    #[serde(rename = "reservationId")]
    pub reservation_id: i64,
}

#[derive(Deserialize)]
pub struct ResetRequest {
    #[serde(rename = "type")]
    pub reset_request_type: ResetType,
}

#[derive(Deserialize)]
pub struct SendLocalListRequest {
    #[serde(rename = "listVersion")]
    pub list_version: i64,
    #[serde(rename = "localAuthorizationList")]
    pub local_authorization_list: Option<Vec<LocalAuthorizationList>>,
    #[serde(rename = "updateType")]
    pub update_type: UpdateType,
}

#[derive(Deserialize)]
pub struct LocalAuthorizationList {
    #[serde(rename = "idTag")]
    pub id_tag: String,
    #[serde(rename = "idTagInfo")]
    pub id_tag_info: Option<IdTagInfo>,
}

#[derive(Deserialize)]
pub struct IdTagInfo {
    #[serde(rename = "expiryDate")]
    pub expiry_date: Option<String>,
    #[serde(rename = "parentIdTag")]
    pub parent_id_tag: Option<String>,
    pub status: IdTagInfoStatus,
}

#[derive(Deserialize)]
pub struct SetChargingProfileRequest {
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
    #[serde(rename = "csChargingProfiles")]
    pub cs_charging_profiles: CsChargingProfiles,
}

#[derive(Deserialize)]
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
    pub recurrency_kind: Option<RecurrencyKind>,
    #[serde(rename = "stackLevel")]
    pub stack_level: i64,
    #[serde(rename = "transactionId")]
    pub transaction_id: Option<i64>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
}

#[derive(Deserialize)]
pub struct StartTransactionRequest {
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
    #[serde(rename = "idTag")]
    pub id_tag: String,
    #[serde(rename = "meterStart")]
    pub meter_start: i64,
    #[serde(rename = "reservationId")]
    pub reservation_id: Option<i64>,
    pub timestamp: String,
}

#[derive(Deserialize)]
pub struct StatusNotificationRequest {
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
    #[serde(rename = "errorCode")]
    pub error_code: ErrorCode,
    pub info: Option<String>,
    pub status: Status,
    pub timestamp: Option<String>,
    #[serde(rename = "vendorErrorCode")]
    pub vendor_error_code: Option<String>,
    #[serde(rename = "vendorId")]
    pub vendor_id: Option<String>,
}

#[derive(Deserialize)]
pub struct StopTransactionRequest {
    #[serde(rename = "idTag")]
    pub id_tag: Option<String>,
    #[serde(rename = "meterStop")]
    pub meter_stop: i64,
    pub reason: Option<Reason>,
    pub timestamp: String,
    #[serde(rename = "transactionData")]
    pub transaction_data: Option<Vec<TransactionDatum>>,
    #[serde(rename = "transactionId")]
    pub transaction_id: i64,
}

#[derive(Deserialize)]
pub struct TransactionDatum {
    #[serde(rename = "sampledValue")]
    pub sampled_value: Vec<SampledValue>,
    pub timestamp: String,
}

#[derive(Deserialize)]
pub struct TriggerMessageRequest {
    #[serde(rename = "connectorId")]
    pub connector_id: Option<i64>,
    #[serde(rename = "requestedMessage")]
    pub requested_message: RequestedMessage,
}

#[derive(Deserialize)]
pub struct UnlockConnectorRequest {
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
}

#[derive(Deserialize)]
pub struct UpdateFirmwareRequest {
    pub location: String,
    pub retries: Option<i64>,
    #[serde(rename = "retrieveDate")]
    pub retrieve_date: String,
    #[serde(rename = "retryInterval")]
    pub retry_interval: Option<i64>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize)]
pub enum RequestedMessage {
    BootNotification,
    DiagnosticsStatusNotification,
    FirmwareStatusNotification,
    Heartbeat,
    MeterValues,
    StatusNotification,
}

#[derive(Deserialize)]
pub enum ChangeAvailabilityRequestType {
    Inoperative,
    Operative,
}


#[derive(Deserialize)]
pub enum ChargingProfileKind {
    Absolute,
    Recurring,
    Relative,
}

#[derive(Deserialize)]
pub enum ChargingProfilePurpose {
    ChargePointMaxProfile,
    TxDefaultProfile,
    TxProfile,
}

#[derive(Deserialize)]
pub enum ChargingRateUnit {
    A,
    W,
}

#[derive(Deserialize)]
pub enum RecurrencyKind {
    Daily,
    Weekly,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub enum FirmwareStatusNotificationRequestStatus {
    DownloadFailed,
    Downloaded,
    Downloading,
    Idle,
    InstallationFailed,
    Installed,
    Installing,
}

#[derive(Deserialize)]
pub enum ResetType {
    Hard,
    Soft,
}

#[derive(Deserialize)]
pub enum IdTagInfoStatus {
    Accepted,
    Blocked,
    ConcurrentTx,
    Expired,
    Invalid,
}

#[derive(Deserialize)]
pub enum UpdateType {
    Differential,
    Full,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub enum Status {
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

#[derive(Deserialize)]
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
