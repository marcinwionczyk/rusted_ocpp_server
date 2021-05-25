use serde::{Serialize, Deserialize};

// structures created with the help of https://app.quicktype.io/ and json schema provided by
// https://www.openchargealliance.org/

#[derive(Serialize, Deserialize)]
pub struct AuthorizeResponse {
    #[serde(rename = "certificateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_status: Option<AuthorizeCertificateStatusEnumType>,
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "idTokenInfo")]
    pub id_token_info: IdTokenInfoType,
}

/// This class does not get 'AdditionalProperties = false' in the schema generation, so it
/// can be extended with arbitrary JSON properties to allow adding custom data.
#[derive(Serialize, Deserialize)]
pub struct CustomDataType {
    #[serde(rename = "vendorId")]
    pub vendor_id: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_expiry_date_time: Option<String>,
    /// Priority from a business point of view. Default priority is 0, The range is from -9 to 9.
    /// Higher values indicate a higher priority. The chargingPriority in
    /// &lt;&lt;transactioneventresponse,TransactionEventResponse&gt;&gt; overrules this one.
    #[serde(rename = "chargingPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_priority: Option<i64>,
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// Only used when the IdToken is only valid for one or more specific EVSEs, not for the
    /// entire Charging Station.
    #[serde(rename = "evseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<Vec<i64>>,
    #[serde(rename = "groupIdToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id_token: Option<IdTokenType>,
    /// ID_ Token. Language1. Language_ Code
    /// urn:x-oca:ocpp:uid:1:569374
    /// Preferred user interface language of identifier user. Contains a language code as defined
    /// in &lt;&lt;ref-RFC5646,[RFC5646]&gt;&gt;.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language1: Option<String>,
    /// ID_ Token. Language2. Language_ Code
    /// urn:x-oca:ocpp:uid:1:569375
    /// Second preferred user interface language of identifier user. Don’t use when language1 is
    /// omitted, has to be different from language1. Contains a language code as defined in
    /// &lt;&lt;ref-RFC5646,[RFC5646]&gt;&gt;.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language2: Option<String>,
    #[serde(rename = "personalMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_message: Option<MessageContentType>,
    pub status: AuthorizationStatusEnumType,
}

/// Contains a case insensitive identifier to use for the authorization and the type of
/// authorization to support multiple forms of identifiers.
#[derive(Serialize, Deserialize)]
pub struct IdTokenType {
    #[serde(rename = "additionalInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Vec<AdditionalInfoType>>,
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// This defines the type of the additionalIdToken. This is a custom type, so the
    /// implementation needs to be agreed upon by all involved parties.
    #[serde(rename = "type")]
    pub additional_info_type_type: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub format: MessageFormatEnumType,
    /// Message_ Content. Language. Language_ Code
    /// urn:x-enexis:ecdm:uid:1:570849
    /// Message language identifier. Contains a language code as defined in
    /// &lt;&lt;ref-RFC5646,[RFC5646]&gt;&gt;.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// Certificate status information.
/// - if all certificates are valid: return 'Accepted'.
/// - if one of the certificates was revoked, return 'CertificateRevoked'.
#[derive(Serialize, Deserialize)]
pub enum AuthorizeCertificateStatusEnumType {
    Accepted,
    CertChainError,
    CertificateExpired,
    CertificateRevoked,
    ContractCancelled,
    NoCertificateAvailable,
    SignatureError,
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

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct BootNotificationResponse {
    /// This contains the CSMS’s current time.
    #[serde(rename = "currentTime")]
    pub current_time: String,
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// When &lt;&lt;cmn_registrationstatusenumtype,Status&gt;&gt; is Accepted, this contains the
    /// heartbeat interval in seconds. If the CSMS returns something other than Accepted, the
    /// value of the interval field indicates the minimum wait time before sending a next
    /// BootNotification request.
    pub interval: i64,
    pub status: RegistrationStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// Element providing more information about the status.
#[derive(Serialize, Deserialize)]
pub struct StatusInfoType {
    /// Additional text to provide detailed information.
    #[serde(rename = "additionalInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// A predefined code for the reason why the status is returned in this response. The string
    /// is case-insensitive.
    #[serde(rename = "reasonCode")]
    pub reason_code: String,
}

/// This contains whether the Charging Station has been registered
/// within the CSMS.
#[derive(Serialize, Deserialize)]
pub enum RegistrationStatusEnumType {
    Accepted,
    Pending,
    Rejected,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct CancelReservationResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: CancelReservationStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// This indicates the success or failure of the canceling of a reservation by CSMS.
#[derive(Serialize, Deserialize)]
pub enum CancelReservationStatusEnumType {
    Accepted,
    Rejected,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct CertificateSignedResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: CertificateSignedStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// Returns whether certificate signing has been accepted, otherwise rejected.
#[derive(Serialize, Deserialize)]
pub enum CertificateSignedStatusEnumType {
    Accepted,
    Rejected,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ChangeAvailabilityResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: ChangeAvailabilityStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// This indicates whether the Charging Station is able to perform the availability change.
#[derive(Serialize, Deserialize)]
pub enum ChangeAvailabilityStatusEnumType {
    Accepted,
    Rejected,
    Scheduled,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ClearCacheResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: ClearCacheStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// Accepted if the Charging Station has executed the request, otherwise rejected.
#[derive(Serialize, Deserialize)]
pub enum ClearCacheStatusEnumType {
    Accepted,
    Rejected,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ClearChargingProfileResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: ClearChargingProfileStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// Indicates if the Charging Station was able to execute the request.
#[derive(Serialize, Deserialize)]
pub enum ClearChargingProfileStatusEnumType {
    Accepted,
    Unknown,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ClearDisplayMessageResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: ClearMessageStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// Returns whether the Charging Station has been able to remove the message.
#[derive(Serialize, Deserialize)]
pub enum ClearMessageStatusEnumType {
    Accepted,
    Unknown,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ClearedChargingLimitResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ClearVariableMonitoringResponse {
    #[serde(rename = "clearMonitoringResult")]
    pub clear_monitoring_result: Vec<ClearMonitoringResultType>,
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Serialize, Deserialize)]
pub struct ClearMonitoringResultType {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// Id of the monitor of which a clear was requested.
    pub id: i64,
    pub status: ClearMonitoringStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// Result of the clear request for this monitor, identified by its Id.
#[derive(Serialize, Deserialize)]
pub enum ClearMonitoringStatusEnumType {
    Accepted,
    NotFound,
    Rejected,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct CostUpdatedResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct CustomerInformationResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: CustomerInformationStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// Indicates whether the request was accepted.
#[derive(Serialize, Deserialize)]
pub enum CustomerInformationStatusEnumType {
    Accepted,
    Invalid,
    Rejected,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct DataTransferResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// Data without specified length or format, in response to request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    pub status: DataTransferStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// This indicates the success or failure of the data transfer.
#[derive(Serialize, Deserialize)]
pub enum DataTransferStatusEnumType {
    Accepted,
    Rejected,
    UnknownMessageId,
    UnknownVendorId,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct DeleteCertificateResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: DeleteCertificateStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// Charging Station indicates if it can process the request.
#[derive(Serialize, Deserialize)]
pub enum DeleteCertificateStatusEnumType {
    Accepted,
    Failed,
    NotFound,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct FirmwareStatusNotificationResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct Get15118EvCertificateResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// Raw CertificateInstallationRes response for the EV, Base64 encoded.
    #[serde(rename = "exiResponse")]
    pub exi_response: String,
    pub status: Iso15118EvCertificateStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// Indicates whether the message was processed properly.
#[derive(Serialize, Deserialize)]
pub enum Iso15118EvCertificateStatusEnumType {
    Accepted,
    Failed,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetBaseReportResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: GenericDeviceModelStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// This indicates whether the Charging Station is able to accept this request.
#[derive(Serialize, Deserialize)]
pub enum GenericDeviceModelStatusEnumType {
    Accepted,
    EmptyResultSet,
    NotSupported,
    Rejected,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetCertificateStatusResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// OCSPResponse class as defined in &lt;&lt;ref-ocpp_security_24, IETF RFC 6960&gt;&gt;. DER
    /// encoded (as defined in &lt;&lt;ref-ocpp_security_24, IETF RFC 6960&gt;&gt;), and then
    /// base64 encoded. MAY only be omitted when status is not Accepted.
    #[serde(rename = "ocspResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_result: Option<String>,
    pub status: GetCertificateStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// This indicates whether the charging station was able to retrieve the OCSP certificate
/// status.
#[derive(Serialize, Deserialize)]
pub enum GetCertificateStatusEnumType {
    Accepted,
    Failed,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetChargingProfilesResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: GetChargingProfileStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// This indicates whether the Charging Station is able to process this request and will send
/// &lt;&lt;reportchargingprofilesrequest, ReportChargingProfilesRequest&gt;&gt; messages.
#[derive(Serialize, Deserialize)]
pub enum GetChargingProfileStatusEnumType {
    Accepted,
    NoProfiles,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetCompositeScheduleResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<CompositeScheduleType>,
    pub status: GenericStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// Composite_ Schedule
/// urn:x-oca:ocpp:uid:2:233362
#[derive(Serialize, Deserialize)]
pub struct CompositeScheduleType {
    #[serde(rename = "chargingRateUnit")]
    pub charging_rate_unit: ChargingRateUnitEnumType,
    #[serde(rename = "chargingSchedulePeriod")]
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// Duration of the schedule in seconds.
    pub duration: i64,
    /// The ID of the EVSE for which the
    /// schedule is requested. When evseid=0, the
    /// Charging Station calculated the expected
    /// consumption for the grid connection.
    #[serde(rename = "evseId")]
    pub evse_id: i64,
    /// Composite_ Schedule. Start. Date_ Time
    /// urn:x-oca:ocpp:uid:1:569456
    /// Date and time at which the schedule becomes active. All time measurements within the
    /// schedule are relative to this timestamp.
    #[serde(rename = "scheduleStart")]
    pub schedule_start: String,
}

/// Charging_ Schedule_ Period
/// urn:x-oca:ocpp:uid:2:233257
/// Charging schedule period structure defines a time period in a charging schedule.
#[derive(Serialize, Deserialize)]
pub struct ChargingSchedulePeriodType {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_phases: Option<i64>,
    /// Values: 1..3, Used if numberPhases=1 and if the EVSE is capable of switching the phase
    /// connected to the EV, i.e. ACPhaseSwitchingSupported is defined and true. It’s not allowed
    /// unless both conditions above are true. If both conditions are true, and phaseToUse is
    /// omitted, the Charging Station / EVSE will make the selection on its own.
    #[serde(rename = "phaseToUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_to_use: Option<i64>,
    /// Charging_ Schedule_ Period. Start_ Period. Elapsed_ Time
    /// urn:x-oca:ocpp:uid:1:569240
    /// Start of the period, in seconds from the start of schedule. The value of StartPeriod also
    /// defines the stop time of the previous period.
    #[serde(rename = "startPeriod")]
    pub start_period: i64,
}

/// The unit of measure Limit is
/// expressed in.
#[derive(Serialize, Deserialize)]
pub enum ChargingRateUnitEnumType {
    A,
    W,
}

/// The Charging Station will indicate if it was
/// able to process the request
#[derive(Serialize, Deserialize)]
pub enum GenericStatusEnumType {
    Accepted,
    Rejected,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetDisplayMessagesResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: GetDisplayMessagesStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// Indicates if the Charging Station has Display Messages that match the request criteria in
/// the &lt;&lt;getdisplaymessagesrequest,GetDisplayMessagesRequest&gt;&gt;
#[derive(Serialize, Deserialize)]
pub enum GetDisplayMessagesStatusEnumType {
    Accepted,
    Unknown,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetInstalledCertificateIdsResponse {
    #[serde(rename = "certificateHashDataChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_hash_data_chain: Option<Vec<CertificateHashDataChainType>>,
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: GetInstalledCertificateStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[derive(Serialize, Deserialize)]
pub struct CertificateHashDataChainType {
    #[serde(rename = "certificateHashData")]
    pub certificate_hash_data: CertificateHashDataType,
    #[serde(rename = "certificateType")]
    pub certificate_type: GetCertificateIdUseEnumType,
    #[serde(rename = "childCertificateHashData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_certificate_hash_data: Option<Vec<CertificateHashDataType>>,
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

#[derive(Serialize, Deserialize)]
pub struct CertificateHashDataType {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
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

/// Indicates the type of the requested certificate(s).
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

/// Charging Station indicates if it can process the request.
#[derive(Serialize, Deserialize)]
pub enum GetInstalledCertificateStatusEnumType {
    Accepted,
    NotFound,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetLocalListVersionResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// This contains the current version number of the local authorization list in the Charging
    /// Station.
    #[serde(rename = "versionNumber")]
    pub version_number: i64,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetLogResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// This contains the name of the log file that will be uploaded. This field is not present
    /// when no logging information is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    pub status: LogStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// This field indicates whether the Charging Station was able to accept the request.
#[derive(Serialize, Deserialize)]
pub enum LogStatusEnumType {
    Accepted,
    AcceptedCanceled,
    Rejected,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetMonitoringReportResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: GenericDeviceModelStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetReportResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: GenericDeviceModelStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetTransactionStatusResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// Whether there are still message to be delivered.
    #[serde(rename = "messagesInQueue")]
    pub messages_in_queue: bool,
    /// Whether the transaction is still ongoing.
    #[serde(rename = "ongoingIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ongoing_indicator: Option<bool>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct GetVariablesResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "getVariableResult")]
    pub get_variable_result: Vec<GetVariableResultType>,
}

/// Class to hold results of GetVariables request.
#[derive(Serialize, Deserialize)]
pub struct GetVariableResultType {
    #[serde(rename = "attributeStatus")]
    pub attribute_status: GetVariableStatusEnumType,
    #[serde(rename = "attributeStatusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_status_info: Option<StatusInfoType>,
    #[serde(rename = "attributeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    /// Value of requested attribute type of component-variable. This field can only be empty
    /// when the given status is NOT accepted.
    ///
    /// The Configuration Variable
    /// &lt;&lt;configkey-reporting-value-size,ReportingValueSize&gt;&gt; can be used to limit
    /// GetVariableResult.attributeValue, VariableAttribute.value and EventData.actualValue. The
    /// max size of these values will always remain equal.
    #[serde(rename = "attributeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
    pub component: ComponentType,
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub variable: VariableType,
}

/// A physical or logical component
#[derive(Serialize, Deserialize)]
pub struct ComponentType {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EvseType>,
    /// Name of instance in case the component exists as multiple instances. Case Insensitive.
    /// strongly advised to use Camel Case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// Name of the component. Name should be taken from the list of standardized component names
    /// whenever possible. Case Insensitive. strongly advised to use Camel Case.
    pub name: String,
}

/// EVSE
/// urn:x-oca:ocpp:uid:2:233123
/// Electric Vehicle Supply Equipment
#[derive(Serialize, Deserialize)]
pub struct EvseType {
    /// An id to designate a specific connector (on an EVSE) by connector index number.
    #[serde(rename = "connectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i64>,
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// Identified_ Object. MRID. Numeric_ Identifier
    /// urn:x-enexis:ecdm:uid:1:569198
    /// EVSE Identifier. This contains a number (&gt; 0) designating an EVSE of the Charging
    /// Station.
    pub id: i64,
}

/// Reference key to a component-variable.
#[derive(Serialize, Deserialize)]
pub struct VariableType {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// Name of instance in case the variable exists as multiple instances. Case Insensitive.
    /// strongly advised to use Camel Case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// Name of the variable. Name should be taken from the list of standardized variable names
    /// whenever possible. Case Insensitive. strongly advised to use Camel Case.
    pub name: String,
}

/// Result status of getting the variable.
#[derive(Serialize, Deserialize)]
pub enum GetVariableStatusEnumType {
    Accepted,
    NotSupportedAttributeType,
    Rejected,
    UnknownComponent,
    UnknownVariable,
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
pub struct HeartbeatResponse {
    /// Contains the current time of the CSMS.
    #[serde(rename = "currentTime")]
    pub current_time: String,
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct InstallCertificateResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: InstallCertificateStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// Charging Station indicates if installation was successful.
#[derive(Serialize, Deserialize)]
pub enum InstallCertificateStatusEnumType {
    Accepted,
    Failed,
    Rejected,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct LogStatusNotificationResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct MeterValuesResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct NotifyChargingLimitResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct NotifyCustomerInformationResponse {
    #[serde(rename = "customData")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct NotifyDisplayMessagesResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct NotifyEvChargingNeedsResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: NotifyEvChargingNeedsStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// Returns whether the CSMS has been able to process the message successfully. It does not
/// imply that the evChargingNeeds can be met with the current charging profile.
#[derive(Serialize, Deserialize)]
pub enum NotifyEvChargingNeedsStatusEnumType {
    Accepted,
    Processing,
    Rejected,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct NotifyEvChargingScheduleResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: GenericStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct NotifyEventResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct NotifyMonitoringReportResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct NotifyReportResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct PublishFirmwareRequest {
    /// The MD5 checksum over the entire firmware file as a hexadecimal string of length 32.
    pub checksum: String,
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// This contains a string containing a URI pointing to a
    /// location from which to retrieve the firmware.
    pub location: String,
    /// The Id of the request.
    #[serde(rename = "requestId")]
    pub request_id: i64,
    /// This specifies how many times Charging Station must try
    /// to download the firmware before giving up. If this field is not
    /// present, it is left to Charging Station to decide how many times it wants to retry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    /// The interval in seconds
    /// after which a retry may be
    /// attempted. If this field is not
    /// present, it is left to Charging
    /// Station to decide how long to wait
    /// between attempts.
    #[serde(rename = "retryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i64>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct PublishFirmwareResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: GenericStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct PublishFirmwareStatusNotificationResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ReportChargingProfilesResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct RequestStartTransactionResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: RequestStartStopStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
    /// When the transaction was already started by the Charging Station before the
    /// RequestStartTransactionRequest was received, for example: cable plugged in first. This
    /// contains the transactionId of the already started transaction.
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

/// Status indicating whether the Charging Station accepts the request to start a transaction.
#[derive(Serialize, Deserialize)]
pub enum RequestStartStopStatusEnumType {
    Accepted,
    Rejected,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct RequestStopTransactionResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: RequestStartStopStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ReservationStatusUpdateResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ReserveNowResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: ReserveNowStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// This indicates the success or failure of the reservation.
#[derive(Serialize, Deserialize)]
pub enum ReserveNowStatusEnumType {
    Accepted,
    Faulted,
    Occupied,
    Rejected,
    Unavailable,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct ResetResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: ResetStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// This indicates whether the Charging Station is able to perform the reset.
#[derive(Serialize, Deserialize)]
pub enum ResetStatusEnumType {
    Accepted,
    Rejected,
    Scheduled,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SecurityEventNotificationResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SendLocalListResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: SendLocalListStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// This indicates whether the Charging Station has successfully received and applied the
/// update of the Local Authorization List.
#[derive(Serialize, Deserialize)]
pub enum SendLocalListStatusEnumType {
    Accepted,
    Failed,
    VersionMismatch,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SetChargingProfileResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: ChargingProfileStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// Returns whether the Charging Station has been able to process the message successfully.
/// This does not guarantee the schedule will be followed to the letter. There might be other
/// constraints the Charging Station may need to take into account.
#[derive(Serialize, Deserialize)]
pub enum ChargingProfileStatusEnumType {
    Accepted,
    Rejected,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SetDisplayMessageResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: DisplayMessageStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// This indicates whether the Charging Station is able to display the message.
#[derive(Serialize, Deserialize)]
pub enum DisplayMessageStatusEnumType {
    Accepted,
    NotSupportedMessageFormat,
    NotSupportedPriority,
    NotSupportedState,
    Rejected,
    UnknownTransaction,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SetMonitoringBaseResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: GenericDeviceModelStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SetMonitoringLevelResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: GenericStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SetNetworkProfileResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: SetNetworkProfileStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// Result of operation.
#[derive(Serialize, Deserialize)]
pub enum SetNetworkProfileStatusEnumType {
    Accepted,
    Failed,
    Rejected,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SetVariableMonitoringResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "setMonitoringResult")]
    pub set_monitoring_result: Vec<SetMonitoringResultType>,
}

/// Class to hold result of SetVariableMonitoring request.
#[derive(Serialize, Deserialize)]
pub struct SetMonitoringResultType {
    pub component: ComponentType,
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    /// Id given to the VariableMonitor by the Charging Station. The Id is only returned when
    /// status is accepted. Installed VariableMonitors should have unique id's but the id's of
    /// removed Installed monitors should have unique id's but the id's of removed monitors MAY
    /// be reused.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub status: SetMonitoringStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
    #[serde(rename = "type")]
    pub set_monitoring_result_type_type: MonitorEnumType,
    pub variable: VariableType,
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

/// Status is OK if a value could be returned. Otherwise this will indicate the reason why a
/// value could not be returned.
#[derive(Serialize, Deserialize)]
pub enum SetMonitoringStatusEnumType {
    Accepted,
    Duplicate,
    Rejected,
    UnknownComponent,
    UnknownVariable,
    UnsupportedMonitorType,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SetVariablesResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "setVariableResult")]
    pub set_variable_result: Vec<SetVariableResultType>,
}

#[derive(Serialize, Deserialize)]
pub struct SetVariableResultType {
    #[serde(rename = "attributeStatus")]
    pub attribute_status: SetVariableStatusEnumType,
    #[serde(rename = "attributeStatusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_status_info: Option<StatusInfoType>,
    #[serde(rename = "attributeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    pub component: ComponentType,
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub variable: VariableType,
}

/// Result status of setting the variable.
#[derive(Serialize, Deserialize)]
pub enum SetVariableStatusEnumType {
    Accepted,
    NotSupportedAttributeType,
    RebootRequired,
    Rejected,
    UnknownComponent,
    UnknownVariable,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct SignCertificateResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: GenericStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct StatusNotificationResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct TransactionEventResponse {
    /// Priority from a business point of view. Default priority is 0, The range is from -9 to 9.
    /// Higher values indicate a higher priority. The chargingPriority in
    /// &lt;&lt;transactioneventresponse,TransactionEventResponse&gt;&gt; is temporarily, so it
    /// may not be set in the &lt;&lt;cmn_idtokeninfotype,IdTokenInfoType&gt;&gt; afterwards.
    /// Also the chargingPriority in
    /// &lt;&lt;transactioneventresponse,TransactionEventResponse&gt;&gt; overrules the one in
    /// &lt;&lt;cmn_idtokeninfotype,IdTokenInfoType&gt;&gt;.
    #[serde(rename = "chargingPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_priority: Option<i64>,
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    #[serde(rename = "idTokenInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_info: Option<IdTokenInfoType>,
    /// SHALL only be sent when charging has ended. Final total cost of this transaction,
    /// including taxes. In the currency configured with the Configuration Variable:
    /// &lt;&lt;configkey-currency,`Currency`&gt;&gt;. When omitted, the transaction was NOT
    /// free. To indicate a free transaction, the CSMS SHALL send 0.00.
    #[serde(rename = "totalCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<f64>,
    #[serde(rename = "updatedPersonalMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_personal_message: Option<MessageContentType>,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct TriggerMessageResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: TriggerMessageStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// Indicates whether the Charging Station will send the requested notification or not.
#[derive(Serialize, Deserialize)]
pub enum TriggerMessageStatusEnumType {
    Accepted,
    NotImplemented,
    Rejected,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct UnlockConnectorResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: UnlockStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// This indicates whether the Charging Station has unlocked the connector.
#[derive(Serialize, Deserialize)]
pub enum UnlockStatusEnumType {
    OngoingAuthorizedTransaction,
    UnknownConnector,
    UnlockFailed,
    Unlocked,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct UnpublishFirmwareResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: UnpublishFirmwareStatusEnumType,
}

/// Indicates whether the Local Controller succeeded in unpublishing the firmware.
#[derive(Serialize, Deserialize)]
pub enum UnpublishFirmwareStatusEnumType {
    DownloadOngoing,
    NoFirmware,
    Unpublished,
}

//=================================================================================================

#[derive(Serialize, Deserialize)]
pub struct UpdateFirmwareResponse {
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
    pub status: UpdateFirmwareStatusEnumType,
    #[serde(rename = "statusInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// This field indicates whether the Charging Station was able to accept the request.
#[derive(Serialize, Deserialize)]
pub enum UpdateFirmwareStatusEnumType {
    Accepted,
    AcceptedCanceled,
    InvalidCertificate,
    Rejected,
    RevokedCertificate,
}
