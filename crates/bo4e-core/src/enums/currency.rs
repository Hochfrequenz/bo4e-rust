//! Currency code (Waehrungscode) enumeration.

use serde::{Deserialize, Serialize};

/// ISO 4217 currency codes.
///
/// Enumeration of currency codes following ISO 4217 standard.
///
/// German: Waehrungscode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Currency {
    /// UAE Dirham
    #[serde(rename = "AED")]
    Aed,
    /// Afghani
    #[serde(rename = "AFN")]
    Afn,
    /// Lek
    #[serde(rename = "ALL")]
    All,
    /// Armenian Dram
    #[serde(rename = "AMD")]
    Amd,
    /// Netherlands Antillean Guilder
    #[serde(rename = "ANG")]
    Ang,
    /// Kwanza
    #[serde(rename = "AOA")]
    Aoa,
    /// Argentine Peso
    #[serde(rename = "ARS")]
    Ars,
    /// Australian Dollar
    #[serde(rename = "AUD")]
    Aud,
    /// Aruban Florin
    #[serde(rename = "AWG")]
    Awg,
    /// Azerbaijanian Manat
    #[serde(rename = "AZN")]
    Azn,
    /// Convertible Mark
    #[serde(rename = "BAM")]
    Bam,
    /// Barbados Dollar
    #[serde(rename = "BBD")]
    Bbd,
    /// Taka
    #[serde(rename = "BDT")]
    Bdt,
    /// Bulgarian Lev
    #[serde(rename = "BGN")]
    Bgn,
    /// Bahraini Dinar
    #[serde(rename = "BHD")]
    Bhd,
    /// Burundi Franc
    #[serde(rename = "BIF")]
    Bif,
    /// Bermudian Dollar
    #[serde(rename = "BMD")]
    Bmd,
    /// Brunei Dollar
    #[serde(rename = "BND")]
    Bnd,
    /// Boliviano
    #[serde(rename = "BOB")]
    Bob,
    /// Mvdol
    #[serde(rename = "BOV")]
    Bov,
    /// Brazilian Real
    #[serde(rename = "BRL")]
    Brl,
    /// Bahamian Dollar
    #[serde(rename = "BSD")]
    Bsd,
    /// Ngultrum
    #[serde(rename = "BTN")]
    Btn,
    /// Pula
    #[serde(rename = "BWP")]
    Bwp,
    /// Belarusian Ruble
    #[serde(rename = "BYN")]
    Byn,
    /// Belarusian Ruble (old)
    #[serde(rename = "BYR")]
    Byr,
    /// Belize Dollar
    #[serde(rename = "BZD")]
    Bzd,
    /// Canadian Dollar
    #[serde(rename = "CAD")]
    Cad,
    /// Congolese Franc
    #[serde(rename = "CDF")]
    Cdf,
    /// WIR Euro
    #[serde(rename = "CHE")]
    Che,
    /// Swiss Franc
    #[serde(rename = "CHF")]
    Chf,
    /// WIR Franc
    #[serde(rename = "CHW")]
    Chw,
    /// Unidad de Fomento
    #[serde(rename = "CLF")]
    Clf,
    /// Chilean Peso
    #[serde(rename = "CLP")]
    Clp,
    /// Yuan Renminbi
    #[serde(rename = "CNY")]
    Cny,
    /// Colombian Peso
    #[serde(rename = "COP")]
    Cop,
    /// Unidad de Valor Real
    #[serde(rename = "COU")]
    Cou,
    /// Costa Rican Colon
    #[serde(rename = "CRC")]
    Crc,
    /// Peso Convertible
    #[serde(rename = "CUC")]
    Cuc,
    /// Cuban Peso
    #[serde(rename = "CUP")]
    Cup,
    /// Cape Verde Escudo
    #[serde(rename = "CVE")]
    Cve,
    /// Czech Koruna
    #[serde(rename = "CZK")]
    Czk,
    /// Djibouti Franc
    #[serde(rename = "DJF")]
    Djf,
    /// Danish Krone
    #[serde(rename = "DKK")]
    Dkk,
    /// Dominican Peso
    #[serde(rename = "DOP")]
    Dop,
    /// Algerian Dinar
    #[serde(rename = "DZD")]
    Dzd,
    /// Egyptian Pound
    #[serde(rename = "EGP")]
    Egp,
    /// Nakfa
    #[serde(rename = "ERN")]
    Ern,
    /// Ethiopian Birr
    #[serde(rename = "ETB")]
    Etb,
    /// Euro
    #[serde(rename = "EUR")]
    Eur,
    /// Fiji Dollar
    #[serde(rename = "FJD")]
    Fjd,
    /// Falkland Islands Pound
    #[serde(rename = "FKP")]
    Fkp,
    /// Pound Sterling
    #[serde(rename = "GBP")]
    Gbp,
    /// Lari
    #[serde(rename = "GEL")]
    Gel,
    /// Ghana Cedi
    #[serde(rename = "GHS")]
    Ghs,
    /// Gibraltar Pound
    #[serde(rename = "GIP")]
    Gip,
    /// Dalasi
    #[serde(rename = "GMD")]
    Gmd,
    /// Guinea Franc
    #[serde(rename = "GNF")]
    Gnf,
    /// Quetzal
    #[serde(rename = "GTQ")]
    Gtq,
    /// Guyana Dollar
    #[serde(rename = "GYD")]
    Gyd,
    /// Hong Kong Dollar
    #[serde(rename = "HKD")]
    Hkd,
    /// Lempira
    #[serde(rename = "HNL")]
    Hnl,
    /// Croatian Kuna
    #[serde(rename = "HRK")]
    Hrk,
    /// Gourde
    #[serde(rename = "HTG")]
    Htg,
    /// Forint
    #[serde(rename = "HUF")]
    Huf,
    /// Rupiah
    #[serde(rename = "IDR")]
    Idr,
    /// New Israeli Sheqel
    #[serde(rename = "ILS")]
    Ils,
    /// Indian Rupee
    #[serde(rename = "INR")]
    Inr,
    /// Iraqi Dinar
    #[serde(rename = "IQD")]
    Iqd,
    /// Iranian Rial
    #[serde(rename = "IRR")]
    Irr,
    /// Iceland Krona
    #[serde(rename = "ISK")]
    Isk,
    /// Jamaican Dollar
    #[serde(rename = "JMD")]
    Jmd,
    /// Jordanian Dinar
    #[serde(rename = "JOD")]
    Jod,
    /// Yen
    #[serde(rename = "JPY")]
    Jpy,
    /// Kenyan Shilling
    #[serde(rename = "KES")]
    Kes,
    /// Som
    #[serde(rename = "KGS")]
    Kgs,
    /// Riel
    #[serde(rename = "KHR")]
    Khr,
    /// Comoro Franc
    #[serde(rename = "KMF")]
    Kmf,
    /// North Korean Won
    #[serde(rename = "KPW")]
    Kpw,
    /// Won
    #[serde(rename = "KRW")]
    Krw,
    /// Kuwaiti Dinar
    #[serde(rename = "KWD")]
    Kwd,
    /// Cayman Islands Dollar
    #[serde(rename = "KYD")]
    Kyd,
    /// Tenge
    #[serde(rename = "KZT")]
    Kzt,
    /// Kip
    #[serde(rename = "LAK")]
    Lak,
    /// Lebanese Pound
    #[serde(rename = "LBP")]
    Lbp,
    /// Sri Lanka Rupee
    #[serde(rename = "LKR")]
    Lkr,
    /// Liberian Dollar
    #[serde(rename = "LRD")]
    Lrd,
    /// Loti
    #[serde(rename = "LSL")]
    Lsl,
    /// Lithuanian Litas
    #[serde(rename = "LTL")]
    Ltl,
    /// Libyan Dinar
    #[serde(rename = "LYD")]
    Lyd,
    /// Moroccan Dirham
    #[serde(rename = "MAD")]
    Mad,
    /// Moldovan Leu
    #[serde(rename = "MDL")]
    Mdl,
    /// Malagasy Ariary
    #[serde(rename = "MGA")]
    Mga,
    /// Denar
    #[serde(rename = "MKD")]
    Mkd,
    /// Kyat
    #[serde(rename = "MMK")]
    Mmk,
    /// Tugrik
    #[serde(rename = "MNT")]
    Mnt,
    /// Pataca
    #[serde(rename = "MOP")]
    Mop,
    /// Ouguiya
    #[serde(rename = "MRO")]
    Mro,
    /// Mauritius Rupee
    #[serde(rename = "MUR")]
    Mur,
    /// Rufiyaa
    #[serde(rename = "MVR")]
    Mvr,
    /// Kwacha
    #[serde(rename = "MWK")]
    Mwk,
    /// Mexican Peso
    #[serde(rename = "MXN")]
    Mxn,
    /// Mexican Unidad de Inversion (UDI)
    #[serde(rename = "MXV")]
    Mxv,
    /// Malaysian Ringgit
    #[serde(rename = "MYR")]
    Myr,
    /// Mozambique Metical
    #[serde(rename = "MZN")]
    Mzn,
    /// Namibia Dollar
    #[serde(rename = "NAD")]
    Nad,
    /// Naira
    #[serde(rename = "NGN")]
    Ngn,
    /// Cordoba Oro
    #[serde(rename = "NIO")]
    Nio,
    /// Norwegian Krone
    #[serde(rename = "NOK")]
    Nok,
    /// Nepalese Rupee
    #[serde(rename = "NPR")]
    Npr,
    /// New Zealand Dollar
    #[serde(rename = "NZD")]
    Nzd,
    /// Rial Omani
    #[serde(rename = "OMR")]
    Omr,
    /// Balboa
    #[serde(rename = "PAB")]
    Pab,
    /// Nuevo Sol
    #[serde(rename = "PEN")]
    Pen,
    /// Kina
    #[serde(rename = "PGK")]
    Pgk,
    /// Philippine Peso
    #[serde(rename = "PHP")]
    Php,
    /// Pakistan Rupee
    #[serde(rename = "PKR")]
    Pkr,
    /// Zloty
    #[serde(rename = "PLN")]
    Pln,
    /// Guarani
    #[serde(rename = "PYG")]
    Pyg,
    /// Qatari Rial
    #[serde(rename = "QAR")]
    Qar,
    /// New Romanian Leu
    #[serde(rename = "RON")]
    Ron,
    /// Serbian Dinar
    #[serde(rename = "RSD")]
    Rsd,
    /// Russian Ruble
    #[serde(rename = "RUB")]
    Rub,
    /// Russian Ruble (old)
    #[serde(rename = "RUR")]
    Rur,
    /// Rwanda Franc
    #[serde(rename = "RWF")]
    Rwf,
    /// Saudi Riyal
    #[serde(rename = "SAR")]
    Sar,
    /// Solomon Islands Dollar
    #[serde(rename = "SBD")]
    Sbd,
    /// Seychelles Rupee
    #[serde(rename = "SCR")]
    Scr,
    /// Sudanese Pound
    #[serde(rename = "SDG")]
    Sdg,
    /// Swedish Krona
    #[serde(rename = "SEK")]
    Sek,
    /// Singapore Dollar
    #[serde(rename = "SGD")]
    Sgd,
    /// Saint Helena Pound
    #[serde(rename = "SHP")]
    Shp,
    /// Leone
    #[serde(rename = "SLL")]
    Sll,
    /// Somali Shilling
    #[serde(rename = "SOS")]
    Sos,
    /// Surinam Dollar
    #[serde(rename = "SRD")]
    Srd,
    /// South Sudanese Pound
    #[serde(rename = "SSP")]
    Ssp,
    /// Dobra
    #[serde(rename = "STD")]
    Std,
    /// El Salvador Colon
    #[serde(rename = "SVC")]
    Svc,
    /// Syrian Pound
    #[serde(rename = "SYP")]
    Syp,
    /// Lilangeni
    #[serde(rename = "SZL")]
    Szl,
    /// Baht
    #[serde(rename = "THB")]
    Thb,
    /// Somoni
    #[serde(rename = "TJS")]
    Tjs,
    /// Turkmenistan New Manat
    #[serde(rename = "TMT")]
    Tmt,
    /// Tunisian Dinar
    #[serde(rename = "TND")]
    Tnd,
    /// Pa'anga
    #[serde(rename = "TOP")]
    Top,
    /// Turkish Lira
    #[serde(rename = "TRY")]
    Try,
    /// Trinidad and Tobago Dollar
    #[serde(rename = "TTD")]
    Ttd,
    /// New Taiwan Dollar
    #[serde(rename = "TWD")]
    Twd,
    /// Tanzanian Shilling
    #[serde(rename = "TZS")]
    Tzs,
    /// Hryvnia
    #[serde(rename = "UAH")]
    Uah,
    /// Uganda Shilling
    #[serde(rename = "UGX")]
    Ugx,
    /// US Dollar
    #[serde(rename = "USD")]
    Usd,
    /// US Dollar (Next day)
    #[serde(rename = "USN")]
    Usn,
    /// US Dollar (Same day)
    #[serde(rename = "USS")]
    Uss,
    /// Uruguay Peso en Unidades Indexadas
    #[serde(rename = "UYI")]
    Uyi,
    /// Peso Uruguayo
    #[serde(rename = "UYU")]
    Uyu,
    /// Uzbekistan Sum
    #[serde(rename = "UZS")]
    Uzs,
    /// Bolivar
    #[serde(rename = "VEF")]
    Vef,
    /// Dong
    #[serde(rename = "VND")]
    Vnd,
    /// Vatu
    #[serde(rename = "VUV")]
    Vuv,
    /// Tala
    #[serde(rename = "WST")]
    Wst,
    /// CFA Franc BEAC
    #[serde(rename = "XAF")]
    Xaf,
    /// Silver
    #[serde(rename = "XAG")]
    Xag,
    /// Gold
    #[serde(rename = "XAU")]
    Xau,
    /// Bond Markets Unit European Composite Unit (EURCO)
    #[serde(rename = "XBA")]
    Xba,
    /// Bond Markets Unit European Monetary Unit (E.M.U.-6)
    #[serde(rename = "XBB")]
    Xbb,
    /// Bond Markets Unit European Unit of Account 9 (E.U.A.-9)
    #[serde(rename = "XBC")]
    Xbc,
    /// Bond Markets Unit European Unit of Account 17 (E.U.A.-17)
    #[serde(rename = "XBD")]
    Xbd,
    /// East Caribbean Dollar
    #[serde(rename = "XCD")]
    Xcd,
    /// SDR (Special Drawing Right)
    #[serde(rename = "XDR")]
    Xdr,
    /// CFA Franc BCEAO
    #[serde(rename = "XOF")]
    Xof,
    /// Palladium
    #[serde(rename = "XPD")]
    Xpd,
    /// CFP Franc
    #[serde(rename = "XPF")]
    Xpf,
    /// Platinum
    #[serde(rename = "XPT")]
    Xpt,
    /// Sucre
    #[serde(rename = "XSU")]
    Xsu,
    /// Codes specifically reserved for testing purposes
    #[serde(rename = "XTS")]
    Xts,
    /// ADB Unit of Account
    #[serde(rename = "XUA")]
    Xua,
    /// No currency
    #[serde(rename = "XXX")]
    Xxx,
    /// Yemeni Rial
    #[serde(rename = "YER")]
    Yer,
    /// Rand
    #[serde(rename = "ZAR")]
    Zar,
    /// Zambian Kwacha
    #[serde(rename = "ZMW")]
    Zmw,
    /// Zimbabwe Dollar
    #[serde(rename = "ZWL")]
    Zwl,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(serde_json::to_string(&Currency::Eur).unwrap(), r#""EUR""#);
        assert_eq!(serde_json::to_string(&Currency::Usd).unwrap(), r#""USD""#);
    }

    #[test]
    fn test_roundtrip() {
        for curr in [
            Currency::Eur,
            Currency::Usd,
            Currency::Gbp,
            Currency::Chf,
            Currency::Jpy,
        ] {
            let json = serde_json::to_string(&curr).unwrap();
            let parsed: Currency = serde_json::from_str(&json).unwrap();
            assert_eq!(curr, parsed);
        }
    }
}
