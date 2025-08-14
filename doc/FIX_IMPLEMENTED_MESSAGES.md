# FIX Messages Implemented in deribit-fix

This document lists the FIX messages implemented in the `deribit-fix` crate, grouped by functional area. For each message you will find:

- Purpose: what the message is used for
- Location: the Rust source file where it is implemented
- Key FIX tags: the core tags populated by our builders (standard header/trailer tags like 8/9/10 are omitted)

All message building uses the common `MessageBuilder` to assemble FIX fields, ensuring proper headers, timestamps, and serialization.

## Administrative (Session)

### Logon (35=A)
- Purpose: Open a FIX session and authenticate to Deribit using Deribit-specific authentication (RawData + SHA256(secret)).
- Location: `src/session/fix_session.rs` (method `logon`)
- Key FIX tags:
  - 96 RawData (timestamp.nonce)
  - 98 EncryptMethod (0)
  - 108 HeartBtInt (heartbeat interval)
  - 553 Username
  - 554 Password (base64(sha256(RawData ++ access_secret)))

### Logout (35=5)
- Purpose: Close the FIX session gracefully.
- Location: `src/session/fix_session.rs` (method `logout`)
- Key FIX tags:
  - 58 Text (logout reason)

### Heartbeat (35=0)
- Purpose: Keep-alive message to maintain session connectivity and measure liveness.
- Location: `src/message/admin.rs` (`Heartbeat`)
- Key FIX tags:
  - 112 TestReqID (optional, when responding to TestRequest)

### Test Request (35=1)
- Purpose: Force a Heartbeat response to verify connectivity when no traffic has been received.
- Location: `src/message/admin.rs` (`TestRequest`)
- Key FIX tags:
  - 112 TestReqID

### Resend Request (35=2)
- Purpose: Request retransmission of a range of messages when a sequence gap is detected.
- Location: `src/message/admin.rs` (`ResendRequest`)
- Key FIX tags:
  - 7 BeginSeqNo
  - 16 EndSeqNo (0 = all since BeginSeqNo)

### Reject (35=3)
- Purpose: Indicate a message could not be processed; includes references to the offending message/tag.
- Location: `src/message/admin.rs` (`Reject`)
- Key FIX tags:
  - 45 RefSeqNum
  - 371 RefTagID (optional)
  - 372 RefMsgType (optional)
  - 373 SessionRejectReason (optional)
  - 58 Text (optional)

### Sequence Reset (35=4)
- Purpose: Adjust incoming/outgoing sequence numbers to recover from desynchronization.
- Location: Handled at session level; no dedicated builder struct yet.

---

## Market Data

### Market Data Request (35=V)
- Purpose: Request market data snapshots and/or real-time updates.
- Location: `src/message/market_data.rs` (`MarketDataRequest`)
- Key FIX tags:
  - 262 MDReqID
  - 263 SubscriptionRequestType (0/1/2)
  - 264 MarketDepth (optional)
  - 265 MDUpdateType (optional; 0=Full, 1=Incremental)
  - 267 NoMDEntryTypes; 269 MDEntryType (repeating)
  - 146 NoRelatedSym; 55 Symbol (repeating)

### Market Data Request Reject (35=Y)
- Purpose: Inform that a market data request cannot be fulfilled.
- Location: `src/message/market_data.rs` (`MarketDataRequestReject`)
- Key FIX tags:
  - 262 MDReqID
  - 281 MDReqRejReason
  - 58 Text (optional)

### Market Data Snapshot/Full Refresh (35=W)
- Purpose: Send a full snapshot of the current market data for a symbol.
- Location: `src/message/market_data.rs` (`MarketDataSnapshotFullRefresh`)
- Key FIX tags:
  - 55 Symbol
  - 262 MDReqID (optional)
  - 268 NoMDEntries; for each entry:
    - 269 MDEntryType, 270 MDEntryPx, 271 MDEntrySize, 272 MDEntryDate
    - 54 Side (for trades), 100009 DeribitTradeId (custom)

### Market Data Incremental Refresh (35=X)
- Purpose: Send incremental updates to previously subscribed market data.
- Location: `src/message/market_data.rs` (`MarketDataIncrementalRefresh`)
- Key FIX tags:
  - 55 Symbol
  - 262 MDReqID (optional)
  - 268 NoMDEntries; for each entry:
    - 279 MDUpdateAction, 269 MDEntryType, 270 MDEntryPx, 271 MDEntrySize, 272 MDEntryDate
    - 54 Side (for trades), 100009 DeribitTradeId (custom)

---

## Security Information

### Security List Request (35=x)
- Purpose: Request a snapshot (and optionally subscribe) to the list of tradable instruments.
- Location: `src/message/security_list.rs` (`SecurityListRequest`)
- Key FIX tags:
  - 320 SecurityReqID
  - 559 SecurityListRequestType (0/4)
  - 263 SubscriptionRequestType (optional)
  - 15 Currency, 5544 SecondaryCurrency (optional)
  - 167 SecurityType (optional)
  - 9013 DisplayMulticastInstrumentID, 9018 DisplayIncrementSteps (custom)

### Security List (35=y)
- Purpose: Return the list of securities and their attributes.
- Location: `src/message/security_list.rs` (`SecurityList`)
- Key FIX tags:
  - 320 SecurityReqID, 322 SecurityResponseID, 560 SecurityRequestResult
  - 146 NoRelatedSym; for each security include e.g. 55 Symbol, 107 SecurityDesc, 167 SecurityType, etc.

### Security Definition Request (35=c)
- Purpose: Request the definition of a specific security or instrument.
- Location: `src/message/security_definition.rs` (`SecurityDefinitionRequest`)
- Key FIX tags:
  - 320 SecurityReqID
  - 55 Symbol (optional)
  - 167 SecurityType (optional)
  - 15 Currency (optional)
  - 58 Text (optional)

### Security Definition (35=d)
- Purpose: Provide the definition and attributes of a security.
- Location: `src/message/security_definition.rs` (`SecurityDefinition`)
- Key FIX tags:
  - 320 SecurityReqID
  - 322 SecurityResponseID
  - 560 SecurityRequestResult
  - 55 Symbol
  - 107 SecurityDesc
  - 167 SecurityType
  - ... (other security attributes as in Security List)

### Security Status Request (35=e)
- Purpose: Request the current status of a security.
- Location: `src/message/security_status.rs` (`SecurityStatusRequest`)
- Key FIX tags:
  - 324 SecurityStatusReqID
  - 55 Symbol
  - 263 SubscriptionRequestType (optional)

### Security Status (35=f)
- Purpose: Report the current status of a security.
- Location: `src/message/security_status.rs` (`SecurityStatus`)
- Key FIX tags:
  - 324 SecurityStatusReqID
  - 55 Symbol
  - 965 SecurityTradingStatus
  - 326 SecurityTradingEvent (optional)
  - 58 Text (optional)

---

## Order Management

### New Order Single (35=D)
- Purpose: Submit a new order (market/limit/stop variants via fields).
- Location: `src/message/orders/new_order.rs` (`NewOrderSingle`)
- Key FIX tags:
  - 11 ClOrdID, 54 Side, 38 OrderQty, 44 Price, 55 Symbol
  - 62 ValidUntilTime (optional), 18 ExecInst (post-only/reduce-only),
  - 40 OrdType, 59 TimeInForce, 99 StopPx, 1138 DisplayQty, 1088 RefreshQty,
  - 854 QtyType, 211 PegOffsetValue, 1094 PegPriceType,
  - 100010 DeribitLabel (custom), 100012 DeribitAdvOrderType (custom), 9008 DeribitMMProtection (custom), 5127 DeribitConditionTriggerMethod (custom)

### Order Cancel Request (35=F)
- Purpose: Cancel an existing order by OrigClOrdID, ClOrdID or DeribitLabel.
- Location: `src/message/orders/cancel_request.rs` (`OrderCancelRequest`)
- Key FIX tags:
  - 11 ClOrdID (optional), 41 OrigClOrdID (optional)
  - 100010 DeribitLabel (optional), 55 Symbol (conditional), 15 Currency (optional)

### Order Cancel/Replace Request (35=G)
- Purpose: Modify an existing order (quantity, price, parameters).
- Location: `src/message/orders/cancel_replace_request.rs` (`OrderCancelReplaceRequest`)
- Key FIX tags:
  - 41 OrigClOrdID, 11 ClOrdID, 55 Symbol, 54 Side, 60 TransactTime
  - 38 OrderQty, 44 Price, 40 OrdType, 59 TimeInForce, 99 StopPx,
  - 1138 DisplayQty, 854 QtyType, 100010 DeribitLabel, 9008 DeribitMMProtection (custom)

### Order Cancel Reject (35=9)
- Purpose: Indicate a cancel request was rejected and why.
- Location: `src/message/orders/cancel_reject.rs` (`OrderCancelReject`)
- Key FIX tags:
  - 52 SendingTime
  - 39 OrdStatus (optional), 102 CxlRejReason (optional), 434 CxlRejResponseTo (optional)
  - 58 Text (optional), 11 ClOrdID (optional), 41 OrigClOrdID (optional), 100010 DeribitLabel (optional)

### Order Mass Cancel Request (35=q)
- Purpose: Cancel multiple orders by filter (symbol, security type, Deribit label, all orders).
- Location: `src/message/orders/mass_cancel.rs` (`OrderMassCancelRequest`)
- Key FIX tags:
  - 11 ClOrdID
  - 530 MassCancelRequestType (1/5/7/10)
  - Conditional: 100010 DeribitLabel | 167 SecurityType | 55 Symbol
  - 15 Currency (optional), 9031 FreezeQuotes (custom)

### Order Mass Cancel Report (35=r)
- Purpose: Acknowledge mass cancel request with details of affected orders or rejection.
- Location: `src/message/orders/mass_cancel.rs` (`OrderMassCancelReport`)
- Key FIX tags:
  - 530 MassCancelRequestType
  - 11 ClOrdID (optional), 37 OrderID (optional)
  - 531 MassCancelResponse (optional), 532 MassCancelRejectReason (optional)
  - 533 TotalAffectedOrders (optional), 534 NoAffectedOrders (optional), 535+ Affected OrigClOrdIDs (repeating)
  - 58 Text (optional)

### Order Mass Status Request (35=AF)
- Purpose: Query status for all orders or a specific order (by OrigClOrdID/ClOrdID/DeribitLabel).
- Location: `src/message/orders/mass_status.rs` (`OrderMassStatusRequest`)
- Key FIX tags:
  - 584 MassStatusReqID
  - 585 MassStatusReqType (1/7)
  - 9014 MassStatusReqIDType (0=OrigClOrdId, 1=ClOrdId, 2=DeribitLabel)
  - 15 Currency (optional), 55 Symbol (optional)

---

## Execution Reports

### Execution Report (35=8)
- Purpose: Notify order state changes and trade executions (new, partial fill, fill, reject, replace).
- Location: `src/message/orders/execution_report.rs` (`ExecutionReport`)
- Key FIX tags:
  - 37 OrderID, 11 ClOrdID, 17 ExecID, 150 ExecType, 39 OrdStatus
  - 55 Symbol, 54 Side, 151 LeavesQty, 14 CumQty, 38 OrderQty
  - 60 TransactTime
  - Optional: 41 OrigClOrdID, 6 AvgPx, 31 LastPx, 32 LastQty, 44 Price, 58 Text, 103 OrdRejReason, 100010 DeribitLabel

---

## Position Management

### Request For Positions (35=AN)
- Purpose: Request current positions (snapshot/stream according to request).
- Location: `src/session/fix_session.rs` (method `request_positions`)
- Key FIX tags:
  - 710 PosReqID, 724 PosReqType (0 = Positions)
  - 263 SubscriptionRequestType, 715 ClearingBusinessDate

Note: Position Report (35=AP) consumption is supported by the type system and session processing; a dedicated builder is not provided in this crate.

---

## Quote Management (Market Making)

### Quote Request (35=R)
- Purpose: Request a quote for a given instrument and side/quantity.
- Location: `src/message/quotes/quote_request.rs` (`QuoteRequest`)
- Key FIX tags:
  - 131 QuoteReqID, 55 Symbol, 537 QuoteType, 54 Side, 38 OrderQty
  - 62 ValidUntilTime, 303 QuoteRequestType, 59 TimeInForce, 110 MinQty, 63 SettlType (all optional)
  - 100010 DeribitLabel (optional), 1300 MarketSegmentID (optional)

### Quote Request Reject (35=AG)
- Purpose: Reject a quote request with a reason.
- Location: `src/message/quotes/quote_request_reject.rs` (`QuoteRequestReject`)
- Key FIX tags:
  - 131 QuoteReqID, 658 QuoteRequestRejectReason
  - 58 Text, 55 Symbol, 146 NoRelatedSym, 100010 DeribitLabel (all optional)

### Quote Status Report (35=AI)
- Purpose: Report the status of a quote (accepted, rejected, expired, etc.).
- Location: `src/message/quotes/quote_status_report.rs` (`QuoteStatusReport`)
- Key FIX tags:
  - 649 QuoteStatusReportID, 297 QuoteStatus, 55 Symbol, 60 TransactTime
  - 131 QuoteReqID, 117 QuoteID, 301 QuoteRespLevel, 300 QuoteRejectReason (optional)
  - 54 Side, 132 BidPx, 133 OfferPx, 134 BidSize, 135 OfferSize, 62 ValidUntilTime, 631 MidPx (optional)
  - 58 Text, 100010 DeribitLabel (optional)

### Mass Quote (35=i)
- Purpose: Submit a batch of quotes (one or two-sided) across instruments.
- Location: `src/message/quotes/mass_quote.rs` (`MassQuote`)
- Key FIX tags:
  - 117 QuoteID, 302 QuoteSetID, 295 TotQuoteEntries
  - 131 QuoteReqID, 301 QuoteRespLevel, 293 DefaultBidSize, 294 DefaultOfferSize, 367 QuoteSetValidUntilTime (optional)
  - 1 Account, 59 TimeInForce, 100010 DeribitLabel (optional)
  - Quote entries: optionally uses standard FIX repeating groups (via `use_standard_repeating_groups` flag) or backward-compatible custom grouped fields with base tag 2000+ (contains QuoteEntryID, Symbol, Side, BidPx/OfferPx, BidSize/OfferSize)

### Mass Quote Acknowledgement (35=b)
- Purpose: Acknowledge a mass quote, with per-entry acceptance/rejection details.
- Location: `src/message/quotes/mass_quote_acknowledgement.rs` (`MassQuoteAcknowledgement`)
- Key FIX tags:
  - 117 QuoteID, 297 QuoteAckStatus
  - 131 QuoteReqID, 300 QuoteRejectReason, 301 QuoteRespLevel, 302 QuoteSetID, 295 TotQuoteEntries (optional)
  - 1 Account, 58 Text, 100010 DeribitLabel (optional)
  - Entry acks: optionally uses standard FIX repeating groups (via `use_standard_repeating_groups` flag) or backward-compatible custom grouped fields with base tag 3000+ (includes QuoteEntryID, Symbol, QuoteAckStatus, optional reasons/prices/sizes)

### Quote Cancel (35=Z)
- Purpose: Cancel existing quotes (all, by symbol, by type, etc.).
- Location: `src/message/quotes/quote_cancel.rs` (`QuoteCancel`)
- Key FIX tags:
  - 117 QuoteID, 298 QuoteCancelType (1/2/3/4/5)
  - 131 QuoteReqID, 301 QuoteRespLevel, 1 Account, 302 QuoteSetID, 311 UnderlyingSymbol, 295 TotQuoteEntries, 336/625 Trading Session IDs (optional)
  - 58 Text, 100010 DeribitLabel (optional)
  - Entries: optionally uses standard FIX repeating groups (via `use_standard_repeating_groups` flag) or backward-compatible custom grouped fields with base tag 4000+ (includes QuoteEntryID, Symbol, Side, RejectReason)

---

## RFQ System (Request for Quote)

### RFQ Request (35=AH)
- Purpose: Request quotes for block trades (single or multi-leg), manual or automatic RFQ.
- Location: `src/message/quotes/rfq_request.rs` (`RfqRequest`)
- Key FIX tags:
  - 644 RFQReqID, 146 NoRelatedSym, 55 Symbol, 38 OrderQty
  - 303 RFQRequestType, 263 SubscriptionRequestType, 54 Side, 62 ValidUntilTime, 59 TimeInForce, 63/64 Settlement info (optional)
  - 15 Currency, 1 Account, 440 ClearingAccount, 77 PositionEffect, 336/625 Trading Session IDs, 58 Text, 100010 DeribitLabel (optional)
  - Legs: 555 NoLegs, custom grouped fields 5000+ (LegSymbol, LegSide, LegQty, LegSettlType/Date, LegPrice)

---

## Trade Reporting

### Trade Capture Report Request (35=AD)
- Purpose: Request trade reports (all/matched/unmatched/advisories) with optional streaming.
- Location: `src/message/trade/trade_capture_report_request.rs` (`TradeCaptureReportRequest`)
- Key FIX tags:
  - 568 TradeRequestID, 569 TradeRequestType, 263 SubscriptionRequestType (optional)
  - 571 TradeReportID (optional), 55 Symbol (optional), 54 Side (optional), 38 OrderQty (optional)
  - 60 TransactTime (from), 126 ExpireTime (to) (optional time range)
  - 715 ClearingBusinessDate, 75 TradeDate, 1 Account, 440 ClearingAccount, 1300 MarketSegmentID (optional)
  - 336/625 Trading Session IDs, 58 Text, 100010 DeribitLabel (optional)

### Trade Capture Report Request Ack (35=AQ)
- Purpose: Acknowledge a trade capture report request, including status/result and totals.
- Location: `src/message/trade/trade_capture_report_request_ack.rs` (`TradeCaptureReportRequestAck`)
- Key FIX tags:
  - 568 TradeRequestID, 749 TradeRequestStatus, 750 TradeRequestResult (optional)
  - 571 TradeReportID, 748 TotNumTradeReports (optional)
  - 55 Symbol, 442 MultiLegReportingType, 725/726 ResponseTransportType/Destination (optional)
  - 1 Account, 440 ClearingAccount, 1300 MarketSegmentID (optional)
  - 336/625 Trading Session IDs, 58 Text, 100010 DeribitLabel (optional)

### Trade Capture Report (35=AE)
- Purpose: Report execution details of a trade or trade corrections/cancellations.
- Location: `src/message/trade/trade_capture_report.rs` (`TradeCaptureReport`)
- Key FIX tags:
  - 571 TradeReportID, 55 Symbol, 54 Side
  - 53 Quantity, 32 LastQty, 31 LastPx, 75 TradeDate, 60 TransactTime
  - 1003 TradeID, 1040 SecondaryTradeID, 1041 FirmTradeID (optional)
  - 487 TradeReportTransType, 856 TradeReportType (optional)
  - 568 TradeRequestID, 828 TrdType, 829 TradeSubType (optional)
  - 38 OrderQty, 381 GrossTradeAmt, 126 ExecTime, 64 SettlDate (optional)
  - 1 Account, 440 ClearingAccount, 77 PositionEffect, 715 ClearingBusinessDate, 336/625 Trading Session IDs, 1300 MarketSegmentID, 58 Text, 100010 DeribitLabel (optional)

---

## User Management

### User Request (35=BE)
- Purpose: User login/logout, password change, and user status request.
- Location: `src/message/user/user_request.rs` (`UserRequest`)
- Key FIX tags:
  - 923 UserRequestID, 924 UserRequestType, 553 Username
  - 554 Password, 925 NewPassword (optional)
  - 95 RawDataLength, 96 RawData (base64) (optional)
  - 926 UserStatus, 927 UserStatusText (optional)
  - 100010 DeribitLabel (optional)

### User Response (35=BF)
- Purpose: Respond to user requests with status and optional data.
- Location: `src/message/user/user_response.rs` (`UserResponse`)
- Key FIX tags:
  - 923 UserRequestID, 553 Username, 926 UserStatus
  - 927 UserStatusText (optional), 95/96 RawData fields (optional)
  - 100010 DeribitLabel (optional)

---

## Risk Management (Market Maker Protection)

### MM Protection Limits (35=MM)
- Purpose: Set, update, query, or remove market maker protection risk limits.
- Location: `src/message/risk/mm_protection_limits.rs` (`MMProtectionLimits`)
- Key FIX tags (custom unless otherwise noted):
  - 9001 MMProtectionReqID, 9002 MMProtectionAction, 9003 MMProtectionScope
  - Scope qualifiers: 55 Symbol, 311 UnderlyingSymbol, 9004 InstrumentGroup (optional)
  - Limits: 9005 MaxPositionLimit, 9006 MaxOrderQtyLimit, 9007 MaxOrdersLimit, 9009 TimeWindowSeconds
  - Greeks: 9010 DeltaLimit, 9011 VegaLimit, 9012 GammaLimit, 9013 ThetaLimit
  - 9014 TotalRiskLimit, 9015 ValidFrom, 9016 ValidUntil
  - 336 TradingSessionID, 1 Account, 453 Parties, 58 Text, 100010 DeribitLabel (optional)

### MM Protection Limits Result/Reject (35=MR)
- Purpose: Report the outcome of an MM protection request, including current limits.
- Location: `src/message/risk/mm_protection_limits_result.rs` (`MMProtectionLimitsResult`)
- Key FIX tags:
  - 9001 MMProtectionReqID, 9002 Action, 9003 Scope
  - 9017 ResultStatus, 9018 ProcessingTime, 9019 RejectReason (optional)
  - Current limits: 9020..9028, validity: 9029..9030
  - 55/311/9004 for scope qualifiers; 9031 AffectedInstrumentsCount (optional)
  - 1 Account, 453 Parties, 58 Text, 100010 DeribitLabel (optional)

### MM Protection Reset (35=MZ)
- Purpose: Reset MM protection limits and/or counters (soft/hard/scheduled/emergency).
- Location: `src/message/risk/mm_protection_reset.rs` (`MMProtectionReset`)
- Key FIX tags:
  - 9032 ResetReqID, 9033 ResetType, 9034 ResetReason, 9003 Scope
  - 55 Symbol, 311 UnderlyingSymbol, 9004 InstrumentGroup (optional)
  - 9035 ResetEffectiveTime, 9036 ResetExpiryTime (optional)
  - Flags: 9037 ForceReset, 9038 NotifyAllParticipants
  - Counter resets: 9039..9044 for position/order/volume/time-window/greeks/risk flags
  - 1 Account, 453 Parties, 336/625 Trading Session IDs, 58 Text, 100010 DeribitLabel (optional)

---

## Notes

- All timestamps use `YYYYMMDD-HH:MM:SS.sss` format as strings, aligned with FIX 4.4 and Deribit’s documentation.
- Custom tags in the 9000+ range follow Deribit FIX API conventions.
- Headers (8/9/35/34/49/56/52) and trailers (10) are automatically added by the builder and are omitted from the tag lists for brevity.


