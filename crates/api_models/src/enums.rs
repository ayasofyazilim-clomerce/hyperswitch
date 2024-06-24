use std::str::FromStr;

pub use common_enums::*;
use utoipa::ToSchema;

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    strum::Display,
    strum::EnumString,
)]

/// The routing algorithm to be used to process the incoming request from merchant to outgoing payment processor or payment method. The default is 'Custom'
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RoutingAlgorithm {
    RoundRobin,
    MaxConversion,
    MinCost,
    Custom,
}

/// A connector is an integration to fulfill payments
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    ToSchema,
    serde::Deserialize,
    serde::Serialize,
    strum::VariantNames,
    strum::EnumIter,
    strum::Display,
    strum::EnumString,
    Hash,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Connector {
    #[cfg(feature = "dummy_connector")]
    #[serde(rename = "phonypay")]
    #[strum(serialize = "phonypay")]
    DummyConnector1,
    #[cfg(feature = "dummy_connector")]
    #[serde(rename = "fauxpay")]
    #[strum(serialize = "fauxpay")]
    DummyConnector2,
    #[cfg(feature = "dummy_connector")]
    #[serde(rename = "pretendpay")]
    #[strum(serialize = "pretendpay")]
    DummyConnector3,
    #[cfg(feature = "dummy_connector")]
    #[serde(rename = "stripe_test")]
    #[strum(serialize = "stripe_test")]
    DummyConnector4,
    #[cfg(feature = "dummy_connector")]
    #[serde(rename = "adyen_test")]
    #[strum(serialize = "adyen_test")]
    DummyConnector5,
    #[cfg(feature = "dummy_connector")]
    #[serde(rename = "checkout_test")]
    #[strum(serialize = "checkout_test")]
    DummyConnector6,
    #[cfg(feature = "dummy_connector")]
    #[serde(rename = "paypal_test")]
    #[strum(serialize = "paypal_test")]
    DummyConnector7,
    Aci,
    Adyen,
    Airwallex,
    Authorizedotnet,
    Bambora,
    Bankofamerica,
    Billwerk,
    Bitpay,
    Bluesnap,
    Boku,
    Braintree,
    Cashtocode,
    Checkout,
    Coinbase,
    Cryptopay,
    Cybersource,
    Dlocal,
    // Ebanx,
    Esnekpos,
    Fiserv,
    Forte,
    Globalpay,
    Globepay,
    Gocardless,
    Helcim,
    Iatapay,
    Klarna,
    Mollie,
    Multisafepay,
    Netcetera,
    Nexinets,
    Nmi,
    Noon,
    Nuvei,
    // Opayo, added as template code for future usage
    Opennode,
    // Payeezy, As psync and rsync are not supported by this connector, it is added as template code for future usage
    Payme,
    Paypal,
    Payu,
    Placetopay,
    Powertranz,
    Prophetpay,
    Rapyd,
    Shift4,
    Square,
    Stax,
    Stripe,
    Threedsecureio,
    Trustpay,
    // Tsys,
    Tsys,
    Volt,
    Wise,
    Worldline,
    Worldpay,
    Signifyd,
    Plaid,
    Riskified,
    Zen,
    Zsl,
}

impl Connector {
    pub fn supports_access_token(&self, payment_method: PaymentMethod) -> bool {
        matches!(
            (self, payment_method),
            (Self::Airwallex, _)
                | (Self::Globalpay, _)
                | (Self::Paypal, _)
                | (Self::Payu, _)
                | (Self::Trustpay, PaymentMethod::BankRedirect)
                | (Self::Iatapay, _)
                | (Self::Volt, _)
        )
    }
    pub fn supports_file_storage_module(&self) -> bool {
        matches!(self, Self::Stripe | Self::Checkout)
    }
    pub fn requires_defend_dispute(&self) -> bool {
        matches!(self, Self::Checkout)
    }
    pub fn is_separate_authentication_supported(&self) -> bool {
        #[cfg(feature = "dummy_connector")]
        match self {
            Self::DummyConnector1
            | Self::DummyConnector2
            | Self::DummyConnector3
            | Self::DummyConnector4
            | Self::DummyConnector5
            | Self::DummyConnector6
            | Self::DummyConnector7 => false,
            Self::Aci
            | Self::Adyen
            | Self::Airwallex
            | Self::Authorizedotnet
            | Self::Bambora
            | Self::Bankofamerica
            | Self::Billwerk
            | Self::Bitpay
            | Self::Bluesnap
            | Self::Boku
            | Self::Braintree
            | Self::Cashtocode
            | Self::Coinbase
            | Self::Cryptopay
            | Self::Dlocal
            | Self::Fiserv
            | Self::Forte
            | Self::Globalpay
            | Self::Globepay
            | Self::Gocardless
            | Self::Helcim
            | Self::Iatapay
            | Self::Klarna
            | Self::Mollie
            | Self::Multisafepay
            | Self::Nexinets
            | Self::Nuvei
            | Self::Opennode
            | Self::Payme
            | Self::Paypal
            | Self::Payu
            | Self::Placetopay
            | Self::Powertranz
            | Self::Prophetpay
            | Self::Rapyd
            | Self::Shift4
            | Self::Square
            | Self::Stax
            | Self::Trustpay
            | Self::Tsys
            | Self::Volt
            | Self::Wise
            | Self::Worldline
            | Self::Worldpay
            | Self::Zen
            | Self::Zsl
            | Self::Signifyd
            | Self::Plaid
            | Self::Riskified
            | Self::Threedsecureio
            | Self::Netcetera
            | Self::Cybersource
            | Self::Noon
            | Self::Esnekpos
            | Self::Stripe => false,
            Self::Checkout | Self::Nmi => true,
            Connector::Esnekpos => todo!(),
            Connector::DummyConnector1 => todo!(),
            Connector::DummyConnector2 => todo!(),
            Connector::DummyConnector3 => todo!(),
            Connector::DummyConnector4 => todo!(),
            Connector::DummyConnector5 => todo!(),
            Connector::DummyConnector6 => todo!(),
            Connector::DummyConnector7 => todo!(),
            Connector::Aci => todo!(),
            Connector::Adyen => todo!(),
            Connector::Airwallex => todo!(),
            Connector::Authorizedotnet => todo!(),
            Connector::Bambora => todo!(),
            Connector::Bankofamerica => todo!(),
            Connector::Billwerk => todo!(),
            Connector::Bitpay => todo!(),
            Connector::Bluesnap => todo!(),
            Connector::Boku => todo!(),
            Connector::Braintree => todo!(),
            Connector::Cashtocode => todo!(),
            Connector::Checkout => todo!(),
            Connector::Coinbase => todo!(),
            Connector::Cryptopay => todo!(),
            Connector::Cybersource => todo!(),
            Connector::Dlocal => todo!(),
            Connector::Fiserv => todo!(),
            Connector::Forte => todo!(),
            Connector::Globalpay => todo!(),
            Connector::Globepay => todo!(),
            Connector::Gocardless => todo!(),
            Connector::Helcim => todo!(),
            Connector::Iatapay => todo!(),
            Connector::Klarna => todo!(),
            Connector::Mollie => todo!(),
            Connector::Multisafepay => todo!(),
            Connector::Netcetera => todo!(),
            Connector::Nexinets => todo!(),
            Connector::Nmi => todo!(),
            Connector::Noon => todo!(),
            Connector::Nuvei => todo!(),
            Connector::Opennode => todo!(),
            Connector::Payme => todo!(),
            Connector::Paypal => todo!(),
            Connector::Payu => todo!(),
            Connector::Placetopay => todo!(),
            Connector::Powertranz => todo!(),
            Connector::Prophetpay => todo!(),
            Connector::Rapyd => todo!(),
            Connector::Shift4 => todo!(),
            Connector::Square => todo!(),
            Connector::Stax => todo!(),
            Connector::Stripe => todo!(),
            Connector::Threedsecureio => todo!(),
            Connector::Trustpay => todo!(),
            Connector::Tsys => todo!(),
            Connector::Volt => todo!(),
            Connector::Wise => todo!(),
            Connector::Worldline => todo!(),
            Connector::Worldpay => todo!(),
            Connector::Signifyd => todo!(),
            Connector::Plaid => todo!(),
            Connector::Riskified => todo!(),
            Connector::Zen => todo!(),
            Connector::Zsl => todo!(),
        }
        #[cfg(not(feature = "dummy_connector"))]
        match self {
            Self::Aci
            | Self::Adyen
            | Self::Airwallex
            | Self::Authorizedotnet
            | Self::Bambora
            | Self::Bankofamerica
            | Self::Billwerk
            | Self::Bitpay
            | Self::Bluesnap
            | Self::Boku
            | Self::Braintree
            | Self::Cashtocode
            | Self::Coinbase
            | Self::Cryptopay
            | Self::Dlocal
            | Self::Fiserv
            | Self::Esnekpos
            | Self::Forte
            | Self::Globalpay
            | Self::Globepay
            | Self::Gocardless
            | Self::Helcim
            | Self::Iatapay
            | Self::Klarna
            | Self::Mollie
            | Self::Multisafepay
            | Self::Nexinets
            | Self::Nmi
            | Self::Nuvei
            | Self::Opennode
            | Self::Payme
            | Self::Paypal
            | Self::Payu
            | Self::Placetopay
            | Self::Powertranz
            | Self::Prophetpay
            | Self::Rapyd
            | Self::Shift4
            | Self::Square
            | Self::Stax
            | Self::Trustpay
            | Self::Tsys
            | Self::Volt
            | Self::Wise
            | Self::Worldline
            | Self::Worldpay
            | Self::Zen
            | Self::Zsl
            | Self::Signifyd
            | Self::Plaid
            | Self::Riskified
            | Self::Threedsecureio
            | Self::Cybersource
            | Self::Noon
            | Self::Netcetera
            | Self::Stripe => false,
            Self::Checkout => true,
        }
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    strum::Display,
    strum::EnumString,
    ToSchema,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AuthenticationConnectors {
    Threedsecureio,
    Netcetera,
}

#[cfg(feature = "payouts")]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    strum::Display,
    strum::EnumString,
    ToSchema,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PayoutConnectors {
    Adyen,
    Wise,
}

#[cfg(feature = "payouts")]
impl From<PayoutConnectors> for RoutableConnectors {
    fn from(value: PayoutConnectors) -> Self {
        match value {
            PayoutConnectors::Adyen => Self::Adyen,
            PayoutConnectors::Wise => Self::Wise,
        }
    }
}

#[cfg(feature = "payouts")]
impl From<PayoutConnectors> for Connector {
    fn from(value: PayoutConnectors) -> Self {
        match value {
            PayoutConnectors::Adyen => Self::Adyen,
            PayoutConnectors::Wise => Self::Wise,
        }
    }
}

#[cfg(feature = "payouts")]
impl TryFrom<Connector> for PayoutConnectors {
    type Error = String;
    fn try_from(value: Connector) -> Result<Self, Self::Error> {
        match value {
            Connector::Adyen => Ok(Self::Adyen),
            Connector::Wise => Ok(Self::Wise),
            _ => Err(format!("Invalid payout connector {}", value)),
        }
    }
}

#[cfg(feature = "frm")]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    strum::Display,
    strum::EnumString,
    ToSchema,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FrmConnectors {
    /// Signifyd Risk Manager. Official docs: https://docs.signifyd.com/
    Signifyd,
    Riskified,
}

#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, strum::Display, strum::EnumString, ToSchema,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum FrmAction {
    CancelTxn,
    AutoRefund,
    ManualReview,
}

#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, strum::Display, strum::EnumString, ToSchema,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum FrmPreferredFlowTypes {
    Pre,
    Post,
}
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnresolvedResponseReason {
    pub code: String,
    /// A message to merchant to give hint on next action he/she should do to resolve
    pub message: String,
}

/// Possible field type of required fields in payment_method_data
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    strum::Display,
    strum::EnumString,
    ToSchema,
    Hash,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FieldType {
    UserCardNumber,
    UserCardExpiryMonth,
    UserCardExpiryYear,
    UserCardCvc,
    UserFullName,
    UserEmailAddress,
    UserPhoneNumber,
    UserCountryCode,                      //phone number's country code
    UserCountry { options: Vec<String> }, //for country inside payment method data ex- bank redirect
    UserCurrency { options: Vec<String> },
    UserBillingName,
    UserAddressLine1,
    UserAddressLine2,
    UserAddressCity,
    UserAddressPincode,
    UserAddressState,
    UserAddressCountry { options: Vec<String> },
    UserBlikCode,
    UserBank,
    Text,
    DropDown { options: Vec<String> },
}

#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    strum::Display,
    strum::EnumString,
    Clone,
    PartialEq,
    Eq,
    ToSchema,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RetryAction {
    /// Payment can be retried from the client side until the payment is successful or payment expires or the attempts(configured by the merchant) for payment are exhausted
    ManualRetry,
    /// Denotes that the payment is requeued
    Requeue,
}

#[derive(Clone, Copy)]
pub enum LockerChoice {
    HyperswitchCardVault,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    strum::Display,
    strum::EnumString,
    frunk::LabelledGeneric,
    ToSchema,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PmAuthConnectors {
    Plaid,
}

pub fn convert_pm_auth_connector(connector_name: &str) -> Option<PmAuthConnectors> {
    PmAuthConnectors::from_str(connector_name).ok()
}

pub fn convert_authentication_connector(connector_name: &str) -> Option<AuthenticationConnectors> {
    AuthenticationConnectors::from_str(connector_name).ok()
}
