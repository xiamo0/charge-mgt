//! OCPP 2.1 Confirmation (Response) Messages — All 91 messages

// Block A — Security
pub mod security_event_notification_conf;

// Block B — Provisioning
pub mod boot_notification_conf;
pub mod get_base_report_conf;
pub mod get_report_conf;
pub mod get_variables_conf;
pub mod heartbeat_conf;
pub mod notify_report_conf;
pub mod reset_conf;
pub mod set_network_profile_conf;
pub mod set_variables_conf;

// Block C — Authorization
pub mod authorize_conf;
pub mod clear_cache_conf;

// Block D — LocalAuthList
pub mod get_local_list_version_conf;
pub mod send_local_list_conf;

// Block E — Transactions
pub mod get_transaction_status_conf;
pub mod transaction_event_conf;

// Block F — RemoteControl
pub mod request_start_transaction_conf;
pub mod request_stop_transaction_conf;
pub mod trigger_message_conf;
pub mod unlock_connector_conf;

// Block G — Availability
pub mod change_availability_conf;
pub mod status_notification_conf;

// Block H — Reservation
pub mod cancel_reservation_conf;
pub mod reservation_status_update_conf;
pub mod reserve_now_conf;

// Block I — TariffAndCost (2.1 enhanced)
pub mod change_transaction_tariff_conf;
pub mod clear_tariffs_conf;
pub mod cost_updated_conf;
pub mod get_tariffs_conf;
pub mod notify_settlement_conf;
pub mod notify_web_payment_started_conf;
pub mod set_default_tariff_conf;
pub mod vat_number_validation_conf;

// Block J — MeterValues
pub mod meter_values_conf;

// Block K — SmartCharging
pub mod clear_charging_profile_conf;
pub mod cleared_charging_limit_conf;
pub mod get_charging_profiles_conf;
pub mod get_composite_schedule_conf;
pub mod notify_charging_limit_conf;
pub mod notify_ev_charging_needs_conf;
pub mod notify_ev_charging_schedule_conf;
pub mod notify_priority_charging_conf;
pub mod pull_dynamic_schedule_update_conf;
pub mod report_charging_profiles_conf;
pub mod set_charging_profile_conf;
pub mod update_dynamic_schedule_conf;
pub mod use_priority_charging_conf;

// Block L — Firmware
pub mod firmware_status_notification_conf;
pub mod publish_firmware_conf;
pub mod publish_firmware_status_notification_conf;
pub mod unpublish_firmware_conf;
pub mod update_firmware_conf;

// Block M — Certificates
pub mod certificate_signed_conf;
pub mod delete_certificate_conf;
pub mod get_15118_ev_certificate_conf;
pub mod get_certificate_chain_status_conf;
pub mod get_certificate_status_conf;
pub mod get_installed_certificate_ids_conf;
pub mod install_certificate_conf;
pub mod sign_certificate_conf;

// Block N — Diagnostics
pub mod adjust_periodic_event_stream_conf;
pub mod clear_variable_monitoring_conf;
pub mod close_periodic_event_stream_conf;
pub mod customer_information_conf;
pub mod get_log_conf;
pub mod get_monitoring_report_conf;
pub mod get_periodic_event_stream_conf;
pub mod log_status_notification_conf;
pub mod notify_customer_information_conf;
pub mod notify_event_conf;
pub mod notify_monitoring_report_conf;
pub mod notify_periodic_event_stream_conf;
pub mod open_periodic_event_stream_conf;
pub mod set_monitoring_base_conf;
pub mod set_monitoring_level_conf;
pub mod set_variable_monitoring_conf;

// Block O — Display
pub mod clear_display_message_conf;
pub mod get_display_messages_conf;
pub mod notify_display_messages_conf;
pub mod set_display_message_conf;

// Block P — DataTransfer
pub mod data_transfer_conf;

// Block Q — Bidirectional / V2X (2.1 NEW)
pub mod afrr_signal_conf;
pub mod notify_allowed_energy_transfer_conf;

// Block R — DERControl (2.1 NEW)
pub mod clear_der_control_conf;
pub mod get_der_control_conf;
pub mod notify_der_alarm_conf;
pub mod notify_der_start_stop_conf;
pub mod report_der_control_conf;
pub mod set_der_control_conf;

// Block S — BatterySwap (2.1 NEW)
pub mod battery_swap_conf;
pub mod request_battery_swap_conf;
