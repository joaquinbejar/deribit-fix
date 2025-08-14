# FIX Tags Coverage by Message (deribit-fix)

This document enumerates, per FIX message, the tags our builders currently set (“Implemented”) and notable tags from the
Deribit FIX API that are not emitted yet (“Missing”). Standard header/trailer fields (8/9/10, 35, 34, 49, 56, 52) are
omitted for brevity.

Notes:

- Deribit-specific custom tags (9xxx/1000xx) follow their public FIX API.
- Repeating groups are summarized; in some places we use simplified custom grouping where noted.

## Administrative (Session)

### Logon (35=A)

- Implemented: 108, 95, 96, 553, 554, 9002, 9001, 9004, 9005, 9007, 9009, 9010, 9015, 9018
- Missing: None (all documented Deribit tags now implemented)

### Logout (35=5)

- Implemented: 58, 9003
- Missing: —

### Heartbeat (35=0)

- Implemented: 112 (optional)
- Missing: —

### Test Request (35=1)

- Implemented: 112
- Missing: —

### Resend Request (35=2)

- Implemented: 7, 16
- Missing: —

### Reject (35=3)

- Implemented: 45, 371, 372, 373, 58
- Missing: —

### Sequence Reset (35=4)

- Implemented: 36, 123 (optional)
- Missing: —

### Business Message Reject (35=j)

- Implemented: 372, 380, 379 (optional), 58 (optional)
- Missing: —

---

## Market Data

### Market Data Request (35=V)

- Implemented: 262, 263, 264, 265, 267, 269 (group), 146, 55 (group), 9011, 9012, 100007, 100008
- Missing: —

### Market Data Request Reject (35=Y)

- Implemented: 262, 281, 58 (optional)
- Missing: —

### Market Data Snapshot/Full Refresh (35=W)

- Implemented: 55, 262 (optional), 268 + entries: 269, 270, 271, 272, 54 (for trades), 100009 DeribitTradeId
- Missing: —

### Market Data Incremental Refresh (35=X)

- Implemented: 55, 262 (optional), 268 + entries: 279, 269, 270, 271, 272, 54 (for trades), 100009 DeribitTradeId
- Missing: —

---

## Security Information

### Security List Request (35=x)

- Implemented: 320, 559, 263 (optional), 15 (optional), 5544 (optional), 167 (optional), 9013, 9018
- Missing: —

### Security List (35=y)

- Implemented: 320, 322, 560, 146 + security fields: 55, 107, 167, 201, 202, 947, 15, 1524, 2576, 969, 311, 225, 541,
  1079, 562, 63, 120, 479, 231, 454, 455, 456, 1205, 1206, 1208, 965
- Missing: —

### Security Definition Request (35=c)

- Implemented: 320, 55 (optional), 167 (optional), 15 (optional), 58 (optional)
- Missing: —

### Security Definition (35=d)

- Implemented: 320, 322, 560, 55, 107, 167, ... (other security attributes as in Security List)
- Missing: —

### Security Status Request (35=e)

- Implemented: 324, 55, 263 (optional)
- Missing: —

### Security Status (35=f)

- Implemented: 324, 55, 965, 326 (optional), 58 (optional)
- Missing: —

---

## Position Management

### Request For Positions (35=AN)

- Implemented: 710, 724, 263 (optional), 715 (optional), 146 (optional), 55 (group, optional)
- Missing: —

### Position Report (35=AP)

- Implemented: 710, 55, 703 (optional), 6 (optional), 1247 (optional), 1248 (optional), 704 (optional)

---

## Order Management

### New Order Single (35=D)

- Implemented: 11, 54, 38, 44, 55, 62 (optional), 18 (optional), 40 (optional), 59 (optional), 99 (optional), 1138 (optional), 1088 (optional), 854 (optional), 211 (optional), 1094 (optional), 100010 (optional), 100012 (optional), 9008 (optional), 5127 (optional)

### Order Cancel Request (35=F)

- Implemented: 11 (optional), 41 (optional), 100010 (optional), 55 (conditional), 15 (optional)
- Missing: —

### Order Cancel/Replace Request (35=G)

- Implemented: 41, 11, 55, 54, 60, 38 (optional), 44 (optional), 40 (optional), 59 (optional), 99 (optional), 1138 (
  optional), 854 (optional), 100010 (optional), 9008 (optional)
- Missing: —

### Order Cancel Reject (35=9)

- Implemented: 52, 39 (optional), 102 (optional), 434 (optional), 58 (optional), 11 (optional), 41 (optional), 100010 (
  optional)
- Missing: —

### Order Mass Cancel Request (35=q)

- Implemented: 11, 530, (conditional) 100010 | 167 | 55, 15 (optional), 9031 (optional)
- Missing: —

### Order Mass Cancel Report (35=r)

- Implemented: 530, 11 (optional), 37 (optional), 531 (optional), 532 (optional), 533 (optional), 534 (optional), 535+
  Affected IDs (optional), 58 (optional)
- Missing: —

### Order Mass Status Request (35=AF)

- Implemented: 584, 585, 9014 (optional), 15 (optional), 55 (optional)
- Missing: —

---

## Execution Reports

### Execution Report (35=8)

- Implemented: 37, 11, 17, 150, 39, 55, 54, 151, 14, 38, 60, 41 (optional), 6 (optional), 31 (optional), 32 (optional),
  44 (optional), 58 (optional), 103 (optional), 100010 (optional), 527 (optional), 40 (optional), 12 (optional), 207 (
  optional), 854 (optional), 231 (optional), 1138 (optional), 100012 (optional), 1188 (optional), 839 (optional), 880 (
  optional), 9008 (optional), 9019 (optional), 302 (optional), 117 (optional), 299 (optional), 18 (optional), 99 (
  optional), 5127 (optional), 851 (optional)
- Missing: —

---

## Position Management

### Request For Positions (35=AN)

- Implemented: 710, 724, 263, 715, symbols (optional)

### Position Report (35=AP)

- Implemented: 710, 55, 703 (optional), 6 (optional), 1247 (optional), 1248 (optional), 704 (optional); Full builder pattern for emission with to_fix_message method
- Missing: —

---

## Quote Management

### Quote Request (35=R)

- Implemented: 131, 55, 537, 54, 38, 62 (optional), 303 (optional), 59 (optional), 110 (optional), 63 (optional), 100010 (optional), 1300 (optional)

### Quote Request Reject (35=AG)

- Implemented: 131, 658, 58 (optional), 55 (optional), 146 (optional), 100010 (optional)
- Missing: —

### Quote Status Report (35=AI)

- Implemented: 649, 297, 55, 60, 131 (optional), 117 (optional), 301 (optional), 300 (optional), 54 (optional), 132 (
  optional), 133 (optional), 134 (optional), 135 (optional), 62 (optional), 631 (optional), 58 (optional), 100010 (
  optional)
- Missing: —

### Mass Quote (35=i)

- Implemented: 117, 302, 295, 131 (optional), 301 (optional), 293 (optional), 294 (optional), 367 (optional), 1 (optional), 59 (optional), 100010 (optional); Full standard repeating group structures optionally supported (via `use_standard_repeating_groups` flag)

### Mass Quote Acknowledgement (35=b)

- Implemented: 117, 297, 131 (optional), 300 (optional), 301 (optional), 302 (optional), 295 (optional), 1 (optional), 58 (optional), 100010 (optional); Full standard repeating group structures optionally supported (via `use_standard_repeating_groups` flag)

### Quote Cancel (35=Z)

- Implemented: 117, 298, 131 (optional), 301 (optional), 1 (optional), 302 (optional), 311 (optional), 295 (optional), 336 (optional), 625 (optional), 58 (optional), 100010 (optional); Full standard repeating group structures optionally supported (via `use_standard_repeating_groups` flag)

---

## RFQ System

### RFQ Request (35=AH)

- Implemented: 644, 146, 55, 38, 303 (optional), 263 (optional), 54 (optional), 62 (optional), 59 (optional), 63 (optional), 64 (optional), 15 (optional), 1 (optional), 440 (optional), 77 (optional), 555 (optional), 336 (optional), 625 (optional), 58 (optional), 100010 (optional)

---

## Trade Reporting

### Trade Capture Report Request (35=AD)

- Implemented: 568, 569, 263 (optional), 571 (optional), 55 (optional), 54 (optional), 38 (optional), 60 (from), 126 (
  to), 715 (optional), 75 (optional), 1 (optional), 440 (optional), 1300 (optional), 336 (optional), 625 (optional),
  58 (optional), 100010 (optional)
- Missing: —

### Trade Capture Report Request Ack (35=AQ)

- Implemented: 568, 749, 750 (optional), 571 (optional), 55 (optional), 748 (optional), 442 (optional), 725 (optional),
  726 (optional), 1 (optional), 440 (optional), 1300 (optional), 336 (optional), 625 (optional), 58 (optional), 100010 (
  optional)
- Missing: —

### Trade Capture Report (35=AE)

- Implemented: 571, 55, 54, 53, 32, 31, 75, 60, 1003 (optional), 1040 (optional), 1041 (optional), 487 (optional), 856 (optional), 568 (optional), 828 (optional), 829 (optional), 38 (optional), 381 (optional), 126 (optional), 64 (optional), 442 (optional), 570 (optional), 423 (optional), 810 (optional), 1 (optional), 440 (optional), 77 (optional), 715 (optional), 336 (optional), 625 (optional), 1300 (optional), 58 (optional), 100010 (optional)

---

## User Management

### User Request (35=BE)

- Implemented: 923, 924, 553, 554 (optional), 925 (optional), 95 (optional), 96 (optional, base64), 926 (optional),
  927 (optional), 100010 (optional)
- Missing: —

### User Response (35=BF)

- Implemented: 923, 553, 926, 927 (optional), 95 (optional), 96 (optional, base64), 100010 (optional)
- Missing: —

---

## Risk Management (Market Maker Protection)

### MM Protection Limits (35=MM)

- Implemented: 9001, 9002, 9003, 55 (optional), 311 (optional), 9004 (optional), 9005 (optional), 9006 (optional),
  9007 (optional), 9009 (optional), 9010 (optional), 9011 (optional), 9012 (optional), 9013 (optional), 9014 (optional),
  9015 (optional), 9016 (optional), 336 (optional), 1 (optional), 453 (optional), 58 (optional), 100010 (optional)
- Missing: —

### MM Protection Limits Result/Reject (35=MR)

- Implemented: 9001, 9002, 9003, 9017, 9018, 9019 (optional), 55 (optional), 311 (optional), 9004 (optional),
  9020..9028 (optionals), 9029 (optional), 9030 (optional), 9031 (optional), 1 (optional), 453 (optional), 58 (
  optional), 100010 (optional)
- Missing: —

### MM Protection Reset (35=MZ)

- Implemented: 9032, 9033, 9034, 9003, 55 (optional), 311 (optional), 9004 (optional), 9035 (optional), 9036 (optional),
  9037 (optional), 9038 (optional), 9039..9044 (optionals), 1 (optional), 453 (optional), 336 (optional), 625 (
  optional), 58 (optional), 100010 (optional)
- Missing: —

---

## Summary of Notable Gaps


- Documentation/tests: Add coverage and examples for recently implemented messages.

Status: All core messages, including Security Definition/Status family, are now implemented with practical tag coverage.
Quote messages support optional standard repeating groups.
