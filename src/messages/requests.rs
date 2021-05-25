use serde::{Serialize, Deserialize};

// structures created with the help of https://app.quicktype.io/ and json schema provided by
// https://www.openchargealliance.org/

#[derive(Serialize, Deserialize)]
pub struct AuthorizeRequest {
    /// The X.509 certificated presented by EV and encoded in PEM format.
    pub certificate: Option<String>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "idToken")]
    pub id_token: IdTokenType,
    #[serde(rename = "iso15118CertificateHashData")]
    pub iso15118_certificate_hash_data: Option<Vec<OcspRequestDataType>>,
}

/// This class does not get 'AdditionalProperties = false' in the schema generation, so it
/// can be extended with arbitrary JSON properties to allow adding custom data.
#[derive(Serialize, Deserialize)]
pub struct CustomDataType {
    #[serde(rename = "vendorId")]
    pub vendor_id: String,
}

/// Contains a case insensitive identifier to use for the authorization and the type of
/// authorization to support multiple forms of identifiers.
#[derive(Serialize, Deserialize)]
pub struct IdTokenType {
    #[serde(rename = "additionalInfo")]
    pub additional_info: Option<Vec<AdditionalInfoType>>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// IdToken is case insensitive. Might hold the hidden id of an RFID tag, but can for example
    /// also contain a UUID.
    #[serde(rename = "idToken")]
    pub id_token: String,
    #[serde(rename = "type")]
    pub id_token_type_type: IdTokenEnumType,
}

/// Contains a case insensitive identifier to use for the authorization and the type of
/// authorization to support multiple forms of identifiers.
#[derive(Serialize, Deserialize)]
pub struct AdditionalInfoType {
    /// This field specifies the additional IdToken.
    #[serde(rename = "additionalIdToken")]
    pub additional_id_token: String,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// This defines the type of the additionalIdToken. This is a custom type, so the
    /// implementation needs to be agreed upon by all involved parties.
    #[serde(rename = "type")]
    pub additional_info_type_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct OcspRequestDataType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "hashAlgorithm")]
    pub hash_algorithm: HashAlgorithmEnumType,
    /// Hashed value of the issuers public key
    #[serde(rename = "issuerKeyHash")]
    pub issuer_key_hash: String,
    /// Hashed value of the Issuer DN (Distinguished Name).
    #[serde(rename = "issuerNameHash")]
    pub issuer_name_hash: String,
    /// This contains the responder URL (Case insensitive).
    #[serde(rename = "responderURL")]
    pub responder_url: String,
    /// The serial number of the certificate.
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
}

/// Enumeration of possible idToken types.
#[derive(Serialize, Deserialize)]
pub enum IdTokenEnumType {
    Central,
    #[serde(rename = "eMAID")]
    EMaid,
    #[serde(rename = "ISO14443")]
    Iso14443,
    #[serde(rename = "ISO15693")]
    Iso15693,
    KeyCode,
    Local,
    MacAddress,
    NoAuthorization,
}

/// Used algorithms for the hashes provided.
#[derive(Serialize, Deserialize)]
pub enum HashAlgorithmEnumType {
    #[serde(rename = "SHA256")]
    Sha256,
    #[serde(rename = "SHA384")]
    Sha384,
    #[serde(rename = "SHA512")]
    Sha512,
}

//=================================================================================================
#[derive(Serialize, Deserialize)]
pub struct BootNotificationRequest {
    #[serde(rename = "chargingStation")]
    pub charging_station: ChargingStationType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    pub reason: BootReasonEnumType,
}

/// Charge_ Point
/// urn:x-oca:ocpp:uid:2:233122
/// The physical system where an Electrical Vehicle (EV) can be charged.
#[derive(Serialize, Deserialize)]
pub struct ChargingStationType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// This contains the firmware version of the Charging Station.
    #[serde(rename = "firmwareVersion")]
    pub firmware_version: Option<String>,
    /// Device. Model. CI20_ Text
    /// urn:x-oca:ocpp:uid:1:569325
    /// Defines the model of the device.
    pub model: String,
    pub modem: Option<ModemType>,
    /// Device. Serial_ Number. Serial_ Number
    /// urn:x-oca:ocpp:uid:1:569324
    /// Vendor-specific device identifier.
    #[serde(rename = "serialNumber")]
    pub serial_number: Option<String>,
    /// Identifies the vendor (not necessarily in a unique manner).
    #[serde(rename = "vendorName")]
    pub vendor_name: String,
}

/// This class does not get 'AdditionalProperties = false' in the schema generation, so it
/// can be extended with arbitrary JSON properties to allow adding custom data.


/// Wireless_ Communication_ Module
/// urn:x-oca:ocpp:uid:2:233306
/// Defines parameters required for initiating and maintaining wireless communication with
/// other devices.
#[derive(Serialize, Deserialize)]
pub struct ModemType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Wireless_ Communication_ Module. ICCID. CI20_ Text
    /// urn:x-oca:ocpp:uid:1:569327
    /// This contains the ICCID of the modem’s SIM card.
    pub iccid: Option<String>,
    /// Wireless_ Communication_ Module. IMSI. CI20_ Text
    /// urn:x-oca:ocpp:uid:1:569328
    /// This contains the IMSI of the modem’s SIM card.
    pub imsi: Option<String>,
}

/// This contains the reason for sending this message to the CSMS.
#[derive(Serialize, Deserialize)]
pub enum BootReasonEnumType {
    ApplicationReset,
    FirmwareUpdate,
    LocalReset,
    PowerUp,
    RemoteReset,
    ScheduledReset,
    Triggered,
    Unknown,
    Watchdog,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct CancelReservationRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Id of the reservation to cancel.
    #[serde(rename = "reservationId")]
    pub reservation_id: i64,
}

//=================================================================================================
#[derive(Serialize, Deserialize)]
pub struct CertificateSignedRequest {
    /// The signed PEM encoded X.509 certificate. This can also contain the necessary sub CA
    /// certificates. In that case, the order of the bundle should follow the certificate chain,
    /// starting from the leaf certificate.
    ///
    /// The Configuration Variable
    /// &lt;&lt;configkey-max-certificate-chain-size,MaxCertificateChainSize&gt;&gt; can be used
    /// to limit the maximum size of this field.
    #[serde(rename = "certificateChain")]
    pub certificate_chain: String,
    #[serde(rename = "certificateType")]
    pub certificate_type: Option<CertificateSigningUseEnumType>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
}

/// Indicates the type of the signed certificate that is returned. When omitted the
/// certificate is used for both the 15118 connection (if implemented) and the Charging
/// Station to CSMS connection. This field is required when a typeOfCertificate was included
/// in the &lt;&lt;signcertificaterequest,SignCertificateRequest&gt;&gt; that requested this
/// certificate to be signed AND both the 15118 connection and the Charging Station
/// connection are implemented.
#[derive(Serialize, Deserialize)]
pub enum CertificateSigningUseEnumType {
    ChargingStationCertificate,
    V2GCertificate,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ChangeAvailabilityRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    pub evse: Option<EvseType>,
    #[serde(rename = "operationalStatus")]
    pub operational_status: OperationalStatusEnumType,
}

/// EVSE
/// urn:x-oca:ocpp:uid:2:233123
/// Electric Vehicle Supply Equipment
#[derive(Serialize, Deserialize)]
pub struct EvseType {
    /// An id to designate a specific connector (on an EVSE) by connector index number.
    #[serde(rename = "connectorId")]
    pub connector_id: Option<i64>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Identified_ Object. MRID. Numeric_ Identifier
    /// urn:x-enexis:ecdm:uid:1:569198
    /// EVSE Identifier. This contains a number (&gt; 0) designating an EVSE of the Charging
    /// Station.
    pub id: i64,
}

/// This contains the type of availability change that the Charging Station should perform.
#[derive(Serialize, Deserialize)]
pub enum OperationalStatusEnumType {
    Inoperative,
    Operative,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ClearCacheRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ClearChargingProfileRequest {
    #[serde(rename = "chargingProfileCriteria")]
    pub charging_profile_criteria: Option<ClearChargingProfileType>,
    /// The Id of the charging profile to clear.
    #[serde(rename = "chargingProfileId")]
    pub charging_profile_id: Option<i64>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
}

/// Charging_ Profile
/// urn:x-oca:ocpp:uid:2:233255
/// A ChargingProfile consists of a ChargingSchedule, describing the amount of power or
/// current that can be delivered per time interval.
#[derive(Serialize, Deserialize)]
pub struct ClearChargingProfileType {
    #[serde(rename = "chargingProfilePurpose")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Identified_ Object. MRID. Numeric_ Identifier
    /// urn:x-enexis:ecdm:uid:1:569198
    /// Specifies the id of the EVSE for which to clear charging profiles. An evseId of zero (0)
    /// specifies the charging profile for the overall Charging Station. Absence of this
    /// parameter means the clearing applies to all charging profiles that match the other
    /// criteria in the request.
    #[serde(rename = "evseId")]
    pub evse_id: Option<i64>,
    /// Charging_ Profile. Stack_ Level. Counter
    /// urn:x-oca:ocpp:uid:1:569230
    /// Specifies the stackLevel for which charging profiles will be cleared, if they meet the
    /// other criteria in the request.
    #[serde(rename = "stackLevel")]
    pub stack_level: Option<i64>,
}

/// Charging_ Profile. Charging_ Profile_ Purpose. Charging_ Profile_ Purpose_ Code
/// urn:x-oca:ocpp:uid:1:569231
/// Specifies to purpose of the charging profiles that will be cleared, if they meet the
/// other criteria in the request.
#[derive(Serialize, Deserialize)]
pub enum ChargingProfilePurposeEnumType {
    ChargingStationExternalConstraints,
    ChargingStationMaxProfile,
    TxDefaultProfile,
    TxProfile,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ClearDisplayMessageRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Id of the message that SHALL be removed from the Charging Station.
    pub id: i64,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ClearedChargingLimitRequest {
    #[serde(rename = "chargingLimitSource")]
    pub charging_limit_source: ChargingLimitSourceEnumType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// EVSE Identifier.
    #[serde(rename = "evseId")]
    pub evse_id: Option<i64>,
}

/// Source of the charging limit.
#[derive(Serialize, Deserialize)]
pub enum ChargingLimitSourceEnumType {
    #[serde(rename = "CSO")]
    Cso,
    #[serde(rename = "EMS")]
    Ems,
    Other,
    #[serde(rename = "SO")]
    So,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ClearVariableMonitoringRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// List of the monitors to be cleared, identified by there Id.
    pub id: Vec<i64>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct CostUpdatedRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Current total cost, based on the information known by the CSMS, of the transaction
    /// including taxes. In the currency configured with the configuration Variable:
    /// [&lt;&lt;configkey-currency, Currency&gt;&gt;]
    #[serde(rename = "totalCost")]
    pub total_cost: f64,
    /// Transaction Id of the transaction the current cost are asked for.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct CustomerInformationRequest {
    /// Flag indicating whether the Charging Station should clear all information about the
    /// customer referred to.
    pub clear: bool,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "customerCertificate")]
    pub customer_certificate: Option<CertificateHashDataType>,
    /// A (e.g. vendor specific) identifier of the customer this request refers to. This field
    /// contains a custom identifier other than IdToken and Certificate.
    /// One of the possible identifiers (customerIdentifier, customerIdToken or
    /// customerCertificate) should be in the request message.
    #[serde(rename = "customerIdentifier")]
    pub customer_identifier: Option<String>,
    #[serde(rename = "idToken")]
    pub id_token: Option<IdTokenType>,
    /// Flag indicating whether the Charging Station should return
    /// NotifyCustomerInformationRequest messages containing information about the customer
    /// referred to.
    pub report: bool,
    /// The Id of the request.
    #[serde(rename = "requestId")]
    pub request_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CertificateHashDataType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "hashAlgorithm")]
    pub hash_algorithm: HashAlgorithmEnumType,
    /// Hashed value of the issuers public key
    #[serde(rename = "issuerKeyHash")]
    pub issuer_key_hash: String,
    /// Hashed value of the Issuer DN (Distinguished Name).
    #[serde(rename = "issuerNameHash")]
    pub issuer_name_hash: String,
    /// The serial number of the certificate.
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct DataTransferRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Data without specified length or format. This needs to be decided by both parties (Open
    /// to implementation).
    pub data: Option<serde_json::Value>,
    /// May be used to indicate a specific message or implementation.
    #[serde(rename = "messageId")]
    pub message_id: Option<String>,
    /// This identifies the Vendor specific implementation
    #[serde(rename = "vendorId")]
    pub vendor_id: String,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct DeleteCertificateRequest {
    #[serde(rename = "certificateHashData")]
    pub certificate_hash_data: CertificateHashDataType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct FirmwareStatusNotificationRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// The request id that was provided in the
    /// UpdateFirmwareRequest that started this firmware update.
    /// This field is mandatory, unless the message was triggered by a TriggerMessageRequest AND
    /// there is no firmware update ongoing.
    #[serde(rename = "requestId")]
    pub request_id: Option<i64>,
    pub status: FirmwareStatusEnumType,
}

/// This contains the progress status of the firmware installation.
#[derive(Serialize, Deserialize)]
pub enum FirmwareStatusEnumType {
    DownloadFailed,
    DownloadPaused,
    DownloadScheduled,
    Downloaded,
    Downloading,
    Idle,
    InstallRebooting,
    InstallScheduled,
    InstallVerificationFailed,
    InstallationFailed,
    Installed,
    Installing,
    InvalidSignature,
    SignatureVerified,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct Get15118EvCertificateRequest {
    pub action: CertificateActionEnumType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Raw CertificateInstallationReq request from EV, Base64 encoded.
    #[serde(rename = "exiRequest")]
    pub exi_request: String,
    /// Schema version currently used for the 15118 session between EV and Charging Station.
    /// Needed for parsing of the EXI stream by the CSMS.
    #[serde(rename = "iso15118SchemaVersion")]
    pub iso15118_schema_version: String,
}

/// Defines whether certificate needs to be installed or updated.
#[derive(Serialize, Deserialize)]
pub enum CertificateActionEnumType {
    Install,
    Update,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetBaseReportRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "reportBase")]
    pub report_base: ReportBaseEnumType,
    /// The Id of the request.
    #[serde(rename = "requestId")]
    pub request_id: i64,
}

/// This field specifies the report base.
#[derive(Serialize, Deserialize)]
pub enum ReportBaseEnumType {
    ConfigurationInventory,
    FullInventory,
    SummaryInventory,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetCertificateStatusRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "ocspRequestData")]
    pub ocsp_request_data: OcspRequestDataType,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetChargingProfilesRequest {
    #[serde(rename = "chargingProfile")]
    pub charging_profile: ChargingProfileCriterionType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// For which EVSE installed charging profiles SHALL be reported. If 0, only charging
    /// profiles installed on the Charging Station itself (the grid connection) SHALL be
    /// reported. If omitted, all installed charging profiles SHALL be reported.
    #[serde(rename = "evseId")]
    pub evse_id: Option<i64>,
    /// Reference identification that is to be used by the Charging Station in the
    /// &lt;&lt;reportchargingprofilesrequest, ReportChargingProfilesRequest&gt;&gt; when
    /// provided.
    #[serde(rename = "requestId")]
    pub request_id: i64,
}

/// Charging_ Profile
/// urn:x-oca:ocpp:uid:2:233255
/// A ChargingProfile consists of ChargingSchedule, describing the amount of power or current
/// that can be delivered per time interval.
#[derive(Serialize, Deserialize)]
pub struct ChargingProfileCriterionType {
    /// For which charging limit sources, charging profiles SHALL be reported. If omitted, the
    /// Charging Station SHALL not filter on chargingLimitSource.
    #[serde(rename = "chargingLimitSource")]
    pub charging_limit_source: Option<Vec<ChargingLimitSourceEnumType>>,
    /// List of all the chargingProfileIds requested. Any ChargingProfile that matches one of
    /// these profiles will be reported. If omitted, the Charging Station SHALL not filter on
    /// chargingProfileId. This field SHALL NOT contain more ids than set in
    /// &lt;&lt;configkey-charging-profile-entries,ChargingProfileEntries.maxLimit&gt;&gt;
    #[serde(rename = "chargingProfileId")]
    pub charging_profile_id: Option<Vec<i64>>,
    #[serde(rename = "chargingProfilePurpose")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Charging_ Profile. Stack_ Level. Counter
    /// urn:x-oca:ocpp:uid:1:569230
    /// Value determining level in hierarchy stack of profiles. Higher values have precedence
    /// over lower values. Lowest level is 0.
    #[serde(rename = "stackLevel")]
    pub stack_level: Option<i64>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetCompositeScheduleRequest {
    #[serde(rename = "chargingRateUnit")]
    pub charging_rate_unit: Option<ChargingRateUnitEnumType>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Length of the requested schedule in seconds.
    pub duration: i64,
    /// The ID of the EVSE for which the schedule is requested. When evseid=0, the Charging
    /// Station will calculate the expected consumption for the grid connection.
    #[serde(rename = "evseId")]
    pub evse_id: i64,
}

/// Can be used to force a power or current profile.
#[derive(Serialize, Deserialize)]
pub enum ChargingRateUnitEnumType {
    A,
    W,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetDisplayMessagesRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// If provided the Charging Station shall return Display Messages of the given ids. This
    /// field SHALL NOT contain more ids than set in
    /// &lt;&lt;configkey-number-of-display-messages,NumberOfDisplayMessages.maxLimit&gt;&gt;
    pub id: Option<Vec<i64>>,
    pub priority: Option<MessagePriorityEnumType>,
    /// The Id of this request.
    #[serde(rename = "requestId")]
    pub request_id: i64,
    pub state: Option<MessageStateEnumType>,
}

/// If provided the Charging Station shall return Display Messages with the given priority
/// only.
#[derive(Serialize, Deserialize)]
pub enum MessagePriorityEnumType {
    AlwaysFront,
    InFront,
    NormalCycle,
}

/// If provided the Charging Station shall return Display Messages with the given state only.
#[derive(Serialize, Deserialize)]
pub enum MessageStateEnumType {
    Charging,
    Faulted,
    Idle,
    Unavailable,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetInstalledCertificateIdsRequest {
    /// Indicates the type of certificates requested. When omitted, all certificate types are
    /// requested.
    #[serde(rename = "certificateType")]
    pub certificate_type: Option<Vec<GetCertificateIdUseEnumType>>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Serialize, Deserialize)]
pub enum GetCertificateIdUseEnumType {
    #[serde(rename = "CSMSRootCertificate")]
    CsmsRootCertificate,
    ManufacturerRootCertificate,
    #[serde(rename = "MORootCertificate")]
    MoRootCertificate,
    V2GCertificateChain,
    V2GRootCertificate,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetLocalListVersionRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetLogRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    pub log: LogParametersType,
    #[serde(rename = "logType")]
    pub log_type: LogEnumType,
    /// The Id of this request
    #[serde(rename = "requestId")]
    pub request_id: i64,
    /// This specifies how many times the Charging Station must try to upload the log before
    /// giving up. If this field is not present, it is left to Charging Station to decide how
    /// many times it wants to retry.
    pub retries: Option<i64>,
    /// The interval in seconds after which a retry may be attempted. If this field is not
    /// present, it is left to Charging Station to decide how long to wait between attempts.
    #[serde(rename = "retryInterval")]
    pub retry_interval: Option<i64>,
}

/// Log
/// urn:x-enexis:ecdm:uid:2:233373
/// Generic class for the configuration of logging entries.
#[derive(Serialize, Deserialize)]
pub struct LogParametersType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Log. Latest_ Timestamp. Date_ Time
    /// urn:x-enexis:ecdm:uid:1:569482
    /// This contains the date and time of the latest logging information to include in the
    /// diagnostics.
    #[serde(rename = "latestTimestamp")]
    pub latest_timestamp: Option<String>,
    /// Log. Oldest_ Timestamp. Date_ Time
    /// urn:x-enexis:ecdm:uid:1:569477
    /// This contains the date and time of the oldest logging information to include in the
    /// diagnostics.
    #[serde(rename = "oldestTimestamp")]
    pub oldest_timestamp: Option<String>,
    /// Log. Remote_ Location. URI
    /// urn:x-enexis:ecdm:uid:1:569484
    /// The URL of the location at the remote system where the log should be stored.
    #[serde(rename = "remoteLocation")]
    pub remote_location: String,
}

/// This contains the type of log file that the Charging Station
/// should send.
#[derive(Serialize, Deserialize)]
pub enum LogEnumType {
    DiagnosticsLog,
    SecurityLog,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetMonitoringReportRequest {
    #[serde(rename = "componentVariable")]
    pub component_variable: Option<Vec<ComponentVariableType>>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// This field contains criteria for components for which a monitoring report is requested
    #[serde(rename = "monitoringCriteria")]
    pub monitoring_criteria: Option<Vec<MonitoringCriterionEnumType>>,
    /// The Id of the request.
    #[serde(rename = "requestId")]
    pub request_id: i64,
}

/// Class to report components, variables and variable attributes and characteristics.
#[derive(Serialize, Deserialize)]
pub struct ComponentVariableType {
    pub component: ComponentType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    pub variable: Option<VariableType>,
}

/// A physical or logical component
#[derive(Serialize, Deserialize)]
pub struct ComponentType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    pub evse: Option<EvseType>,
    /// Name of instance in case the component exists as multiple instances. Case Insensitive.
    /// strongly advised to use Camel Case.
    pub instance: Option<String>,
    /// Name of the component. Name should be taken from the list of standardized component names
    /// whenever possible. Case Insensitive. strongly advised to use Camel Case.
    pub name: String,
}

/// Reference key to a component-variable.
#[derive(Serialize, Deserialize)]
pub struct VariableType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Name of instance in case the variable exists as multiple instances. Case Insensitive.
    /// strongly advised to use Camel Case.
    pub instance: Option<String>,
    /// Name of the variable. Name should be taken from the list of standardized variable names
    /// whenever possible. Case Insensitive. strongly advised to use Camel Case.
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub enum MonitoringCriterionEnumType {
    DeltaMonitoring,
    PeriodicMonitoring,
    ThresholdMonitoring,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetReportRequest {
    /// This field contains criteria for components for which a report is requested
    #[serde(rename = "componentCriteria")]
    pub component_criteria: Option<Vec<ComponentCriterionEnumType>>,
    #[serde(rename = "componentVariable")]
    pub component_variable: Option<Vec<ComponentVariableType>>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// The Id of the request.
    #[serde(rename = "requestId")]
    pub request_id: i64,
}

#[derive(Serialize, Deserialize)]
pub enum ComponentCriterionEnumType {
    Active,
    Available,
    Enabled,
    Problem,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetTransactionStatusRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// The Id of the transaction for which the status is requested.
    #[serde(rename = "transactionId")]
    pub transaction_id: Option<String>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetVariablesRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "getVariableData")]
    pub get_variable_data: Vec<GetVariableDataType>,
}

/// Class to hold parameters for GetVariables request.
#[derive(Serialize, Deserialize)]
pub struct GetVariableDataType {
    #[serde(rename = "attributeType")]
    pub attribute_type: Option<AttributeEnumType>,
    pub component: ComponentType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    pub variable: VariableType,
}

/// Attribute type for which value is requested. When absent, default Actual is assumed.
#[derive(Serialize, Deserialize)]
pub enum AttributeEnumType {
    Actual,
    MaxSet,
    MinSet,
    Target,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct HeartbeatRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct InstallCertificateRequest {
    /// A PEM encoded X.509 certificate.
    pub certificate: String,
    #[serde(rename = "certificateType")]
    pub certificate_type: InstallCertificateUseEnumType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
}

/// Indicates the certificate type that is sent.
#[derive(Serialize, Deserialize)]
pub enum InstallCertificateUseEnumType {
    #[serde(rename = "CSMSRootCertificate")]
    CsmsRootCertificate,
    ManufacturerRootCertificate,
    #[serde(rename = "MORootCertificate")]
    MoRootCertificate,
    V2GRootCertificate,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct LogStatusNotificationRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// The request id that was provided in GetLogRequest that started this log upload. This
    /// field is mandatory,
    /// unless the message was triggered by a TriggerMessageRequest AND there is no log upload
    /// ongoing.
    #[serde(rename = "requestId")]
    pub request_id: Option<i64>,
    pub status: UploadLogStatusEnumType,
}

/// This contains the status of the log upload.
#[derive(Serialize, Deserialize)]
pub enum UploadLogStatusEnumType {
    AcceptedCanceled,
    BadMessage,
    Idle,
    NotSupportedOperation,
    PermissionDenied,
    UploadFailure,
    Uploaded,
    Uploading,
}

//=================================================================================================

/// Request_ Body
/// urn:x-enexis:ecdm:uid:2:234744
#[derive(Serialize, Deserialize)]
pub struct MeterValuesRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Request_ Body. EVSEID. Numeric_ Identifier
    /// urn:x-enexis:ecdm:uid:1:571101
    /// This contains a number (&gt;0) designating an EVSE of the Charging Station. ‘0’ (zero) is
    /// used to designate the main power meter.
    #[serde(rename = "evseId")]
    pub evse_id: i64,
    #[serde(rename = "meterValue")]
    pub meter_value: Vec<MeterValueType>,
}

/// Meter_ Value
/// urn:x-oca:ocpp:uid:2:233265
/// Collection of one or more sampled values in MeterValuesRequest and TransactionEvent. All
/// sampled values in a MeterValue are sampled at the same point in time.
#[derive(Serialize, Deserialize)]
pub struct MeterValueType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "sampledValue")]
    pub sampled_value: Vec<SampledValueType>,
    /// Meter_ Value. Timestamp. Date_ Time
    /// urn:x-oca:ocpp:uid:1:569259
    /// Timestamp for measured value(s).
    pub timestamp: String,
}

/// Sampled_ Value
/// urn:x-oca:ocpp:uid:2:233266
/// Single sampled value in MeterValues. Each value can be accompanied by optional fields.
///
/// To save on mobile data usage, default values of all of the optional fields are such that.
/// The value without any additional fields will be interpreted, as a register reading of
/// active import energy in Wh (Watt-hour) units.
#[derive(Serialize, Deserialize)]
pub struct SampledValueType {
    pub context: Option<ReadingContextEnumType>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    pub location: Option<LocationEnumType>,
    pub measurand: Option<MeasurandEnumType>,
    pub phase: Option<PhaseEnumType>,
    #[serde(rename = "signedMeterValue")]
    pub signed_meter_value: Option<SignedMeterValueType>,
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: Option<UnitOfMeasureType>,
    /// Sampled_ Value. Value. Measure
    /// urn:x-oca:ocpp:uid:1:569260
    /// Indicates the measured value.
    pub value: f64,
}

/// Represent a signed version of the meter value.
#[derive(Serialize, Deserialize)]
pub struct SignedMeterValueType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Method used to encode the meter values before applying the digital signature algorithm.
    #[serde(rename = "encodingMethod")]
    pub encoding_method: String,
    /// Base64 encoded, sending depends on configuration variable _PublicKeyWithSignedMeterValue_.
    #[serde(rename = "publicKey")]
    pub public_key: String,
    /// Base64 encoded, contains the signed data which might contain more then just the meter
    /// value. It can contain information like timestamps, reference to a customer etc.
    #[serde(rename = "signedMeterData")]
    pub signed_meter_data: String,
    /// Method used to create the digital signature.
    #[serde(rename = "signingMethod")]
    pub signing_method: String,
}

/// Represents a UnitOfMeasure with a multiplier
#[derive(Serialize, Deserialize)]
pub struct UnitOfMeasureType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Multiplier, this value represents the exponent to base 10. I.e. multiplier 3 means 10
    /// raised to the 3rd power. Default is 0.
    pub multiplier: Option<i64>,
    /// Unit of the value. Default = "Wh" if the (default) measurand is an "Energy" type.
    /// This field SHALL use a value from the list Standardized Units of Measurements in Part 2
    /// Appendices.
    /// If an applicable unit is available in that list, otherwise a "custom" unit might be used.
    pub unit: Option<String>,
}

/// Sampled_ Value. Context. Reading_ Context_ Code
/// urn:x-oca:ocpp:uid:1:569261
/// Type of detail value: start, end or sample. Default = "Sample.Periodic"
#[derive(Serialize, Deserialize)]
pub enum ReadingContextEnumType {
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

/// Sampled_ Value. Location. Location_ Code
/// urn:x-oca:ocpp:uid:1:569265
/// Indicates where the measured value has been sampled. Default =  "Outlet"
#[derive(Serialize, Deserialize)]
pub enum LocationEnumType {
    Body,
    Cable,
    #[serde(rename = "EV")]
    Ev,
    Inlet,
    Outlet,
}

/// Sampled_ Value. Measurand. Measurand_ Code
/// urn:x-oca:ocpp:uid:1:569263
/// Type of measurement. Default = "Energy.Active.Import.Register"
#[derive(Serialize, Deserialize)]
pub enum MeasurandEnumType {
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
    #[serde(rename = "Energy.Active.Net")]
    EnergyActiveNet,
    #[serde(rename = "Energy.Apparent.Export")]
    EnergyApparentExport,
    #[serde(rename = "Energy.Apparent.Import")]
    EnergyApparentImport,
    #[serde(rename = "Energy.Apparent.Net")]
    EnergyApparentNet,
    #[serde(rename = "Energy.Reactive.Export.Interval")]
    EnergyReactiveExportInterval,
    #[serde(rename = "Energy.Reactive.Export.Register")]
    EnergyReactiveExportRegister,
    #[serde(rename = "Energy.Reactive.Import.Interval")]
    EnergyReactiveImportInterval,
    #[serde(rename = "Energy.Reactive.Import.Register")]
    EnergyReactiveImportRegister,
    #[serde(rename = "Energy.Reactive.Net")]
    EnergyReactiveNet,
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
    SoC,
    Voltage,
}

/// Sampled_ Value. Phase. Phase_ Code
/// urn:x-oca:ocpp:uid:1:569264
/// Indicates how the measured value is to be interpreted. For instance between L1 and
/// neutral (L1-N) Please note that not all values of phase are applicable to all Measurands.
/// When phase is absent, the measured value is interpreted as an overall value.
#[derive(Serialize, Deserialize)]
pub enum PhaseEnumType {
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

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct NotifyChargingLimitRequest {
    #[serde(rename = "chargingLimit")]
    pub charging_limit: ChargingLimitType,
    #[serde(rename = "chargingSchedule")]
    pub charging_schedule: Option<Vec<ChargingScheduleType>>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// The charging schedule contained in this notification applies to an EVSE. evseId must be
    /// &gt; 0.
    #[serde(rename = "evseId")]
    pub evse_id: Option<i64>,
}

/// Charging_ Limit
/// urn:x-enexis:ecdm:uid:2:234489
#[derive(Serialize, Deserialize)]
pub struct ChargingLimitType {
    #[serde(rename = "chargingLimitSource")]
    pub charging_limit_source: ChargingLimitSourceEnumType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Charging_ Limit. Is_ Grid_ Critical. Indicator
    /// urn:x-enexis:ecdm:uid:1:570847
    /// Indicates whether the charging limit is critical for the grid.
    #[serde(rename = "isGridCritical")]
    pub is_grid_critical: Option<bool>,
}

/// Charging_ Schedule
/// urn:x-oca:ocpp:uid:2:233256
/// Charging schedule structure defines a list of charging periods, as used in:
/// GetCompositeSchedule.conf and ChargingProfile.
#[derive(Serialize, Deserialize)]
pub struct ChargingScheduleType {
    #[serde(rename = "chargingRateUnit")]
    pub charging_rate_unit: ChargingRateUnitEnumType,
    #[serde(rename = "chargingSchedulePeriod")]
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Charging_ Schedule. Duration. Elapsed_ Time
    /// urn:x-oca:ocpp:uid:1:569236
    /// Duration of the charging schedule in seconds. If the duration is left empty, the last
    /// period will continue indefinitely or until end of the transaction if
    /// chargingProfilePurpose = TxProfile.
    pub duration: Option<i64>,
    /// Identifies the ChargingSchedule.
    pub id: i64,
    /// Charging_ Schedule. Min_ Charging_ Rate. Numeric
    /// urn:x-oca:ocpp:uid:1:569239
    /// Minimum charging rate supported by the EV. The unit of measure is defined by the
    /// chargingRateUnit. This parameter is intended to be used by a local smart charging
    /// algorithm to optimize the power allocation for in the case a charging process is
    /// inefficient at lower charging rates. Accepts at most one digit fraction (e.g. 8.1)
    #[serde(rename = "minChargingRate")]
    pub min_charging_rate: Option<f64>,
    #[serde(rename = "salesTariff")]
    pub sales_tariff: Option<SalesTariffType>,
    /// Charging_ Schedule. Start_ Schedule. Date_ Time
    /// urn:x-oca:ocpp:uid:1:569237
    /// Starting point of an absolute schedule. If absent the schedule will be relative to start
    /// of charging.
    #[serde(rename = "startSchedule")]
    pub start_schedule: Option<String>,
}

/// Charging_ Schedule_ Period
/// urn:x-oca:ocpp:uid:2:233257
/// Charging schedule period structure defines a time period in a charging schedule.
#[derive(Serialize, Deserialize)]
pub struct ChargingSchedulePeriodType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Charging_ Schedule_ Period. Limit. Measure
    /// urn:x-oca:ocpp:uid:1:569241
    /// Charging rate limit during the schedule period, in the applicable chargingRateUnit, for
    /// example in Amperes (A) or Watts (W). Accepts at most one digit fraction (e.g. 8.1).
    pub limit: f64,
    /// Charging_ Schedule_ Period. Number_ Phases. Counter
    /// urn:x-oca:ocpp:uid:1:569242
    /// The number of phases that can be used for charging. If a number of phases is needed,
    /// numberPhases=3 will be assumed unless another number is given.
    #[serde(rename = "numberPhases")]
    pub number_phases: Option<i64>,
    /// Values: 1..3, Used if numberPhases=1 and if the EVSE is capable of switching the phase
    /// connected to the EV, i.e. ACPhaseSwitchingSupported is defined and true. It’s not allowed
    /// unless both conditions above are true. If both conditions are true, and phaseToUse is
    /// omitted, the Charging Station / EVSE will make the selection on its own.
    #[serde(rename = "phaseToUse")]
    pub phase_to_use: Option<i64>,
    /// Charging_ Schedule_ Period. Start_ Period. Elapsed_ Time
    /// urn:x-oca:ocpp:uid:1:569240
    /// Start of the period, in seconds from the start of schedule. The value of StartPeriod also
    /// defines the stop time of the previous period.
    #[serde(rename = "startPeriod")]
    pub start_period: i64,
}

/// Sales_ Tariff
/// urn:x-oca:ocpp:uid:2:233272
/// NOTE: This dataType is based on dataTypes from &lt;&lt;ref-ISOIEC15118-2,ISO
/// 15118-2&gt;&gt;.
#[derive(Serialize, Deserialize)]
pub struct SalesTariffType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Identified_ Object. MRID. Numeric_ Identifier
    /// urn:x-enexis:ecdm:uid:1:569198
    /// SalesTariff identifier used to identify one sales tariff. An SAID remains a unique
    /// identifier for one schedule throughout a charging session.
    pub id: i64,
    /// Sales_ Tariff. Num_ E_ Price_ Levels. Counter
    /// urn:x-oca:ocpp:uid:1:569284
    /// Defines the overall number of distinct price levels used across all provided SalesTariff
    /// elements.
    #[serde(rename = "numEPriceLevels")]
    pub num_e_price_levels: Option<i64>,
    /// Sales_ Tariff. Sales. Tariff_ Description
    /// urn:x-oca:ocpp:uid:1:569283
    /// A human readable title/short description of the sales tariff e.g. for HMI display
    /// purposes.
    #[serde(rename = "salesTariffDescription")]
    pub sales_tariff_description: Option<String>,
    #[serde(rename = "salesTariffEntry")]
    pub sales_tariff_entry: Vec<SalesTariffEntryType>,
}

/// Sales_ Tariff_ Entry
/// urn:x-oca:ocpp:uid:2:233271
#[derive(Serialize, Deserialize)]
pub struct SalesTariffEntryType {
    #[serde(rename = "consumptionCost")]
    pub consumption_cost: Option<Vec<ConsumptionCostType>>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Sales_ Tariff_ Entry. E_ Price_ Level. Unsigned_ Integer
    /// urn:x-oca:ocpp:uid:1:569281
    /// Defines the price level of this SalesTariffEntry (referring to NumEPriceLevels). Small
    /// values for the EPriceLevel represent a cheaper TariffEntry. Large values for the
    /// EPriceLevel represent a more expensive TariffEntry.
    #[serde(rename = "ePriceLevel")]
    pub e_price_level: Option<i64>,
    #[serde(rename = "relativeTimeInterval")]
    pub relative_time_interval: RelativeTimeIntervalType,
}

/// Consumption_ Cost
/// urn:x-oca:ocpp:uid:2:233259
#[derive(Serialize, Deserialize)]
pub struct ConsumptionCostType {
    pub cost: Vec<CostType>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Consumption_ Cost. Start_ Value. Numeric
    /// urn:x-oca:ocpp:uid:1:569246
    /// The lowest level of consumption that defines the starting point of this consumption
    /// block. The block interval extends to the start of the next interval.
    #[serde(rename = "startValue")]
    pub start_value: f64,
}

/// Cost
/// urn:x-oca:ocpp:uid:2:233258
#[derive(Serialize, Deserialize)]
pub struct CostType {
    /// Cost. Amount. Amount
    /// urn:x-oca:ocpp:uid:1:569244
    /// The estimated or actual cost per kWh
    pub amount: i64,
    /// Cost. Amount_ Multiplier. Integer
    /// urn:x-oca:ocpp:uid:1:569245
    /// Values: -3..3, The amountMultiplier defines the exponent to base 10 (dec). The final
    /// value is determined by: amount * 10 ^ amountMultiplier
    #[serde(rename = "amountMultiplier")]
    pub amount_multiplier: Option<i64>,
    #[serde(rename = "costKind")]
    pub cost_kind: CostKindEnumType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
}

/// Relative_ Timer_ Interval
/// urn:x-oca:ocpp:uid:2:233270
#[derive(Serialize, Deserialize)]
pub struct RelativeTimeIntervalType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Relative_ Timer_ Interval. Duration. Elapsed_ Time
    /// urn:x-oca:ocpp:uid:1:569280
    /// Duration of the interval, in seconds.
    pub duration: Option<i64>,
    /// Relative_ Timer_ Interval. Start. Elapsed_ Time
    /// urn:x-oca:ocpp:uid:1:569279
    /// Start of the interval, in seconds from NOW.
    pub start: i64,
}

/// Cost. Cost_ Kind. Cost_ Kind_ Code
/// urn:x-oca:ocpp:uid:1:569243
/// The kind of cost referred to in the message element amount
#[derive(Serialize, Deserialize)]
pub enum CostKindEnumType {
    CarbonDioxideEmission,
    RelativePricePercentage,
    RenewableGenerationPercentage,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct NotifyCustomerInformationRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// (Part of) the requested data. No format specified in which the data is returned. Should
    /// be human readable.
    pub data: String,
    /// Timestamp of the moment this message was generated at the Charging Station.
    #[serde(rename = "generatedAt")]
    pub generated_at: String,
    /// The Id of the request.
    #[serde(rename = "requestId")]
    pub request_id: i64,
    /// Sequence number of this message. First message starts at 0.
    #[serde(rename = "seqNo")]
    pub seq_no: i64,
    /// “to be continued” indicator. Indicates whether another part of the monitoringData follows
    /// in an upcoming notifyMonitoringReportRequest message. Default value when omitted is false.
    pub tbc: Option<bool>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct NotifyDisplayMessagesRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "messageInfo")]
    pub message_info: Option<Vec<MessageInfoType>>,
    /// The id of the &lt;&lt;getdisplaymessagesrequest,GetDisplayMessagesRequest&gt;&gt; that
    /// requested this message.
    #[serde(rename = "requestId")]
    pub request_id: i64,
    /// "to be continued" indicator. Indicates whether another part of the report follows in an
    /// upcoming NotifyDisplayMessagesRequest message. Default value when omitted is false.
    pub tbc: Option<bool>,
}

/// Message_ Info
/// urn:x-enexis:ecdm:uid:2:233264
/// Contains message details, for a message to be displayed on a Charging Station.
#[derive(Serialize, Deserialize)]
pub struct MessageInfoType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    pub display: Option<ComponentType>,
    /// Message_ Info. End. Date_ Time
    /// urn:x-enexis:ecdm:uid:1:569257
    /// Until what date-time should this message be shown, after this date/time this message
    /// SHALL be removed.
    #[serde(rename = "endDateTime")]
    pub end_date_time: Option<String>,
    /// Identified_ Object. MRID. Numeric_ Identifier
    /// urn:x-enexis:ecdm:uid:1:569198
    /// Master resource identifier, unique within an exchange context. It is defined within the
    /// OCPP context as a positive Integer value (greater or equal to zero).
    pub id: i64,
    pub message: MessageContentType,
    pub priority: MessagePriorityEnumType,
    /// Message_ Info. Start. Date_ Time
    /// urn:x-enexis:ecdm:uid:1:569256
    /// From what date-time should this message be shown. If omitted: directly.
    #[serde(rename = "startDateTime")]
    pub start_date_time: Option<String>,
    pub state: Option<MessageStateEnumType>,
    /// During which transaction shall this message be shown.
    /// Message SHALL be removed by the Charging Station after transaction has
    /// ended.
    #[serde(rename = "transactionId")]
    pub transaction_id: Option<String>,
}

/// Message_ Content
/// urn:x-enexis:ecdm:uid:2:234490
/// Contains message details, for a message to be displayed on a Charging Station.
#[derive(Serialize, Deserialize)]
pub struct MessageContentType {
    /// Message_ Content. Content. Message
    /// urn:x-enexis:ecdm:uid:1:570852
    /// Message contents.
    pub content: String,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    pub format: MessageFormatEnumType,
    /// Message_ Content. Language. Language_ Code
    /// urn:x-enexis:ecdm:uid:1:570849
    /// Message language identifier. Contains a language code as defined in
    /// &lt;&lt;ref-RFC5646,[RFC5646]&gt;&gt;.
    pub language: Option<String>,
}

/// Message_ Content. Format. Message_ Format_ Code
/// urn:x-enexis:ecdm:uid:1:570848
/// Format of the message.
#[derive(Serialize, Deserialize)]
pub enum MessageFormatEnumType {
    #[serde(rename = "ASCII")]
    Ascii,
    #[serde(rename = "HTML")]
    Html,
    #[serde(rename = "URI")]
    Uri,
    #[serde(rename = "UTF8")]
    Utf8,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct NotifyEvChargingNeedsRequest {
    #[serde(rename = "chargingNeeds")]
    pub charging_needs: ChargingNeedsType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Defines the EVSE and connector to which the EV is connected. EvseId may not be 0.
    #[serde(rename = "evseId")]
    pub evse_id: i64,
    /// Contains the maximum schedule tuples the car supports per schedule.
    #[serde(rename = "maxScheduleTuples")]
    pub max_schedule_tuples: Option<i64>,
}

/// Charging_ Needs
/// urn:x-oca:ocpp:uid:2:233249
#[derive(Serialize, Deserialize)]
pub struct ChargingNeedsType {
    #[serde(rename = "acChargingParameters")]
    pub ac_charging_parameters: Option<AcChargingParametersType>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "dcChargingParameters")]
    pub dc_charging_parameters: Option<DcChargingParametersType>,
    /// Charging_ Needs. Departure_ Time. Date_ Time
    /// urn:x-oca:ocpp:uid:1:569223
    /// Estimated departure time of the EV.
    #[serde(rename = "departureTime")]
    pub departure_time: Option<String>,
    #[serde(rename = "requestedEnergyTransfer")]
    pub requested_energy_transfer: EnergyTransferModeEnumType,
}

/// AC_ Charging_ Parameters
/// urn:x-oca:ocpp:uid:2:233250
/// EV AC charging parameters.
#[derive(Serialize, Deserialize)]
pub struct AcChargingParametersType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// AC_ Charging_ Parameters. Energy_ Amount. Energy_ Amount
    /// urn:x-oca:ocpp:uid:1:569211
    /// Amount of energy requested (in Wh). This includes energy required for preconditioning.
    #[serde(rename = "energyAmount")]
    pub energy_amount: i64,
    /// AC_ Charging_ Parameters. EV_ Max. Current
    /// urn:x-oca:ocpp:uid:1:569213
    /// Maximum current (amps) supported by the electric vehicle (per phase). Includes cable
    /// capacity.
    #[serde(rename = "evMaxCurrent")]
    pub ev_max_current: i64,
    /// AC_ Charging_ Parameters. EV_ Max. Voltage
    /// urn:x-oca:ocpp:uid:1:569214
    /// Maximum voltage supported by the electric vehicle
    #[serde(rename = "evMaxVoltage")]
    pub ev_max_voltage: i64,
    /// AC_ Charging_ Parameters. EV_ Min. Current
    /// urn:x-oca:ocpp:uid:1:569212
    /// Minimum current (amps) supported by the electric vehicle (per phase).
    #[serde(rename = "evMinCurrent")]
    pub ev_min_current: i64,
}

/// DC_ Charging_ Parameters
/// urn:x-oca:ocpp:uid:2:233251
/// EV DC charging parameters
#[derive(Serialize, Deserialize)]
pub struct DcChargingParametersType {
    /// DC_ Charging_ Parameters. Bulk_ SOC. Percentage
    /// urn:x-oca:ocpp:uid:1:569222
    /// Percentage of SoC at which the EV considers a fast charging process to end. (possible
    /// values: 0 - 100)
    #[serde(rename = "bulkSoC")]
    pub bulk_so_c: Option<i64>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// DC_ Charging_ Parameters. Energy_ Amount. Energy_ Amount
    /// urn:x-oca:ocpp:uid:1:569217
    /// Amount of energy requested (in Wh). This inludes energy required for preconditioning.
    #[serde(rename = "energyAmount")]
    pub energy_amount: Option<i64>,
    /// DC_ Charging_ Parameters. EV_ Energy_ Capacity. Numeric
    /// urn:x-oca:ocpp:uid:1:569220
    /// Capacity of the electric vehicle battery (in Wh)
    #[serde(rename = "evEnergyCapacity")]
    pub ev_energy_capacity: Option<i64>,
    /// DC_ Charging_ Parameters. EV_ Max. Current
    /// urn:x-oca:ocpp:uid:1:569215
    /// Maximum current (amps) supported by the electric vehicle. Includes cable capacity.
    #[serde(rename = "evMaxCurrent")]
    pub ev_max_current: i64,
    /// DC_ Charging_ Parameters. EV_ Max. Power
    /// urn:x-oca:ocpp:uid:1:569218
    /// Maximum power (in W) supported by the electric vehicle. Required for DC charging.
    #[serde(rename = "evMaxPower")]
    pub ev_max_power: Option<i64>,
    /// DC_ Charging_ Parameters. EV_ Max. Voltage
    /// urn:x-oca:ocpp:uid:1:569216
    /// Maximum voltage supported by the electric vehicle
    #[serde(rename = "evMaxVoltage")]
    pub ev_max_voltage: i64,
    /// DC_ Charging_ Parameters. Full_ SOC. Percentage
    /// urn:x-oca:ocpp:uid:1:569221
    /// Percentage of SoC at which the EV considers the battery fully charged. (possible values:
    /// 0 - 100)
    #[serde(rename = "fullSoC")]
    pub full_so_c: Option<i64>,
    /// DC_ Charging_ Parameters. State_ Of_ Charge. Numeric
    /// urn:x-oca:ocpp:uid:1:569219
    /// Energy available in the battery (in percent of the battery capacity)
    #[serde(rename = "stateOfCharge")]
    pub state_of_charge: Option<i64>,
}

/// Charging_ Needs. Requested. Energy_ Transfer_ Mode_ Code
/// urn:x-oca:ocpp:uid:1:569209
/// Mode of energy transfer requested by the EV.
#[derive(Serialize, Deserialize)]
pub enum EnergyTransferModeEnumType {
    #[serde(rename = "AC_single_phase")]
    AcSinglePhase,
    #[serde(rename = "AC_three_phase")]
    AcThreePhase,
    #[serde(rename = "AC_two_phase")]
    AcTwoPhase,
    #[serde(rename = "DC")]
    Dc,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct NotifyEvChargingScheduleRequest {
    #[serde(rename = "chargingSchedule")]
    pub charging_schedule: ChargingScheduleType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// The charging schedule contained in this notification applies to an EVSE. EvseId must be
    /// &gt; 0.
    #[serde(rename = "evseId")]
    pub evse_id: i64,
    /// Periods contained in the charging profile are relative to this point in time.
    #[serde(rename = "timeBase")]
    pub time_base: String,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct NotifyEventRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "eventData")]
    pub event_data: Vec<EventDataType>,
    /// Timestamp of the moment this message was generated at the Charging Station.
    #[serde(rename = "generatedAt")]
    pub generated_at: String,
    /// Sequence number of this message. First message starts at 0.
    #[serde(rename = "seqNo")]
    pub seq_no: i64,
    /// “to be continued” indicator. Indicates whether another part of the report follows in an
    /// upcoming notifyEventRequest message. Default value when omitted is false.
    pub tbc: Option<bool>,
}

/// Class to report an event notification for a component-variable.
#[derive(Serialize, Deserialize)]
pub struct EventDataType {
    /// Actual value (_attributeType_ Actual) of the variable.
    ///
    /// The Configuration Variable
    /// &lt;&lt;configkey-reporting-value-size,ReportingValueSize&gt;&gt; can be used to limit
    /// GetVariableResult.attributeValue, VariableAttribute.value and EventData.actualValue. The
    /// max size of these values will always remain equal.
    #[serde(rename = "actualValue")]
    pub actual_value: String,
    /// Refers to the Id of an event that is considered to be the cause for this event.
    pub cause: Option<i64>,
    /// _Cleared_ is set to true to report the clearing of a monitored situation, i.e. a 'return
    /// to normal'.
    pub cleared: Option<bool>,
    pub component: ComponentType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Identifies the event. This field can be referred to as a cause by other events.
    #[serde(rename = "eventId")]
    pub event_id: i64,
    #[serde(rename = "eventNotificationType")]
    pub event_notification_type: EventNotificationEnumType,
    /// Technical (error) code as reported by component.
    #[serde(rename = "techCode")]
    pub tech_code: Option<String>,
    /// Technical detail information as reported by component.
    #[serde(rename = "techInfo")]
    pub tech_info: Option<String>,
    /// Timestamp of the moment the report was generated.
    pub timestamp: String,
    /// If an event notification is linked to a specific transaction, this field can be used to
    /// specify its transactionId.
    #[serde(rename = "transactionId")]
    pub transaction_id: Option<String>,
    pub trigger: EventTriggerEnumType,
    pub variable: VariableType,
    /// Identifies the VariableMonitoring which triggered the event.
    #[serde(rename = "variableMonitoringId")]
    pub variable_monitoring_id: Option<i64>,
}

/// Specifies the event notification type of the message.
#[derive(Serialize, Deserialize)]
pub enum EventNotificationEnumType {
    CustomMonitor,
    HardWiredMonitor,
    HardWiredNotification,
    PreconfiguredMonitor,
}

/// Type of monitor that triggered this event, e.g. exceeding a threshold value.
#[derive(Serialize, Deserialize)]
pub enum EventTriggerEnumType {
    Alerting,
    Delta,
    Periodic,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct NotifyMonitoringReportRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Timestamp of the moment this message was generated at the Charging Station.
    #[serde(rename = "generatedAt")]
    pub generated_at: String,
    pub monitor: Option<Vec<MonitoringDataType>>,
    /// The id of the GetMonitoringRequest that requested this report.
    #[serde(rename = "requestId")]
    pub request_id: i64,
    /// Sequence number of this message. First message starts at 0.
    #[serde(rename = "seqNo")]
    pub seq_no: i64,
    /// “to be continued” indicator. Indicates whether another part of the monitoringData follows
    /// in an upcoming notifyMonitoringReportRequest message. Default value when omitted is false.
    pub tbc: Option<bool>,
}

/// Class to hold parameters of SetVariableMonitoring request.
#[derive(Serialize, Deserialize)]
pub struct MonitoringDataType {
    pub component: ComponentType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    pub variable: VariableType,
    #[serde(rename = "variableMonitoring")]
    pub variable_monitoring: Vec<VariableMonitoringType>,
}

/// A monitoring setting for a variable.
#[derive(Serialize, Deserialize)]
pub struct VariableMonitoringType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Identifies the monitor.
    pub id: i64,
    /// The severity that will be assigned to an event that is triggered by this monitor. The
    /// severity range is 0-9, with 0 as the highest and 9 as the lowest severity level.
    ///
    /// The severity levels have the following meaning: +
    /// *0-Danger* +
    /// Indicates lives are potentially in danger. Urgent attention is needed and action should
    /// be taken immediately. +
    /// *1-Hardware Failure* +
    /// Indicates that the Charging Station is unable to continue regular operations due to
    /// Hardware issues. Action is required. +
    /// *2-System Failure* +
    /// Indicates that the Charging Station is unable to continue regular operations due to
    /// software or minor hardware issues. Action is required. +
    /// *3-Critical* +
    /// Indicates a critical error. Action is required. +
    /// *4-Error* +
    /// Indicates a non-urgent error. Action is required. +
    /// *5-Alert* +
    /// Indicates an alert event. Default severity for any type of monitoring event.  +
    /// *6-Warning* +
    /// Indicates a warning event. Action may be required. +
    /// *7-Notice* +
    /// Indicates an unusual event. No immediate action is required. +
    /// *8-Informational* +
    /// Indicates a regular operational event. May be used for reporting, measuring throughput,
    /// etc. No action is required. +
    /// *9-Debug* +
    /// Indicates information useful to developers for debugging, not useful during operations.
    pub severity: i64,
    /// Monitor only active when a transaction is ongoing on a component relevant to this
    /// transaction.
    pub transaction: bool,
    #[serde(rename = "type")]
    pub variable_monitoring_type_type: MonitorEnumType,
    /// Value for threshold or delta monitoring.
    /// For Periodic or PeriodicClockAligned this is the interval in seconds.
    pub value: f64,
}

/// The type of this monitor, e.g. a threshold, delta or periodic monitor.
#[derive(Serialize, Deserialize)]
pub enum MonitorEnumType {
    Delta,
    LowerThreshold,
    Periodic,
    PeriodicClockAligned,
    UpperThreshold,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct NotifyReportRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Timestamp of the moment this message was generated at the Charging Station.
    #[serde(rename = "generatedAt")]
    pub generated_at: String,
    #[serde(rename = "reportData")]
    pub report_data: Option<Vec<ReportDataType>>,
    /// The id of the GetReportRequest  or GetBaseReportRequest that requested this report
    #[serde(rename = "requestId")]
    pub request_id: i64,
    /// Sequence number of this message. First message starts at 0.
    #[serde(rename = "seqNo")]
    pub seq_no: i64,
    /// “to be continued” indicator. Indicates whether another part of the report follows in an
    /// upcoming notifyReportRequest message. Default value when omitted is false.
    pub tbc: Option<bool>,
}

/// Class to report components, variables and variable attributes and characteristics.
#[derive(Serialize, Deserialize)]
pub struct ReportDataType {
    pub component: ComponentType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    pub variable: VariableType,
    #[serde(rename = "variableAttribute")]
    pub variable_attribute: Vec<VariableAttributeType>,
    #[serde(rename = "variableCharacteristics")]
    pub variable_characteristics: Option<VariableCharacteristicsType>,
}

/// Attribute data of a variable.
#[derive(Serialize, Deserialize)]
pub struct VariableAttributeType {
    /// If true, value that will never be changed by the Charging Station at runtime. Default
    /// when omitted is false.
    pub constant: Option<bool>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    pub mutability: Option<MutabilityEnumType>,
    /// If true, value will be persistent across system reboots or power down. Default when
    /// omitted is false.
    pub persistent: Option<bool>,
    #[serde(rename = "type")]
    pub variable_attribute_type_type: Option<AttributeEnumType>,
    /// Value of the attribute. May only be omitted when mutability is set to 'WriteOnly'.
    ///
    /// The Configuration Variable
    /// &lt;&lt;configkey-reporting-value-size,ReportingValueSize&gt;&gt; can be used to limit
    /// GetVariableResult.attributeValue, VariableAttribute.value and EventData.actualValue. The
    /// max size of these values will always remain equal.
    pub value: Option<String>,
}

/// Fixed read-only parameters of a variable.
#[derive(Serialize, Deserialize)]
pub struct VariableCharacteristicsType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "dataType")]
    pub data_type: DataEnumType,
    /// Maximum possible value of this variable. When the datatype of this Variable is String,
    /// OptionList, SequenceList or MemberList, this field defines the maximum length of the
    /// (CSV) string.
    #[serde(rename = "maxLimit")]
    pub max_limit: Option<f64>,
    /// Minimum possible value of this variable.
    #[serde(rename = "minLimit")]
    pub min_limit: Option<f64>,
    /// Flag indicating if this variable supports monitoring.
    #[serde(rename = "supportsMonitoring")]
    pub supports_monitoring: bool,
    /// Unit of the variable. When the transmitted value has a unit, this field SHALL be included.
    pub unit: Option<String>,
    /// Allowed values when variable is Option/Member/SequenceList.
    ///
    /// * OptionList: The (Actual) Variable value must be a single value from the reported (CSV)
    /// enumeration list.
    ///
    /// * MemberList: The (Actual) Variable value  may be an (unordered) (sub-)set of the
    /// reported (CSV) valid values list.
    ///
    /// * SequenceList: The (Actual) Variable value  may be an ordered (priority, etc)  (sub-)set
    /// of the reported (CSV) valid values.
    ///
    /// This is a comma separated list.
    ///
    /// The Configuration Variable
    /// &lt;&lt;configkey-configuration-value-size,ConfigurationValueSize&gt;&gt; can be used to
    /// limit SetVariableData.attributeValue and VariableCharacteristics.valueList. The max size
    /// of these values will always remain equal.
    #[serde(rename = "valuesList")]
    pub values_list: Option<String>,
}

/// Defines the mutability of this attribute. Default is ReadWrite when omitted.
#[derive(Serialize, Deserialize)]
pub enum MutabilityEnumType {
    ReadOnly,
    ReadWrite,
    WriteOnly,
}

/// Data type of this variable.
#[derive(Serialize, Deserialize)]
pub enum DataEnumType {
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "dateTime")]
    DateTime,
    #[serde(rename = "decimal")]
    Decimal,
    #[serde(rename = "integer")]
    Integer,
    MemberList,
    OptionList,
    SequenceList,
    #[serde(rename = "string")]
    String,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct PublishFirmwareStatusNotificationRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Required if status is Published. Can be multiple URI’s, if the Local Controller supports
    /// e.g. HTTP, HTTPS, and FTP.
    pub location: Option<Vec<String>>,
    /// The request id that was
    /// provided in the
    /// PublishFirmwareRequest which
    /// triggered this action.
    #[serde(rename = "requestId")]
    pub request_id: Option<i64>,
    pub status: PublishFirmwareStatusEnumType,
}

/// This contains the progress status of the publishfirmware
/// installation.
#[derive(Serialize, Deserialize)]
pub enum PublishFirmwareStatusEnumType {
    ChecksumVerified,
    DownloadFailed,
    DownloadPaused,
    DownloadScheduled,
    Downloaded,
    Downloading,
    Idle,
    InvalidChecksum,
    PublishFailed,
    Published,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ReportChargingProfilesRequest {
    #[serde(rename = "chargingLimitSource")]
    pub charging_limit_source: ChargingLimitSourceEnumType,
    #[serde(rename = "chargingProfile")]
    pub charging_profile: Vec<ChargingProfileType>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// The evse to which the charging profile applies. If evseId = 0, the message contains an
    /// overall limit for the Charging Station.
    #[serde(rename = "evseId")]
    pub evse_id: i64,
    /// Id used to match the &lt;&lt;getchargingprofilesrequest,
    /// GetChargingProfilesRequest&gt;&gt; message with the resulting
    /// ReportChargingProfilesRequest messages. When the CSMS provided a requestId in the
    /// &lt;&lt;getchargingprofilesrequest, GetChargingProfilesRequest&gt;&gt;, this field SHALL
    /// contain the same value.
    #[serde(rename = "requestId")]
    pub request_id: i64,
    /// To Be Continued. Default value when omitted: false. false indicates that there are no
    /// further messages as part of this report.
    pub tbc: Option<bool>,
}

/// Charging_ Profile
/// urn:x-oca:ocpp:uid:2:233255
/// A ChargingProfile consists of ChargingSchedule, describing the amount of power or current
/// that can be delivered per time interval.
#[derive(Serialize, Deserialize)]
pub struct ChargingProfileType {
    #[serde(rename = "chargingProfileKind")]
    pub charging_profile_kind: ChargingProfileKindEnumType,
    #[serde(rename = "chargingProfilePurpose")]
    pub charging_profile_purpose: ChargingProfilePurposeEnumType,
    #[serde(rename = "chargingSchedule")]
    pub charging_schedule: Vec<ChargingScheduleType>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Identified_ Object. MRID. Numeric_ Identifier
    /// urn:x-enexis:ecdm:uid:1:569198
    /// Id of ChargingProfile.
    pub id: i64,
    #[serde(rename = "recurrencyKind")]
    pub recurrency_kind: Option<RecurrencyKindEnumType>,
    /// Charging_ Profile. Stack_ Level. Counter
    /// urn:x-oca:ocpp:uid:1:569230
    /// Value determining level in hierarchy stack of profiles. Higher values have precedence
    /// over lower values. Lowest level is 0.
    #[serde(rename = "stackLevel")]
    pub stack_level: i64,
    /// SHALL only be included if ChargingProfilePurpose is set to TxProfile. The transactionId
    /// is used to match the profile to a specific transaction.
    #[serde(rename = "transactionId")]
    pub transaction_id: Option<String>,
    /// Charging_ Profile. Valid_ From. Date_ Time
    /// urn:x-oca:ocpp:uid:1:569234
    /// Point in time at which the profile starts to be valid. If absent, the profile is valid as
    /// soon as it is received by the Charging Station.
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    /// Charging_ Profile. Valid_ To. Date_ Time
    /// urn:x-oca:ocpp:uid:1:569235
    /// Point in time at which the profile stops to be valid. If absent, the profile is valid
    /// until it is replaced by another profile.
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
}

/// Charging_ Profile. Charging_ Profile_ Kind. Charging_ Profile_ Kind_ Code
/// urn:x-oca:ocpp:uid:1:569232
/// Indicates the kind of schedule.
#[derive(Serialize, Deserialize)]
pub enum ChargingProfileKindEnumType {
    Absolute,
    Recurring,
    Relative,
}

/// Charging_ Profile. Recurrency_ Kind. Recurrency_ Kind_ Code
/// urn:x-oca:ocpp:uid:1:569233
/// Indicates the start point of a recurrence.
#[derive(Serialize, Deserialize)]
pub enum RecurrencyKindEnumType {
    Daily,
    Weekly,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct RequestStartTransactionRequest {
    #[serde(rename = "chargingProfile")]
    pub charging_profile: Option<ChargingProfileType>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Number of the EVSE on which to start the transaction. EvseId SHALL be &gt; 0
    #[serde(rename = "evseId")]
    pub evse_id: Option<i64>,
    #[serde(rename = "groupIdToken")]
    pub group_id_token: Option<IdTokenType>,
    #[serde(rename = "idToken")]
    pub id_token: IdTokenType,
    /// Id given by the server to this start request. The Charging Station might return this in
    /// the &lt;&lt;transactioneventrequest, TransactionEventRequest&gt;&gt;, letting the server
    /// know which transaction was started for this request. Use to start a transaction.
    #[serde(rename = "remoteStartId")]
    pub remote_start_id: i64,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct RequestStopTransactionRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// The identifier of the transaction which the Charging Station is requested to stop.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ReservationStatusUpdateRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// The ID of the reservation.
    #[serde(rename = "reservationId")]
    pub reservation_id: i64,
    #[serde(rename = "reservationUpdateStatus")]
    pub reservation_update_status: ReservationUpdateStatusEnumType,
}

/// The updated reservation status.
#[derive(Serialize, Deserialize)]
pub enum ReservationUpdateStatusEnumType {
    Expired,
    Removed,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ReserveNowRequest {
    #[serde(rename = "connectorType")]
    pub connector_type: Option<ConnectorEnumType>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// This contains ID of the evse to be reserved.
    #[serde(rename = "evseId")]
    pub evse_id: Option<i64>,
    /// Date and time at which the reservation expires.
    #[serde(rename = "expiryDateTime")]
    pub expiry_date_time: String,
    #[serde(rename = "groupIdToken")]
    pub group_id_token: Option<IdTokenType>,
    /// Id of reservation.
    pub id: i64,
    #[serde(rename = "idToken")]
    pub id_token: IdTokenType,
}

/// This field specifies the connector type.
#[derive(Serialize, Deserialize)]
pub enum ConnectorEnumType {
    #[serde(rename = "cCCS1")]
    CCcs1,
    #[serde(rename = "cCCS2")]
    CCcs2,
    #[serde(rename = "cG105")]
    CG105,
    #[serde(rename = "cTesla")]
    CTesla,
    #[serde(rename = "cType1")]
    CType1,
    #[serde(rename = "cType2")]
    CType2,
    Other1PhMax16A,
    Other1PhOver16A,
    Other3Ph,
    Pan,
    #[serde(rename = "s309-1P-16A")]
    S3091P16A,
    #[serde(rename = "s309-1P-32A")]
    S3091P32A,
    #[serde(rename = "s309-3P-16A")]
    S3093P16A,
    #[serde(rename = "s309-3P-32A")]
    S3093P32A,
    #[serde(rename = "sBS1361")]
    SBs1361,
    #[serde(rename = "sCEE-7-7")]
    SCee77,
    #[serde(rename = "sType2")]
    SType2,
    #[serde(rename = "sType3")]
    SType3,
    Undetermined,
    Unknown,
    #[serde(rename = "wInductive")]
    WInductive,
    #[serde(rename = "wResonant")]
    WResonant,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ResetRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// This contains the ID of a specific EVSE that needs to be reset, instead of the entire
    /// Charging Station.
    #[serde(rename = "evseId")]
    pub evse_id: Option<i64>,
    #[serde(rename = "type")]
    pub reset_request_type: ResetEnumType,
}

/// This contains the type of reset that the Charging Station or EVSE should perform.
#[derive(Serialize, Deserialize)]
pub enum ResetEnumType {
    Immediate,
    OnIdle,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SecurityEventNotificationRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Additional information about the occurred security event.
    #[serde(rename = "techInfo")]
    pub tech_info: Option<String>,
    /// Date and time at which the event occurred.
    pub timestamp: String,
    /// Type of the security event. This value should be taken from the Security events list.
    #[serde(rename = "type")]
    pub security_event_notification_request_type: String,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SendLocalListRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "localAuthorizationList")]
    pub local_authorization_list: Option<Vec<AuthorizationData>>,
    #[serde(rename = "updateType")]
    pub update_type: UpdateEnumType,
    /// In case of a full update this is the version number of the full list. In case of a
    /// differential update it is the version number of the list after the update has been
    /// applied.
    #[serde(rename = "versionNumber")]
    pub version_number: i64,
}

/// Contains the identifier to use for authorization.
#[derive(Serialize, Deserialize)]
pub struct AuthorizationData {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "idToken")]
    pub id_token: IdTokenType,
    #[serde(rename = "idTokenInfo")]
    pub id_token_info: Option<IdTokenInfoType>,
}

/// ID_ Token
/// urn:x-oca:ocpp:uid:2:233247
/// Contains status information about an identifier.
/// It is advised to not stop charging for a token that expires during charging, as
/// ExpiryDate is only used for caching purposes. If ExpiryDate is not given, the status has
/// no end date.
#[derive(Serialize, Deserialize)]
pub struct IdTokenInfoType {
    /// ID_ Token. Expiry. Date_ Time
    /// urn:x-oca:ocpp:uid:1:569373
    /// Date and Time after which the token must be considered invalid.
    #[serde(rename = "cacheExpiryDateTime")]
    pub cache_expiry_date_time: Option<String>,
    /// Priority from a business point of view. Default priority is 0, The range is from -9 to 9.
    /// Higher values indicate a higher priority. The chargingPriority in
    /// &lt;&lt;transactioneventresponse,TransactionEventResponse&gt;&gt; overrules this one.
    #[serde(rename = "chargingPriority")]
    pub charging_priority: Option<i64>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Only used when the IdToken is only valid for one or more specific EVSEs, not for the
    /// entire Charging Station.
    #[serde(rename = "evseId")]
    pub evse_id: Option<Vec<i64>>,
    #[serde(rename = "groupIdToken")]
    pub group_id_token: Option<IdTokenType>,
    /// ID_ Token. Language1. Language_ Code
    /// urn:x-oca:ocpp:uid:1:569374
    /// Preferred user interface language of identifier user. Contains a language code as defined
    /// in &lt;&lt;ref-RFC5646,[RFC5646]&gt;&gt;.
    pub language1: Option<String>,
    /// ID_ Token. Language2. Language_ Code
    /// urn:x-oca:ocpp:uid:1:569375
    /// Second preferred user interface language of identifier user. Don’t use when language1 is
    /// omitted, has to be different from language1. Contains a language code as defined in
    /// &lt;&lt;ref-RFC5646,[RFC5646]&gt;&gt;.
    pub language2: Option<String>,
    #[serde(rename = "personalMessage")]
    pub personal_message: Option<MessageContentType>,
    pub status: AuthorizationStatusEnumType,
}

/// ID_ Token. Status. Authorization_ Status
/// urn:x-oca:ocpp:uid:1:569372
/// Current status of the ID Token.
#[derive(Serialize, Deserialize)]
pub enum AuthorizationStatusEnumType {
    Accepted,
    Blocked,
    ConcurrentTx,
    Expired,
    Invalid,
    NoCredit,
    #[serde(rename = "NotAllowedTypeEVSE")]
    NotAllowedTypeEvse,
    NotAtThisLocation,
    NotAtThisTime,
    Unknown,
}

/// This contains the type of update (full or differential) of this request.
#[derive(Serialize, Deserialize)]
pub enum UpdateEnumType {
    Differential,
    Full,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SetChargingProfileRequest {
    #[serde(rename = "chargingProfile")]
    pub charging_profile: ChargingProfileType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// For TxDefaultProfile an evseId=0 applies the profile to each individual evse. For
    /// ChargingStationMaxProfile and ChargingStationExternalConstraints an evseId=0 contains an
    /// overal limit for the whole Charging Station.
    #[serde(rename = "evseId")]
    pub evse_id: i64,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SetDisplayMessageRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    pub message: MessageInfoType,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SetMonitoringBaseRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "monitoringBase")]
    pub monitoring_base: MonitoringBaseEnumType,
}

/// Specify which monitoring base will be set
#[derive(Serialize, Deserialize)]
pub enum MonitoringBaseEnumType {
    All,
    FactoryDefault,
    HardWiredOnly,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SetMonitoringLevelRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// The Charging Station SHALL only report events with a severity number lower than or equal
    /// to this severity.
    /// The severity range is 0-9, with 0 as the highest and 9 as the lowest severity level.
    ///
    /// The severity levels have the following meaning: +
    /// *0-Danger* +
    /// Indicates lives are potentially in danger. Urgent attention is needed and action should
    /// be taken immediately. +
    /// *1-Hardware Failure* +
    /// Indicates that the Charging Station is unable to continue regular operations due to
    /// Hardware issues. Action is required. +
    /// *2-System Failure* +
    /// Indicates that the Charging Station is unable to continue regular operations due to
    /// software or minor hardware issues. Action is required. +
    /// *3-Critical* +
    /// Indicates a critical error. Action is required. +
    /// *4-Error* +
    /// Indicates a non-urgent error. Action is required. +
    /// *5-Alert* +
    /// Indicates an alert event. Default severity for any type of monitoring event.  +
    /// *6-Warning* +
    /// Indicates a warning event. Action may be required. +
    /// *7-Notice* +
    /// Indicates an unusual event. No immediate action is required. +
    /// *8-Informational* +
    /// Indicates a regular operational event. May be used for reporting, measuring throughput,
    /// etc. No action is required. +
    /// *9-Debug* +
    /// Indicates information useful to developers for debugging, not useful during operations.
    pub severity: i64,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SetNetworkProfileRequest {
    /// Slot in which the configuration should be stored.
    #[serde(rename = "configurationSlot")]
    pub configuration_slot: i64,
    #[serde(rename = "connectionData")]
    pub connection_data: NetworkConnectionProfileType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
}

/// Communication_ Function
/// urn:x-oca:ocpp:uid:2:233304
/// The NetworkConnectionProfile defines the functional and technical parameters of a
/// communication link.
#[derive(Serialize, Deserialize)]
pub struct NetworkConnectionProfileType {
    pub apn: Option<ApnType>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Duration in seconds before a message send by the Charging Station via this network
    /// connection times-out.
    /// The best setting depends on the underlying network and response times of the CSMS.
    /// If you are looking for a some guideline: use 30 seconds as a starting point.
    #[serde(rename = "messageTimeout")]
    pub message_timeout: i64,
    /// Communication_ Function. OCPP_ Central_ System_ URL. URI
    /// urn:x-oca:ocpp:uid:1:569357
    /// URL of the CSMS(s) that this Charging Station  communicates with.
    #[serde(rename = "ocppCsmsUrl")]
    pub ocpp_csms_url: String,
    #[serde(rename = "ocppInterface")]
    pub ocpp_interface: OcppInterfaceEnumType,
    #[serde(rename = "ocppTransport")]
    pub ocpp_transport: OcppTransportEnumType,
    #[serde(rename = "ocppVersion")]
    pub ocpp_version: OcppVersionEnumType,
    /// This field specifies the security profile used when connecting to the CSMS with this
    /// NetworkConnectionProfile.
    #[serde(rename = "securityProfile")]
    pub security_profile: i64,
    pub vpn: Option<VpnType>,
}

/// APN
/// urn:x-oca:ocpp:uid:2:233134
/// Collection of configuration data needed to make a data-connection over a cellular
/// network.
///
/// NOTE: When asking a GSM modem to dial in, it is possible to specify which mobile operator
/// should be used. This can be done with the mobile country code (MCC) in combination with a
/// mobile network code (MNC). Example: If your preferred network is Vodafone Netherlands,
/// the MCC=204 and the MNC=04 which means the key PreferredNetwork = 20404 Some modems
/// allows to specify a preferred network, which means, if this network is not available, a
/// different network is used. If you specify UseOnlyPreferredNetwork and this network is not
/// available, the modem will not dial in.
#[derive(Serialize, Deserialize)]
pub struct ApnType {
    /// APN. APN. URI
    /// urn:x-oca:ocpp:uid:1:568814
    /// The Access Point Name as an URL.
    pub apn: String,
    #[serde(rename = "apnAuthentication")]
    pub apn_authentication: ApnAuthenticationEnumType,
    /// APN. APN. Password
    /// urn:x-oca:ocpp:uid:1:568819
    /// APN Password.
    #[serde(rename = "apnPassword")]
    pub apn_password: Option<String>,
    /// APN. APN. User_ Name
    /// urn:x-oca:ocpp:uid:1:568818
    /// APN username.
    #[serde(rename = "apnUserName")]
    pub apn_user_name: Option<String>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// APN. Preferred_ Network. Mobile_ Network_ ID
    /// urn:x-oca:ocpp:uid:1:568822
    /// Preferred network, written as MCC and MNC concatenated. See note.
    #[serde(rename = "preferredNetwork")]
    pub preferred_network: Option<String>,
    /// APN. SIMPIN. PIN_ Code
    /// urn:x-oca:ocpp:uid:1:568821
    /// SIM card pin code.
    #[serde(rename = "simPin")]
    pub sim_pin: Option<i64>,
    /// APN. Use_ Only_ Preferred_ Network. Indicator
    /// urn:x-oca:ocpp:uid:1:568824
    /// Default: false. Use only the preferred Network, do
    /// not dial in when not available. See Note.
    #[serde(rename = "useOnlyPreferredNetwork")]
    pub use_only_preferred_network: Option<bool>,
}

/// VPN
/// urn:x-oca:ocpp:uid:2:233268
/// VPN Configuration settings
#[derive(Serialize, Deserialize)]
pub struct VpnType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// VPN. Group. Group_ Name
    /// urn:x-oca:ocpp:uid:1:569274
    /// VPN group.
    pub group: Option<String>,
    /// VPN. Key. VPN_ Key
    /// urn:x-oca:ocpp:uid:1:569276
    /// VPN shared secret.
    pub key: String,
    /// VPN. Password. Password
    /// urn:x-oca:ocpp:uid:1:569275
    /// VPN Password.
    pub password: String,
    /// VPN. Server. URI
    /// urn:x-oca:ocpp:uid:1:569272
    /// VPN Server Address
    pub server: String,
    #[serde(rename = "type")]
    pub vpn_type_type: VpnEnumType,
    /// VPN. User. User_ Name
    /// urn:x-oca:ocpp:uid:1:569273
    /// VPN User
    pub user: String,
}

/// APN. APN_ Authentication. APN_ Authentication_ Code
/// urn:x-oca:ocpp:uid:1:568828
/// Authentication method.
#[derive(Serialize, Deserialize)]
pub enum ApnAuthenticationEnumType {
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "CHAP")]
    Chap,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "PAP")]
    Pap,
}

/// Applicable Network Interface.
#[derive(Serialize, Deserialize)]
pub enum OcppInterfaceEnumType {
    Wired0,
    Wired1,
    Wired2,
    Wired3,
    Wireless0,
    Wireless1,
    Wireless2,
    Wireless3,
}

/// Communication_ Function. OCPP_ Transport. OCPP_ Transport_ Code
/// urn:x-oca:ocpp:uid:1:569356
/// Defines the transport protocol (e.g. SOAP or JSON). Note: SOAP is not supported in OCPP
/// 2.0, but is supported by other versions of OCPP.
#[derive(Serialize, Deserialize)]
pub enum OcppTransportEnumType {
    #[serde(rename = "JSON")]
    Json,
    #[serde(rename = "SOAP")]
    Soap,
}

/// Communication_ Function. OCPP_ Version. OCPP_ Version_ Code
/// urn:x-oca:ocpp:uid:1:569355
/// Defines the OCPP version used for this communication function.
#[derive(Serialize, Deserialize)]
pub enum OcppVersionEnumType {
    #[serde(rename = "OCPP12")]
    Ocpp12,
    #[serde(rename = "OCPP15")]
    Ocpp15,
    #[serde(rename = "OCPP16")]
    Ocpp16,
    #[serde(rename = "OCPP20")]
    Ocpp20,
}

/// VPN. Type. VPN_ Code
/// urn:x-oca:ocpp:uid:1:569277
/// Type of VPN
#[derive(Serialize, Deserialize)]
pub enum VpnEnumType {
    #[serde(rename = "IKEv2")]
    IkEv2,
    #[serde(rename = "IPSec")]
    IpSec,
    #[serde(rename = "L2TP")]
    L2Tp,
    #[serde(rename = "PPTP")]
    Pptp,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SetVariableMonitoringRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "setMonitoringData")]
    pub set_monitoring_data: Vec<SetMonitoringDataType>,
}

/// Class to hold parameters of SetVariableMonitoring request.
#[derive(Serialize, Deserialize)]
pub struct SetMonitoringDataType {
    pub component: ComponentType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// An id SHALL only be given to replace an existing monitor. The Charging Station handles
    /// the generation of id's for new monitors.
    pub id: Option<i64>,
    /// The severity that will be assigned to an event that is triggered by this monitor. The
    /// severity range is 0-9, with 0 as the highest and 9 as the lowest severity level.
    ///
    /// The severity levels have the following meaning: +
    /// *0-Danger* +
    /// Indicates lives are potentially in danger. Urgent attention is needed and action should
    /// be taken immediately. +
    /// *1-Hardware Failure* +
    /// Indicates that the Charging Station is unable to continue regular operations due to
    /// Hardware issues. Action is required. +
    /// *2-System Failure* +
    /// Indicates that the Charging Station is unable to continue regular operations due to
    /// software or minor hardware issues. Action is required. +
    /// *3-Critical* +
    /// Indicates a critical error. Action is required. +
    /// *4-Error* +
    /// Indicates a non-urgent error. Action is required. +
    /// *5-Alert* +
    /// Indicates an alert event. Default severity for any type of monitoring event.  +
    /// *6-Warning* +
    /// Indicates a warning event. Action may be required. +
    /// *7-Notice* +
    /// Indicates an unusual event. No immediate action is required. +
    /// *8-Informational* +
    /// Indicates a regular operational event. May be used for reporting, measuring throughput,
    /// etc. No action is required. +
    /// *9-Debug* +
    /// Indicates information useful to developers for debugging, not useful during operations.
    pub severity: i64,
    /// Monitor only active when a transaction is ongoing on a component relevant to this
    /// transaction. Default = false.
    pub transaction: Option<bool>,
    #[serde(rename = "type")]
    pub set_monitoring_data_type_type: MonitorEnumType,
    /// Value for threshold or delta monitoring.
    /// For Periodic or PeriodicClockAligned this is the interval in seconds.
    pub value: f64,
    pub variable: VariableType,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SetVariablesRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "setVariableData")]
    pub set_variable_data: Vec<SetVariableDataType>,
}

#[derive(Serialize, Deserialize)]
pub struct SetVariableDataType {
    #[serde(rename = "attributeType")]
    pub attribute_type: Option<AttributeEnumType>,
    /// Value to be assigned to attribute of variable.
    ///
    /// The Configuration Variable
    /// &lt;&lt;configkey-configuration-value-size,ConfigurationValueSize&gt;&gt; can be used to
    /// limit SetVariableData.attributeValue and VariableCharacteristics.valueList. The max size
    /// of these values will always remain equal.
    #[serde(rename = "attributeValue")]
    pub attribute_value: String,
    pub component: ComponentType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    pub variable: VariableType,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SignCertificateRequest {
    #[serde(rename = "certificateType")]
    pub certificate_type: Option<CertificateSigningUseEnumType>,
    /// The Charging Station SHALL send the public key in form of a Certificate Signing Request
    /// (CSR) as described in RFC 2986 [22] and then PEM encoded, using the
    /// &lt;&lt;signcertificaterequest,SignCertificateRequest&gt;&gt; message.
    pub csr: String,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct StatusNotificationRequest {
    /// The id of the connector within the EVSE for which the status is reported.
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
    #[serde(rename = "connectorStatus")]
    pub connector_status: ConnectorStatusEnumType,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// The id of the EVSE to which the connector belongs for which the the status is reported.
    #[serde(rename = "evseId")]
    pub evse_id: i64,
    /// The time for which the status is reported. If absent time of receipt of the message will
    /// be assumed.
    pub timestamp: String,
}

/// This contains the current status of the Connector.
#[derive(Serialize, Deserialize)]
pub enum ConnectorStatusEnumType {
    Available,
    Faulted,
    Occupied,
    Reserved,
    Unavailable,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct TransactionEventRequest {
    /// The maximum current of the connected cable in Ampere (A).
    #[serde(rename = "cableMaxCurrent")]
    pub cable_max_current: Option<i64>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "eventType")]
    pub event_type: TransactionEventEnumType,
    pub evse: Option<EvseType>,
    #[serde(rename = "idToken")]
    pub id_token: Option<IdTokenType>,
    #[serde(rename = "meterValue")]
    pub meter_value: Option<Vec<MeterValueType>>,
    /// If the Charging Station is able to report the number of phases used, then it SHALL
    /// provide it. When omitted the CSMS may be able to determine the number of phases used via
    /// device management.
    #[serde(rename = "numberOfPhasesUsed")]
    pub number_of_phases_used: Option<i64>,
    /// Indication that this transaction event happened when the Charging Station was offline.
    /// Default = false, meaning: the event occurred when the Charging Station was online.
    pub offline: Option<bool>,
    /// This contains the Id of the reservation that terminates as a result of this transaction.
    #[serde(rename = "reservationId")]
    pub reservation_id: Option<i64>,
    /// Incremental sequence number, helps with determining if all messages of a transaction have
    /// been received.
    #[serde(rename = "seqNo")]
    pub seq_no: i64,
    /// The date and time at which this transaction event occurred.
    pub timestamp: String,
    #[serde(rename = "transactionInfo")]
    pub transaction_info: TransactionType,
    #[serde(rename = "triggerReason")]
    pub trigger_reason: TriggerReasonEnumType,
}

/// Transaction
/// urn:x-oca:ocpp:uid:2:233318
#[derive(Serialize, Deserialize)]
pub struct TransactionType {
    #[serde(rename = "chargingState")]
    pub charging_state: Option<ChargingStateEnumType>,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// The ID given to remote start request (&lt;&lt;requeststarttransactionrequest,
    /// RequestStartTransactionRequest&gt;&gt;. This enables to CSMS to match the started
    /// transaction to the given start request.
    #[serde(rename = "remoteStartId")]
    pub remote_start_id: Option<i64>,
    #[serde(rename = "stoppedReason")]
    pub stopped_reason: Option<ReasonEnumType>,
    /// Transaction. Time_ Spent_ Charging. Elapsed_ Time
    /// urn:x-oca:ocpp:uid:1:569415
    /// Contains the total time that energy flowed from EVSE to EV during the transaction (in
    /// seconds). Note that timeSpentCharging is smaller or equal to the duration of the
    /// transaction.
    #[serde(rename = "timeSpentCharging")]
    pub time_spent_charging: Option<i64>,
    /// This contains the Id of the transaction.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
}

/// This contains the type of this event.
/// The first TransactionEvent of a transaction SHALL contain: "Started" The last
/// TransactionEvent of a transaction SHALL contain: "Ended" All others SHALL contain:
/// "Updated"
#[derive(Serialize, Deserialize)]
pub enum TransactionEventEnumType {
    Ended,
    Started,
    Updated,
}

/// Transaction. State. Transaction_ State_ Code
/// urn:x-oca:ocpp:uid:1:569419
/// Current charging state, is required when state
/// has changed.
#[derive(Serialize, Deserialize)]
pub enum ChargingStateEnumType {
    Charging,
    #[serde(rename = "EVConnected")]
    EvConnected,
    Idle,
    #[serde(rename = "SuspendedEV")]
    SuspendedEv,
    #[serde(rename = "SuspendedEVSE")]
    SuspendedEvse,
}

/// Transaction. Stopped_ Reason. EOT_ Reason_ Code
/// urn:x-oca:ocpp:uid:1:569413
/// This contains the reason why the transaction was stopped. MAY only be omitted when Reason
/// is "Local".
#[derive(Serialize, Deserialize)]
pub enum ReasonEnumType {
    DeAuthorized,
    EmergencyStop,
    EnergyLimitReached,
    #[serde(rename = "EVDisconnected")]
    EvDisconnected,
    GroundFault,
    ImmediateReset,
    Local,
    LocalOutOfCredit,
    MasterPass,
    Other,
    OvercurrentFault,
    PowerLoss,
    PowerQuality,
    Reboot,
    Remote,
    #[serde(rename = "SOCLimitReached")]
    SocLimitReached,
    #[serde(rename = "StoppedByEV")]
    StoppedByEv,
    TimeLimitReached,
    Timeout,
}

/// Reason the Charging Station sends this message to the CSMS
#[derive(Serialize, Deserialize)]
pub enum TriggerReasonEnumType {
    AbnormalCondition,
    Authorized,
    CablePluggedIn,
    ChargingRateChanged,
    ChargingStateChanged,
    Deauthorized,
    EnergyLimitReached,
    #[serde(rename = "EVCommunicationLost")]
    EvCommunicationLost,
    #[serde(rename = "EVConnectTimeout")]
    EvConnectTimeout,
    #[serde(rename = "EVDeparted")]
    EvDeparted,
    #[serde(rename = "EVDetected")]
    EvDetected,
    MeterValueClock,
    MeterValuePeriodic,
    RemoteStart,
    RemoteStop,
    ResetCommand,
    SignedDataReceived,
    StopAuthorized,
    TimeLimitReached,
    Trigger,
    UnlockCommand,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct TriggerMessageRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    pub evse: Option<EvseType>,
    #[serde(rename = "requestedMessage")]
    pub requested_message: MessageTriggerEnumType,
}

/// Type of message to be triggered.
#[derive(Serialize, Deserialize)]
pub enum MessageTriggerEnumType {
    BootNotification,
    FirmwareStatusNotification,
    Heartbeat,
    LogStatusNotification,
    MeterValues,
    PublishFirmwareStatusNotification,
    SignChargingStationCertificate,
    SignCombinedCertificate,
    SignV2GCertificate,
    StatusNotification,
    TransactionEvent,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct UnlockConnectorRequest {
    /// This contains the identifier of the connector that needs to be unlocked.
    #[serde(rename = "connectorId")]
    pub connector_id: i64,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// This contains the identifier of the EVSE for which a connector needs to be unlocked.
    #[serde(rename = "evseId")]
    pub evse_id: i64,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct UnpublishFirmwareRequest {
    /// The MD5 checksum over the entire firmware file as a hexadecimal string of length 32.
    pub checksum: String,
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct UpdateFirmwareRequest {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    pub firmware: FirmwareType,
    /// The Id of this request
    #[serde(rename = "requestId")]
    pub request_id: i64,
    /// This specifies how many times Charging Station must try to download the firmware before
    /// giving up. If this field is not present, it is left to Charging Station to decide how
    /// many times it wants to retry.
    pub retries: Option<i64>,
    /// The interval in seconds after which a retry may be attempted. If this field is not
    /// present, it is left to Charging Station to decide how long to wait between attempts.
    #[serde(rename = "retryInterval")]
    pub retry_interval: Option<i64>,
}

/// Firmware
/// urn:x-enexis:ecdm:uid:2:233291
/// Represents a copy of the firmware that can be loaded/updated on the Charging Station.
#[derive(Serialize, Deserialize)]
pub struct FirmwareType {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
    /// Firmware. Install. Date_ Time
    /// urn:x-enexis:ecdm:uid:1:569462
    /// Date and time at which the firmware shall be installed.
    #[serde(rename = "installDateTime")]
    pub install_date_time: Option<String>,
    /// Firmware. Location. URI
    /// urn:x-enexis:ecdm:uid:1:569460
    /// URI defining the origin of the firmware.
    pub location: String,
    /// Firmware. Retrieve. Date_ Time
    /// urn:x-enexis:ecdm:uid:1:569461
    /// Date and time at which the firmware shall be retrieved.
    #[serde(rename = "retrieveDateTime")]
    pub retrieve_date_time: String,
    /// Firmware. Signature. Signature
    /// urn:x-enexis:ecdm:uid:1:569464
    /// Base64 encoded firmware signature.
    pub signature: Option<String>,
    /// Certificate with which the firmware was signed.
    /// PEM encoded X.509 certificate.
    #[serde(rename = "signingCertificate")]
    pub signing_certificate: Option<String>,
}

