//! OCPP 2.1 Call Messages (Requests) — All 91 messages

// Block A — Security
pub mod security_event_notification;

// Block B — Provisioning
pub mod boot_notification;
pub mod get_base_report;
pub mod get_report;
pub mod get_variables;
pub mod heartbeat;
pub mod notify_report;
pub mod reset;
pub mod set_network_profile;
pub mod set_variables;

// Block C — Authorization
pub mod authorize;
pub mod clear_cache;

// Block D — LocalAuthList
pub mod get_local_list_version;
pub mod send_local_list;

// Block E — Transactions
pub mod get_transaction_status;
pub mod transaction_event;

// Block F — RemoteControl
pub mod request_start_transaction;
pub mod request_stop_transaction;
pub mod trigger_message;
pub mod unlock_connector;

// Block G — Availability
pub mod change_availability;
pub mod status_notification;

// Block H — Reservation
pub mod cancel_reservation;
pub mod reservation_status_update;
pub mod reserve_now;

// Block I — TariffAndCost (2.1 enhanced)
pub mod change_transaction_tariff;
pub mod clear_tariffs;
pub mod cost_updated;
pub mod get_tariffs;
pub mod notify_settlement;
pub mod notify_web_payment_started;
pub mod set_default_tariff;
pub mod vat_number_validation;

// Block J — MeterValues
pub mod meter_values;

// Block K — SmartCharging
pub mod clear_charging_profile;
pub mod cleared_charging_limit;
pub mod get_charging_profiles;
pub mod get_composite_schedule;
pub mod notify_charging_limit;
pub mod notify_ev_charging_needs;
pub mod notify_ev_charging_schedule;
pub mod notify_priority_charging;
pub mod pull_dynamic_schedule_update;
pub mod report_charging_profiles;
pub mod set_charging_profile;
pub mod update_dynamic_schedule;
pub mod use_priority_charging;

// Block L — Firmware
pub mod firmware_status_notification;
pub mod publish_firmware;
pub mod publish_firmware_status_notification;
pub mod unpublish_firmware;
pub mod update_firmware;

// Block M — Certificates
pub mod certificate_signed;
pub mod delete_certificate;
pub mod get_15118_ev_certificate;
pub mod get_certificate_chain_status;
pub mod get_certificate_status;
pub mod get_installed_certificate_ids;
pub mod install_certificate;
pub mod sign_certificate;

// Block N — Diagnostics
pub mod adjust_periodic_event_stream;
pub mod clear_variable_monitoring;
pub mod close_periodic_event_stream;
pub mod customer_information;
pub mod get_log;
pub mod get_monitoring_report;
pub mod get_periodic_event_stream;
pub mod log_status_notification;
pub mod notify_customer_information;
pub mod notify_event;
pub mod notify_monitoring_report;
pub mod notify_periodic_event_stream;
pub mod open_periodic_event_stream;
pub mod set_monitoring_base;
pub mod set_monitoring_level;
pub mod set_variable_monitoring;

// Block O — Display
pub mod clear_display_message;
pub mod get_display_messages;
pub mod notify_display_messages;
pub mod set_display_message;

// Block P — DataTransfer
pub mod data_transfer;

// Block Q — Bidirectional / V2X (2.1 NEW)
pub mod afrr_signal;
pub mod notify_allowed_energy_transfer;

// Block R — DERControl (2.1 NEW)
pub mod clear_der_control;
pub mod get_der_control;
pub mod notify_der_alarm;
pub mod notify_der_start_stop;
pub mod report_der_control;
pub mod set_der_control;

// Block S — BatterySwap (2.1 NEW)
pub mod battery_swap;
pub mod request_battery_swap;
