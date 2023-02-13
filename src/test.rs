

def_id!(AccountId, "acct_");
def_id!(AlipayAccountId, "aliacc_");
def_id!(ApplicationFeeId, "fee_");
def_id!(ApplicationId, "ca_");
def_id!(ApplicationFeeRefundId, "fr_");
def_id!(BalanceTransactionId, "txn_");
def_id!(BankAccountId, "ba_" | "card_");
def_id!(BillingPortalSessionId, "bps_");
def_id!(BillingPortalConfigurationId, "bpc_");
def_id!(BankTokenId, "btok_");
def_id!(
    #[optional]
    enum BalanceTransactionSourceId {
        ApplicationFee(ApplicationFeeId),
        Charge(ChargeId),
        Dispute(DisputeId),
        ApplicationFeeRefund(ApplicationFeeRefundId),
        IssuingAuthorization(IssuingAuthorizationId),
        IssuingDispute(IssuingDisputeId),
        IssuingTransaction(IssuingTransactionId),
        Payout(PayoutId),
        Refund(RefundId),
        Topup(TopupId),
        Transfer(TransferId),
        TransferReversal(TransferReversalId),
    }
);
def_id!(CardId, "card_");
def_id!(CardTokenId, "tok_");
def_id!(ChargeId, "ch_" | "py_"); // TODO: Understand (and then document) why "py_" is a valid charge id
def_id!(CheckoutSessionId, "cs_");
def_id!(CheckoutSessionItemId: String); // TODO: Figure out what prefix this id has
def_id!(ConnectCollectionTransferId, "connct_");
def_id!(CouponId: String); // N.B. A coupon id can be user-provided so can be any arbitrary string
def_id!(CustomerId, "cus_");
def_id!(DiscountId, "di_");
def_id!(DisputeId, "dp_" | "du_");
def_id!(EphemeralKeyId, "ephkey_");
def_id!(EventId, "evt_");
def_id!(FileId, "file_");
def_id!(FileLinkId, "link_");
def_id!(InvoiceId, "in_", { _ });
def_id!(InvoiceItemId, "ii_");
def_id!(InvoiceLineItemIdWebhook, "il_");

def_id!(
    enum InvoiceLineItemId {
        #[default]
        Item(InvoiceItemId),
        Subscription(SubscriptionLineId),
        InvoiceLineItemIdWebhook(InvoiceLineItemIdWebhook),
    }
);
def_id!(IssuingAuthorizationId, "iauth_");
def_id!(IssuingCardId, "ic_");
def_id!(IssuingCardholderId, "ich_");
def_id!(IssuingDisputeId, "idp_");
def_id!(IssuingTransactionId, "ipi_");
def_id!(OrderId, "or_");
def_id!(OrderReturnId, "orret_");
def_id!(MandateId, "mandate_");
def_id!(PaymentIntentId, "pi_");
def_id!(PaymentLinkId, "plink_");
def_id!(PaymentMethodId, "pm_" | "card_" | "src_" | "ba_");
def_id!(
    enum PaymentSourceId {
        #[default]
        Account(AccountId),
        AlipayAccount(AlipayAccountId),
        BankAccount(BankAccountId),
        Card(CardId),
        Source(SourceId),
    }
);
def_id!(PayoutId, "po_");
def_id!(
    enum PayoutDestinationId {
        #[default]
        BankAccount(BankAccountId),
        Card(CardId),
    }
);
def_id!(PersonId, "person_");
def_id!(PlanId: String); // N.B. A plan id can be user-provided so can be any arbitrary string
def_id!(PlatformTaxFeeId, "ptf");
def_id!(PriceId, "price_");
def_id!(ProductId: String); // N.B. A product id can be user-provided so can be any arbitrary string
def_id!(PromotionCodeId, "promo_");
def_id!(QuoteId, "qt_");
def_id!(RecipientId: String); // FIXME: This doesn't seem to be documented yet
def_id!(RefundId, "re_" | "pyr_");
def_id!(ReserveTransactionId, "rtx");
def_id!(ReviewId, "prv_");
def_id!(ScheduledQueryRunId, "sqr_");
def_id!(SetupAttemptId, "setatt_");
def_id!(SetupIntentId, "seti_");
def_id!(SkuId, "sku_");
def_id!(ShippingRateId, "shr_");
def_id!(SourceId, "src_");
def_id!(SubscriptionId, "sub_");
def_id!(SubscriptionItemId, "si_");
def_id!(SubscriptionLineId, "sli_");
def_id!(SubscriptionScheduleId, "sub_sched_");
def_id!(TaxIdId, "txi_");
def_id!(TaxCodeId, "txcd_");
def_id!(TaxDeductedAtSourceId, "itds");
def_id!(TaxRateId, "txr_");
def_id!(TerminalConfigurationId, "tmc_");
def_id!(TerminalLocationId, "tml_");
def_id!(TerminalReaderId, "tmr_");
def_id!(TestHelpersTestClockId, "clock_");
def_id!(
    enum TokenId {
        #[default]
        Card(CardTokenId),
        Bank(BankTokenId),
    }
);
def_id!(TopupId, "tu_");
def_id!(TransferId, "tr_");
def_id!(TransferReversalId, "trr_");
def_id!(UsageRecordId, "mbur_");
def_id!(UsageRecordSummaryId, "urs_");
def_id!(WebhookEndpointId, "we_");

#[cfg(test)]
mod tests {
    use std::fmt::{Debug, Display};
    use std::str::FromStr;

    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use super::*;

    fn assert_ser_de_roundtrip<T>(id: &str)
    where
        T: DeserializeOwned + Serialize + FromStr + Display + Debug,
        <T as FromStr>::Err: Debug,
    {
        let parsed_id = T::from_str(id).expect("Could not parse id");
        let ser = serde_json::to_string(&parsed_id).expect("Could not serialize id");
        let deser: T = serde_json::from_str(&ser).expect("Could not deserialize id");
        assert_eq!(deser.to_string(), id.to_string());
    }

    fn assert_deser_err<T: DeserializeOwned + Debug>(id: &str) {
        let json_str = format!(r#""{}""#, id);
        let deser: Result<T, _> = serde_json::from_str(&json_str);
        assert!(deser.is_err(), "Expected error, got {:?}", deser);
    }

    #[test]
    fn test_empty_invoice_id_default() {
        #[derive(Deserialize)]
        struct WithInvoiceId {
            id: InvoiceId,
        }

        for body in [json!({"id": ""}), json!({})] {
            let deser: WithInvoiceId = serde_json::from_value(body).expect("Could not deser");
            assert_eq!(deser.id, InvoiceId::none());
        }
    }

    #[test]
    fn test_ser_de_roundtrip() {
        // InvoiceId special cased
        for id in ["in_12345", "in_"] {
            assert_ser_de_roundtrip::<InvoiceId>(id);
        }

        // Single prefix
        assert_ser_de_roundtrip::<PriceId>("price_abc");

        // Case where multiple possible prefixes
        for id in ["re_bcd", "pyr_123"] {
            assert_ser_de_roundtrip::<RefundId>(id);
        }

        // Case where id can be anything
        for id in ["anything", ""] {
            assert_ser_de_roundtrip::<ProductId>(id);
        }

        // Case where enum id
        for id in ["tok_123", "btok_456"] {
            assert_ser_de_roundtrip::<TokenId>(id);
        }
    }

    #[test]
    fn test_deser_err() {
        // InvoiceId special cased
        assert_deser_err::<InvoiceId>("in");

        // Single prefix
        for id in ["sub", ""] {
            assert_deser_err::<SubscriptionId>(id);
        }

        // Case where multiple possible prefixes
        for id in ["abc_bcd", "pyr_123"] {
            assert_deser_err::<PaymentMethodId>(id);
        }

        // Case where enum id
        for id in ["tok_123", "btok_456"] {
            assert_deser_err::<PaymentSourceId>(id);
        }
    }

    #[test]
    fn test_parse_customer() {
        assert!("cus_123".parse::<CustomerId>().is_ok());
        let bad_parse = "zzz_123".parse::<CustomerId>();
        assert!(bad_parse.is_err());
        if let Err(err) = bad_parse {
            assert_eq!(
                format!("{}", err),
                "invalid `CustomerId`, expected id to start with \"cus_\""
            );
        }
    }

    #[test]
    fn test_parse_charge() {
        assert!("ch_123".parse::<ChargeId>().is_ok());
        assert!("py_123".parse::<ChargeId>().is_ok());
        let bad_parse = "zz_123".parse::<ChargeId>();
        assert!(bad_parse.is_err());
        if let Err(err) = bad_parse {
            assert_eq!(
                format!("{}", err),
                "invalid `ChargeId`, expected id to start with \"ch_\" or \"py_\""
            );
        }
    }
}