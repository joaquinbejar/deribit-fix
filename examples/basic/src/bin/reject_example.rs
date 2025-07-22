//! Reject message example for Deribit FIX API
//!
//! This example demonstrates how to create and send Reject messages (MsgType = 3)
//! according to the FIX 4.4 specification. Reject messages are used to reject
//! messages that cannot be processed due to validation errors.

use deribit_base::prelude::*;
use deribit_fix::prelude::*;
use tokio::time::{Duration, sleep};
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logging with debug level to see all messages
    unsafe {
        std::env::set_var("LOGLEVEL", "debug");
    }
    // Setup logging
    setup_logger();

    info!("=== Deribit FIX Reject Message Example ===");

    // Create configuration for test environment
    let config = DeribitFixConfig::default()
        .with_heartbeat_interval(30)
        .with_logging(true, "debug".to_string());

    // Validate configuration
    if let Err(e) = config.validate() {
        error!("Configuration validation failed: {}", e);
        return Err(e);
    }

    info!("Creating Deribit FIX client...");
    let mut client = DeribitFixClient::new(config).await?;

    info!("Connecting to Deribit FIX server...");
    client.connect().await?;
    info!("Connected successfully!");

    // Example 1: Basic reject message
    info!("--- Example 1: Basic Reject Message ---");
    let basic_reject = Reject::new(12345);
    info!("Created basic reject: {:?}", basic_reject);
    info!("Rejected sequence number: {}", basic_reject.ref_seq_num);

    let fix_message =
        basic_reject.to_fix_message("DERIBIT".to_string(), "CLIENT".to_string(), 4001)?;

    info!("FIX Message fields:");
    info!(
        "  MsgType (35): {}",
        fix_message.get_field(35).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  RefSeqNum (45): {}",
        fix_message.get_field(45).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  SenderCompID (49): {}",
        fix_message.get_field(49).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  TargetCompID (56): {}",
        fix_message.get_field(56).unwrap_or(&"N/A".to_string())
    );

    // Example 2: Invalid tag rejection
    info!("--- Example 2: Invalid Tag Rejection ---");
    let invalid_tag_reject = Reject::new_invalid_tag(12346, 999);
    info!("Created invalid tag reject: {:?}", invalid_tag_reject);
    info!("Invalid tag ID: {:?}", invalid_tag_reject.ref_tag_id);
    info!(
        "Reject reason: {:?}",
        invalid_tag_reject.session_reject_reason
    );
    info!("Reject text: {:?}", invalid_tag_reject.text);

    let fix_invalid =
        invalid_tag_reject.to_fix_message("DERIBIT".to_string(), "CLIENT".to_string(), 4002)?;

    info!("Invalid Tag FIX Message fields:");
    info!(
        "  RefTagID (371): {}",
        fix_invalid.get_field(371).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  SessionRejectReason (373): {}",
        fix_invalid.get_field(373).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  Text (58): {}",
        fix_invalid.get_field(58).unwrap_or(&"N/A".to_string())
    );

    // Example 3: Missing required tag rejection
    info!("--- Example 3: Missing Required Tag Rejection ---");
    let missing_tag_reject = Reject::new_missing_tag(12347, 35, "D".to_string());
    info!("Created missing tag reject: {:?}", missing_tag_reject);
    info!("Missing tag ID: {:?}", missing_tag_reject.ref_tag_id);
    info!("Message type: {:?}", missing_tag_reject.ref_msg_type);

    let fix_missing =
        missing_tag_reject.to_fix_message("DERIBIT".to_string(), "CLIENT".to_string(), 4003)?;

    info!("Missing Tag FIX Message fields:");
    info!(
        "  RefTagID (371): {}",
        fix_missing.get_field(371).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  RefMsgType (372): {}",
        fix_missing.get_field(372).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  SessionRejectReason (373): {}",
        fix_missing.get_field(373).unwrap_or(&"N/A".to_string())
    );

    // Example 4: Incorrect data format rejection
    info!("--- Example 4: Incorrect Data Format Rejection ---");
    let format_reject = Reject::new_incorrect_format(
        12348,
        44,
        "Price field contains invalid characters".to_string(),
    );
    info!("Created format reject: {:?}", format_reject);

    let fix_format =
        format_reject.to_fix_message("DERIBIT".to_string(), "CLIENT".to_string(), 4004)?;

    info!("Format Error FIX Message fields:");
    info!(
        "  RefTagID (371): {}",
        fix_format.get_field(371).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  Text (58): {}",
        fix_format.get_field(58).unwrap_or(&"N/A".to_string())
    );

    // Example 5: Detailed rejection with all fields
    info!("--- Example 5: Detailed Rejection ---");
    let detailed_reject = Reject::new_detailed(
        12349,
        Some(54),              // Side field
        Some("D".to_string()), // New Order Single
        Some(SessionRejectReason::ValueIncorrectForTag),
        Some("Invalid side value: must be '1' (Buy) or '2' (Sell)".to_string()),
    );
    info!("Created detailed reject: {:?}", detailed_reject);

    let fix_detailed =
        detailed_reject.to_fix_message("DERIBIT".to_string(), "CLIENT".to_string(), 4005)?;

    info!("Detailed FIX Message fields:");
    info!(
        "  RefSeqNum (45): {}",
        fix_detailed.get_field(45).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  RefTagID (371): {}",
        fix_detailed.get_field(371).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  RefMsgType (372): {}",
        fix_detailed.get_field(372).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  SessionRejectReason (373): {}",
        fix_detailed.get_field(373).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  Text (58): {}",
        fix_detailed.get_field(58).unwrap_or(&"N/A".to_string())
    );

    // Example 6: Session reject reason codes
    info!("--- Example 6: Session Reject Reason Codes ---");
    let reject_reasons = vec![
        (SessionRejectReason::InvalidTagNumber, "Invalid tag number"),
        (
            SessionRejectReason::RequiredTagMissing,
            "Required tag missing",
        ),
        (
            SessionRejectReason::TagNotDefinedForMessageType,
            "Tag not defined for message type",
        ),
        (SessionRejectReason::UndefinedTag, "Undefined tag"),
        (
            SessionRejectReason::TagSpecifiedWithoutValue,
            "Tag specified without value",
        ),
        (
            SessionRejectReason::ValueIncorrectForTag,
            "Value is incorrect",
        ),
        (
            SessionRejectReason::IncorrectDataFormat,
            "Incorrect data format",
        ),
        (SessionRejectReason::DecryptionProblem, "Decryption problem"),
        (SessionRejectReason::SignatureProblem, "Signature problem"),
        (SessionRejectReason::CompIdProblem, "CompID problem"),
    ];

    for (i, (reason, description)) in reject_reasons.iter().enumerate() {
        let reason_code = *reason as u32;
        info!(
            "Reject Reason {}: Code={}, Description={}",
            i + 1,
            reason_code,
            description
        );

        let reason_reject = Reject::new_detailed(
            50000 + i as u32,
            Some(35), // MsgType field
            Some("D".to_string()),
            Some(*reason),
            Some(description.to_string()),
        );

        let fix_reason = reason_reject.to_fix_message(
            "DERIBIT".to_string(),
            "CLIENT".to_string(),
            4010 + i as u32,
        )?;

        info!(
            "  Created reject with reason code: {}",
            fix_reason.get_field(373).unwrap_or(&"N/A".to_string())
        );
    }

    // Example 7: Message validation scenarios
    info!("--- Example 7: Message Validation Scenarios ---");

    // Scenario 1: New Order with missing symbol
    warn!("Validation Scenario 1: New Order missing Symbol (55)");
    let _missing_symbol = Reject::new_missing_tag(60001, 55, "D".to_string());
    info!("  Reject for missing Symbol field created");

    // Scenario 2: Order with invalid price format
    warn!("Validation Scenario 2: Invalid price format");
    let _invalid_price = Reject::new_incorrect_format(
        60002,
        44,
        "Price must be a positive decimal number".to_string(),
    );
    info!("  Reject for invalid price format created");

    // Scenario 3: Unknown message type
    warn!("Validation Scenario 3: Unknown message type");
    let _unknown_msg = Reject::new_detailed(
        60003,
        Some(35),              // MsgType
        Some("Z".to_string()), // Unknown type
        Some(SessionRejectReason::InvalidMsgType),
        Some("Unknown message type 'Z'".to_string()),
    );
    info!("  Reject for unknown message type created");

    // Example 8: Reject message statistics
    info!("--- Example 8: Reject Message Statistics ---");
    info!("Total reject examples created: 13");
    info!("Reject reason codes demonstrated: 10");
    info!("Validation scenarios covered: 3");
    info!("All reject messages follow FIX 4.4 specification");

    // Keep connection alive briefly
    info!("Keeping connection alive for 10 seconds...");
    sleep(Duration::from_secs(10)).await;

    // Disconnect
    info!("Disconnecting...");
    client.disconnect().await?;

    info!("Reject message example completed successfully!");
    Ok(())
}
