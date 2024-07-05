//!  Provides methods to read and query DTCs on the ECU, as well as grabbing Env data about each DTC

use crate::uds::UDSClientSession;

pub use automotive_diag::uds::DtcSubFunction;
use automotive_diag::uds::UdsCommand;

impl UDSClientSession {
    /// Returns the number of DTCs stored on the ECU
    /// matching the provided status_mask
    ///
    /// ## Returns
    /// Returns a tuple of the given information:
    /// 1. (u8) - DTCStatusAvailabilityMask
    /// 2. ([DTCFormatType]) - Format of the DTCs
    /// 3. (u16) - Number of DTCs which match the status mask
    pub fn uds_get_number_of_dtcs_by_status_mask(&mut self, status_mask: u8) {
        self.send_command_with_response(
            UdsCommand::ReadDTCInformation,
            &[
                DtcSubFunction::ReportNumberOfDtcByStatusMask as u8,
                status_mask,
            ],
        );
    }

    /// Returns a list of DTCs stored on the ECU
    /// matching the provided status_mask
    pub fn uds_get_dtcs_by_status_mask(&mut self, status_mask: u8) {
        self.send_command_with_response(
            UdsCommand::ReadDTCInformation,
            &[DtcSubFunction::ReportDtcByStatusMask as u8, status_mask],
        );
    }

    pub fn uds_get_dtc_snapshot_record_by_dtc_number(
        &mut self,
        dtc_mask_record: u32,
        snapshot_record_number: u8,
    ) {
        self.send_command_with_response(
            UdsCommand::ReadDTCInformation,
            &[
                DtcSubFunction::ReportDtcSnapshotRecordByDtcNumber as u8,
                (dtc_mask_record >> 16) as u8,
                (dtc_mask_record >> 8) as u8,
                dtc_mask_record as u8,
                snapshot_record_number,
            ],
        );
    }

    pub fn uds_get_dtc_snapshot_identification(&mut self) {
        self.send_command_with_response(
            UdsCommand::ReadDTCInformation,
            &[DtcSubFunction::ReportDtcSnapshotIdentifier as u8],
        );
    }

    pub fn uds_get_dtc_snapshot_record_by_record_number(&mut self, snapshot_record_number: u8) {
        self.send_command_with_response(
            UdsCommand::ReadDTCInformation,
            &[
                DtcSubFunction::ReportDtcSnapshotRecordByRecordNumber as u8,
                snapshot_record_number,
            ],
        );
    }

    /// Returns the DTCExtendedData record(s) associated with the provided DTC mask and record number.
    /// For the record_number, 0xFE implies all OBD records. and 0xFF implies all records.
    ///
    /// ## Returns
    /// This function will return the ECUs full response if successful
    pub fn uds_get_dtc_extended_data_record_by_dtc_number(
        &mut self,
        dtc: u32,
        extended_data_record_number: u8,
    ) {
        self.send_command_with_response(
            UdsCommand::ReadDTCInformation,
            &[
                DtcSubFunction::ReportDtcExtendedDataRecordByDtcNumber as u8,
                (dtc >> 16) as u8, // High byte
                (dtc >> 8) as u8,  // Mid byte
                dtc as u8,         // Low byte
                extended_data_record_number,
            ],
        );
    }

    /// Returns the number of DTCs stored on the ECU that match the provided severity and status mask
    pub fn uds_get_number_of_dtcs_by_severity_mask_record(
        &mut self,
        severity_mask: u8,
        status_mask: u8,
    ) {
        self.send_command_with_response(
            UdsCommand::ReadDTCInformation,
            &[
                DtcSubFunction::ReportNumberOfDtcBySeverityMaskRecord as u8,
                severity_mask,
                status_mask,
            ],
        );
    }

    /// Returns a list of DTCs who's severity mask matches the provided mask
    pub fn uds_get_dtcs_by_severity_mask_record(&mut self, severity_mask: u8, status_mask: u8) {
        self.send_command_with_response(
            UdsCommand::ReadDTCInformation,
            &[
                DtcSubFunction::ReportDtcBySeverityMaskRecord as u8,
                severity_mask,
                status_mask,
            ],
        );
    }

    /// Returns the severity status of a provided DTC
    pub fn uds_get_severity_information_of_dtc(&mut self, dtc: u32) {
        self.send_command_with_response(
            UdsCommand::ReadDTCInformation,
            &[
                DtcSubFunction::ReportSeverityInformationOfDtc as u8,
                (dtc >> 16) as u8,
                (dtc >> 8) as u8,
                dtc as u8,
            ],
        );
    }

    /// Returns a list of all DTCs that the ECU can return
    pub fn uds_get_supported_dtc(&mut self) {
        self.send_command_with_response(
            UdsCommand::ReadDTCInformation,
            &[DtcSubFunction::ReportSupportedDtc as u8],
        );
    }

    /// Returns the first failed DTC to be detected since the last DTC clear operation
    pub fn uds_get_first_test_failed_dtc(&mut self) {
        self.send_command_with_response(
            UdsCommand::ReadDTCInformation,
            &[DtcSubFunction::ReportFirstTestFailedDtc as u8],
        );
    }

    /// Returns the first confirmed DTC to be detected since the last DTC clear operationn
    pub fn uds_get_first_confirmed_dtc(&mut self) {
        self.send_command_with_response(
            UdsCommand::ReadDTCInformation,
            &[DtcSubFunction::ReportFirstConfirmedDtc as u8],
        );
    }

    /// Returns the most recent DTC to be detected since the last DTC clear operation
    pub fn uds_get_most_recent_test_failed_dtc(&mut self) {
        self.send_command_with_response(
            UdsCommand::ReadDTCInformation,
            &[DtcSubFunction::ReportMostRecentTestFailedDtc as u8],
        );
    }

    /// Returns the most recent DTC to be detected since the last DTC clear operation
    pub fn uds_get_most_recent_confirmed_dtc(&mut self) {
        self.send_command_with_response(
            UdsCommand::ReadDTCInformation,
            &[DtcSubFunction::ReportMostRecentConfirmedDtc as u8],
        );
    }

    /// Returns the current number of 'pre-failed' DTCs on the ECU, which have not yet been confirmed
    /// as being either 'pending' or 'confirmed'
    ///
    /// ## Returns
    /// This function will return a vector of information, where each element is a tuple containing the following values:
    /// 1. (u32) - DTC Code
    /// 2. (u8) - Fault detection counter
    pub fn uds_get_dtc_fault_detection_counter(&mut self) {
        self.send_command_with_response(
            UdsCommand::ReadDTCInformation,
            &[DtcSubFunction::ReportDtcFaultDetectionCounter as u8],
        );
    }

    /// Returns a list of DTCs that have a permanent status
    pub fn uds_get_dtc_with_permanent_status(&mut self) {
        self.send_command_with_response(
            UdsCommand::ReadDTCInformation,
            &[DtcSubFunction::ReportDtcWithPermanentStatus as u8],
        );
    }
}
